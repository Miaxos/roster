use bytes::Bytes;
use bytestring::ByteString;

use super::parse::{Parse, ParseError};
use super::CommandExecution;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// Returns PONG if no argument is provided, otherwise
/// return a copy of the argument as a bulk.
///
/// This command is often used to test if a connection
/// is still alive, or to measure latency.
#[derive(Debug, Default)]
pub struct Ping {
    /// optional message to be returned
    msg: Option<Bytes>,
}

impl Ping {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(msg: Option<Bytes>) -> Ping {
        Ping { msg }
    }

    /// Parse a `Ping` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `PING` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `Ping` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing `PING` and an optional message.
    ///
    /// ```text
    /// PING [message]
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parse) -> anyhow::Result<Ping> {
        match parse.next_bytes() {
            Ok(msg) => Ok(Ping::new(Some(msg))),
            Err(ParseError::EndOfStream) => Ok(Ping::default()),
            Err(e) => Err(e.into()),
        }
    }
}

impl CommandExecution for Ping {
    async fn apply(
        self,
        dst: &mut WriteConnection,
        _ctx: Context,
    ) -> anyhow::Result<()> {
        let response = match self.msg {
            None => Frame::Simple(ByteString::from_static("PONG")),
            Some(msg) => Frame::Bulk(msg),
        };

        // Write the response back to the client
        dst.write_frame(&response).await?;

        Ok(())
    }
}
