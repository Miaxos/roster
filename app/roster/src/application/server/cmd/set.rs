use std::time::Duration;

use bytes::Bytes;
use bytestring::ByteString;

use super::parse::{Parse, ParseError};
use super::CommandExecution;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;
use crate::domain::storage::SetOptions;

/// Set key to hold the string value. If key already holds a value, it is
/// overwritten, regardless of its type.
///
/// Any previous time to live associated with the key is discarded on successful
/// SET operation.

/// # Options
/// The SET command supports a set of options that modify its behavior:
///
/// - EX seconds -- Set the specified expire time, in seconds (a positive
/// integer).
/// - PX milliseconds -- Set the specified expire time, in milliseconds
/// (a positive integer).
/// - EXAT timestamp-seconds -- Set the specified Unix time
/// at which the key will expire, in seconds (a positive integer).
/// - PXAT timestamp-milliseconds -- Set the specified Unix time at which the
///   key
/// will expire, in milliseconds (a positive integer).
/// - NX -- Only set the key if
/// it does not already exist. XX -- Only set the key if it already exists.
/// - KEEPTTL -- Retain the time to live associated with the key.
/// - GET -- Return the old string stored at key, or nil if key did not exist.
///
/// An error is returned and SET aborted if the value stored at key is not a
/// string.
///
/// **Note**: Since the SET command options can replace SETNX, SETEX,
/// PSETEX, GETSET, it is possible that in future versions of Redis these
/// commands will be deprecated and finally removed.
#[derive(Debug, Default)]
pub struct Set {
    /// the lookup key
    key: ByteString,

    /// the value to be stored
    value: Bytes,

    /// When to expire the key
    expire: Option<Duration>,
}

static OK_STR: ByteString = ByteString::from_static("OK");

impl Set {
    /// Parse a `Set` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `SET` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `Set` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing at least 3 entries.
    ///
    /// ```text
    /// SET key value [EX seconds|PX milliseconds]
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parse) -> anyhow::Result<Set> {
        use ParseError::EndOfStream;

        // Read the key to set. This is a required field
        let key = parse.next_string()?;

        // Read the value to set. This is a required field.
        let value = parse.next_bytes()?;

        // The expiration is optional. If nothing else follows, then it is
        // `None`.
        let mut expire = None;

        // Attempt to parse another string.
        match parse.next_string() {
            Ok(s) if s.to_uppercase() == "EX" => {
                // An expiration is specified in seconds. The next value is an
                // integer.
                let secs = parse.next_int()?;
                expire = Some(Duration::from_secs(secs));
            }
            Ok(s) if s.to_uppercase() == "PX" => {
                // An expiration is specified in milliseconds. The next value is
                // an integer.
                let ms = parse.next_int()?;
                expire = Some(Duration::from_millis(ms));
            }
            // Currently, roster does not support any of the other SET
            // options. An error here results in the connection being
            // terminated.
            Ok(_) => {
                return Err(anyhow::anyhow!(
                    "currently `SET` only supports the expiration option"
                ));
            }
            Err(EndOfStream) => {}
            Err(err) => return Err(err.into()),
        }

        Ok(Set { key, value, expire })
    }
}

impl CommandExecution for Set {
    async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        let expired = self.expire.map(|dur| ctx.now() + dur.into());

        // let now = Instant::now();
        let response = match ctx
            .storage
            .set_async(self.key, self.value, SetOptions { expired })
            .await
        {
            Ok(_) => Frame::Simple(OK_STR.clone()),
            Err(_) => Frame::Null,
        };
        // let elapsed = now.elapsed();
        // dbg!(elapsed);

        // let response = Frame::Null;

        // Write the response back to the client
        dst.write_frame(&response).await?;

        Ok(())
    }
}
