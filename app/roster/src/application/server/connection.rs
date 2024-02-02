use std::fmt::Debug;
use std::io::{self, Cursor};

use bytes::BytesMut;
use monoio::io::{
    AsyncReadRent, BufReader, BufWriter, OwnedReadHalf, OwnedWriteHalf,
    Splitable,
};
use monoio::net::TcpStream;

use super::frame::write::write_frame;
use super::frame::Frame;

/// Send and receive `Frame` values from a remote peer.
///
/// When implementing networking protocols, a message on that protocol is
/// often composed of several smaller messages known as frames. The purpose of
/// `Connection` is to read and write frames on the underlying `TcpStream`.
///
/// To read frames, the `Connection` uses an internal buffer, which is filled
/// up until there are enough bytes to create a full frame. Once this happens,
/// the `Connection` creates the frame and returns it to the caller.
///
/// When sending frames, the frame is first encoded into the write buffer.
/// The contents of the write buffer are then written to the socket.
pub struct WriteConnection {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides
    // write level buffering.
    stream_w: BufWriter<OwnedWriteHalf<TcpStream>>,
}

pub struct ReadConnection {
    pub stream_r: BufReader<OwnedReadHalf<TcpStream>>,
    buffer: BytesMut,
}

impl Debug for ReadConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReadConnection")
    }
}

impl ReadConnection {
    /// Read a single `Frame` value from the underlying stream.
    ///
    /// The function waits until it has retrieved enough data to parse a frame.
    /// Any data remaining in the read buffer after the frame has been parsed is
    /// kept there for the next call to `read_frame`.
    ///
    /// # Returns
    ///
    /// On success, the received frame is returned. If the `TcpStream`
    /// is closed in a way that doesn't break a frame in half, it returns
    /// `None`. Otherwise, an error is returned.
    pub async fn read_frame(&mut self) -> anyhow::Result<Option<Frame>> {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data
            // has been buffered, the frame is returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            let mut in_going = BytesMut::new();
            std::mem::swap(&mut self.buffer, &mut in_going);

            // TODO: Timeout
            let (size, buf) = self.stream_r.read(in_going).await;
            self.buffer = buf;

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            if 0 == size? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(anyhow::anyhow!("connection reset by peer"));
                }
            }
        }
    }

    fn parse_frame(&mut self) -> anyhow::Result<Option<Frame>> {
        use super::frame::Error::Incomplete;

        let mut buf = Cursor::new(&self.buffer);

        // TODO: Change this because we do a lot of useless copy
        // We should to the check but prepare for a zero-copy deserialization
        // into a frame.
        match Frame::check(&mut buf) {
            Ok(_) => {
                // The `check` function will have advanced the cursor until the
                // end of the frame. Since the cursor had position set to zero
                // before `Frame::check` was called, we obtain the length of the
                // frame by checking the cursor position.
                let len = buf.position() as usize;

                let mut buf = Cursor::new(self.buffer.split_to(len).freeze());
                // Reset the position to zero before passing the cursor to
                // `Frame::parse`.
                buf.set_position(0);

                // Parse the frame from the buffer. This allocates the necessary
                // structures to represent the frame and returns the frame
                // value.
                //
                // If the encoded frame representation is invalid, an error is
                // returned. This should terminate the **current** connection
                // but should not impact any other connected client.
                let frame = Frame::parse(&mut buf)?;

                // Discard the parsed data from the read buffer.
                //
                // When `advance` is called on the read buffer, all of the data
                // up to `len` is discarded. The details of how this works is
                // left to `BytesMut`. This is often done by moving an internal
                // cursor, but it may be done by reallocating and copying data.
                // self.buffer.advance(len);
                if self.buffer.len() < 1024 {
                    self.buffer.reserve(4 * 1024);
                }

                // Return the parsed frame to the caller.
                Ok(Some(frame))
            }
            // There is not enough data present in the read buffer to parse a
            // single frame. We must wait for more data to be received from the
            // socket. Reading from the socket will be done in the statement
            // after this `match`.
            //
            // We do not want to return `Err` from here as this "error" is an
            // expected runtime condition.
            Err(Incomplete) => Ok(None),
            // An error was encountered while parsing the frame. The connection
            // is now in an invalid state. Returning `Err` from here will result
            // in the connection being closed.
            Err(e) => Err(e.into()),
        }
    }

    pub fn into_inner(self) -> OwnedReadHalf<TcpStream> {
        self.stream_r.into_inner()
    }
}

impl WriteConnection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(
        socket: TcpStream,
        buf_size: usize,
    ) -> (WriteConnection, ReadConnection) {
        let (read, write) = socket.into_split();

        (
            WriteConnection {
                stream_w: BufWriter::new(write),
            },
            ReadConnection {
                stream_r: BufReader::new(read),
                buffer: BytesMut::with_capacity(buf_size),
            },
        )
    }

    /// Write a single `Frame` value to the underlying stream.
    ///
    /// The `Frame` value is written to the socket using the various `write_*`
    /// functions provided by `AsyncWrite`. Calling these functions directly on
    /// a `TcpStream` is **not** advised, as this will result in a large number
    /// of syscalls. However, it is fine to call these functions on a
    /// *buffered* write stream. The data will be written to the buffer.
    /// Once the buffer is full, it is flushed to the underlying socket.
    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        write_frame(&mut self.stream_w, frame).await
    }

    pub fn into_inner(self) -> OwnedWriteHalf<TcpStream> {
        self.stream_w.into_inner()
    }

    pub fn reunite(self, read: ReadConnection) -> TcpStream {
        self.into_inner().reunite(read.into_inner()).unwrap()
    }
}
