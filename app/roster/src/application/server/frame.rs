//! Provides a type representing a Redis protocol frame as well as utilities for
//! parsing frames from a byte array.

use std::convert::TryInto;
use std::io::Cursor;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use bytes::{Buf, Bytes, BytesMut};
use bytestring::ByteString;

/// A frame in the Redis protocol.
#[derive(Clone, Debug)]
pub enum Frame {
    Simple(ByteString),
    Error(ByteString),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    #[error("ended early")]
    Incomplete,

    /// Invalid message encoding
    #[error("Invalid message encoding: {0}")]
    Other(#[from] anyhow::Error),
}

impl Frame {
    /// Checks if an entire message can be decoded from `src`
    pub fn check(src: &mut Cursor<&BytesMut>) -> Result<(), Error> {
        match get_u8_mut(src)? {
            b'+' => {
                get_line_mut_no_return(src)?;
                Ok(())
            }
            b'-' => {
                get_line_mut_no_return(src)?;
                Ok(())
            }
            b':' => {
                let _ = get_decimal_mut(src)?;
                Ok(())
            }
            b'$' => {
                if b'-' == peek_u8_mut(src)? {
                    // Skip '-1\r\n'
                    skip_mut(src, 4)
                } else {
                    // Read the bulk string
                    let len: usize = get_decimal_mut(src)?.try_into()?;

                    // skip that number of bytes + 2 (\r\n).
                    skip_mut(src, len + 2)
                }
            }
            b'*' => {
                let len = get_decimal_mut(src)?;

                for _ in 0..len {
                    Frame::check(src)?;
                }

                Ok(())
            }
            actual => Err(format!(
                "protocol error; invalid frame type byte `{}`",
                actual
            )
            .into()),
        }
    }

    /// The message has already been validated with `check`.
    pub fn parse(src: &mut Cursor<Bytes>) -> Result<Frame, Error> {
        match get_u8(src)? {
            b'+' => {
                // Read the line and convert it to `Vec<u8>`
                let line = get_line(src)?;
                let string = ByteString::try_from(line).unwrap();

                Ok(Frame::Simple(string))
            }
            b'-' => {
                // Read the line and convert it to `Vec<u8>`
                let line = get_line(src)?;
                let string = ByteString::try_from(line).unwrap();

                Ok(Frame::Error(string))
            }
            b':' => {
                let len = get_decimal(src)?;
                Ok(Frame::Integer(len))
            }
            b'$' => {
                if b'-' == peek_u8(src)? {
                    let line = get_line(src)?;

                    if line != b"-1".as_slice() {
                        return Err(
                            "protocol error; invalid frame format".into()
                        );
                    }

                    Ok(Frame::Null)
                } else {
                    // Read the bulk string
                    let len: usize = get_decimal(src)?.try_into()?;
                    let n = len + 2;

                    if src.remaining() < n {
                        return Err(Error::Incomplete);
                    }

                    let pos = src.position() as usize;
                    let len = pos + len;
                    let data = src.get_ref().slice(pos..len);

                    // skip that number of bytes + 2 (\r\n).
                    skip(src, n)?;

                    Ok(Frame::Bulk(data))
                }
            }
            b'*' => {
                let len = get_decimal(src)?.try_into()?;
                let mut out = Vec::with_capacity(len);

                for _ in 0..len {
                    out.push(Frame::parse(src)?);
                }

                Ok(Frame::Array(out))
            }
            _ => unimplemented!(),
        }
    }
}

fn peek_u8(src: &mut Cursor<Bytes>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.chunk()[0])
}

fn peek_u8_mut(src: &mut Cursor<&BytesMut>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.chunk()[0])
}

fn get_u8(src: &mut Cursor<Bytes>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u8())
}

fn get_u8_mut(src: &mut Cursor<&BytesMut>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u8())
}

fn skip(src: &mut Cursor<Bytes>, n: usize) -> Result<(), Error> {
    if src.remaining() < n {
        return Err(Error::Incomplete);
    }

    src.advance(n);
    Ok(())
}

fn skip_mut(src: &mut Cursor<&BytesMut>, n: usize) -> Result<(), Error> {
    if src.remaining() < n {
        return Err(Error::Incomplete);
    }

    src.advance(n);
    Ok(())
}

/// Read a new-line terminated decimal
#[inline]
fn get_decimal(src: &mut Cursor<Bytes>) -> Result<u64, Error> {
    use atoi_simd::parse;

    let line = get_line(src)?;

    parse::<u64>(&line)
        .map_err(|_| "protocol error; invalid frame format".into())
}

/// Read a new-line terminated decimal
#[inline]
fn get_decimal_mut(src: &mut Cursor<&BytesMut>) -> Result<u64, Error> {
    use atoi_simd::parse;

    let range = get_line_mut(src)?;

    let line = &src.get_ref().as_ref()[range];

    parse::<u64>(line)
        .map_err(|_| "protocol error; invalid frame format".into())
}

/// Find a line
#[inline]
fn get_line(src: &mut Cursor<Bytes>) -> Result<Bytes, Error> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);

            // Return the line
            return Ok(src.get_ref().slice(start..i));
        }
    }

    Err(Error::Incomplete)
}

#[inline]
fn get_line_mut_no_return(src: &mut Cursor<&BytesMut>) -> Result<(), Error> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);
            return Ok(());
        }
    }

    Err(Error::Incomplete)
}

#[inline]
fn get_line_mut(
    src: &mut Cursor<&BytesMut>,
) -> Result<std::ops::Range<usize>, Error> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);

            return Ok(start..i);
        }
    }

    Err(Error::Incomplete)
}

impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::Other(anyhow::anyhow!(src))
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        src.to_string().into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid frame format".into()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::BytesMut;

    use super::Frame;

    #[test]
    fn test_simple_frame() {
        let test_case: Vec<&[u8]> = vec![
            b"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n",
            b"*2\r\n$4\r\nPING\r\n$5\r\nhello\r\n",
        ];

        for t in test_case {
            let b = BytesMut::from(t);
            let mut cur = Cursor::new(&b);
            assert!(Frame::check(&mut cur).is_ok());
        }
    }

    #[test]
    fn test_map_frame() {
        let test_case: Vec<&[u8]> =
            vec![b"%2\r\n+first\r\n:1\r\n+second\r\n:2\r\n"];

        for t in test_case {
            let b = BytesMut::from(t);
            let mut cur = Cursor::new(&b);
            assert!(Frame::check(&mut cur).is_ok());
        }
    }
}
