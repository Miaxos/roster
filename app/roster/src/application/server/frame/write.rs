use std::io;
use std::io::Cursor;

use monoio::buf::{IoBuf, IoVecBuf};
use monoio::io::AsyncWriteRent;
use monoio::BufResult;

use crate::application::server::frame::Frame;

/// Write a decimal value
pub async fn write_decimal(
    buf_w: &mut impl AsyncWriteRent,
    val: u64,
) -> io::Result<()> {
    use std::io::Write;

    // Convert the value to a string
    let buf = vec![0u8; 20];
    let mut buf = Cursor::new(buf);
    write!(&mut buf, "{}", val)?;

    let pos = buf.position() as usize;

    buf_w.write(buf.into_inner().slice(..pos)).await.0?;
    buf_w.write(b"\r\n").await.0?;

    Ok(())
}

/// Write a value
#[async_recursion::async_recursion(?Send)]
pub async fn write_value(
    buf_w: &mut impl AsyncWriteRent,
    frame: &Frame,
) -> io::Result<()> {
    match frame {
        Frame::Simple(val) => {
            buf_w.write(&[b'+']).await.0?;
            buf_w.write(val.as_bytes().slice(..)).await.0?;
            buf_w.write(&[b'\r', b'\n']).await.0?;
        }
        Frame::Error(val) => {
            buf_w.write(&[b'-']).await.0?;
            buf_w.write(val.as_bytes().slice(..)).await.0?;
            buf_w.write(&[b'\r', b'\n']).await.0?;
        }
        Frame::Integer(val) => {
            buf_w.write(&[b':']).await.0?;
            write_decimal(buf_w, *val).await?;
        }
        Frame::Null => {
            buf_w.write(b"$-1\r\n").await.0?;
        }
        Frame::Bulk(val) => {
            let len = val.len();

            buf_w.write([b'$'].as_slice()).await.0?;
            write_decimal(buf_w, len as u64).await?;
            buf_w.write(val.slice(..)).await.0?;
            buf_w.write(&[b'\r', b'\n']).await.0?;
        }
        Frame::HashMap(val) => {
            let len = val.len();

            buf_w.write([b'%'].as_slice()).await.0?;
            write_decimal(buf_w, len as u64).await?;
            for (key, value) in val {
                write_value(buf_w, key).await?;
                write_value(buf_w, value).await?;
            }
            buf_w.write(&[b'\r', b'\n']).await.0?;
        }
        // Encoding an `Array` from within a value cannot be done using a
        // recursive strategy. In general, async fns do not support
        // recursion. Mini-redis has not needed to encode nested arrays yet,
        // so for now it is skipped.
        Frame::Array(_val) => unreachable!(),
    }

    Ok(())
}

pub async fn write_frame(
    buf_w: &mut impl AsyncWriteRent,
    frame: &Frame,
) -> io::Result<()> {
    // Arrays are encoded by encoding each entry. All other frame types are
    // considered literals. For now, mini-redis is not able to encode
    // recursive frame structures. See below for more details.
    match frame {
        Frame::Array(val) => {
            // Encode the length of the array.
            buf_w.write(&[b'*']).await.0?;
            write_decimal(buf_w, val.len() as u64).await?;

            // Iterate and encode each entry in the array.
            for entry in &**val {
                write_value(buf_w, entry).await?;
            }
        }
        // The frame type is a literal. Encode the value directly.
        _ => write_value(buf_w, frame).await?,
    }

    // Ensure the encoded frame is written to the socket. The calls above
    // are to the buffered stream and writes. Calling `flush` writes the
    // remaining contents of the buffer to the socket.
    buf_w.flush().await
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use monoio::buf::{IoBuf, IoVecBuf};
    use monoio::io::AsyncWriteRent;
    use monoio::BufResult;

    use super::write_decimal;

    struct TestUtilVec<W>(pub Vec<W>);

    impl AsyncWriteRent for TestUtilVec<u8> {
        async fn write<T: IoBuf>(&mut self, buf: T) -> BufResult<usize, T> {
            let ptr = buf.read_ptr();
            let len = buf.bytes_init();
            let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
            let a = self.0.write(slice);
            (a, buf)
        }

        async fn flush(&mut self) -> std::io::Result<()> {
            todo!();
        }

        async fn writev<T: IoVecBuf>(
            &mut self,
            _buf_vec: T,
        ) -> BufResult<usize, T> {
            todo!()
        }

        async fn shutdown(&mut self) -> std::io::Result<()> {
            todo!()
        }
    }

    #[monoio::test]
    async fn simple_decimal_write() {
        let mut v = TestUtilVec(Vec::new());
        write_decimal(&mut v, 12).await.unwrap();
        assert_eq!(v.0, b"12\r\n");
    }
}
