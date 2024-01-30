use std::time::Duration;

use bytes::Bytes;
use bytestring::ByteString;

use super::super::parse::{Parse, ParseError};
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// The CLIENT SETNAME command assigns a name to the current connection.
///
/// The assigned name is displayed in the output of CLIENT LIST so that it is
/// possible to identify the client that performed a given connection.
///
/// For instance when Redis is used in order to implement a queue, producers and
/// consumers of messages may want to set the name of the connection according
/// to their role.
///
/// There is no limit to the length of the name that can be assigned if not the
/// usual limits of the Redis string type (512 MB). However it is not possible
/// to use spaces in the connection name as this would violate the format of the
/// CLIENT LIST reply.
///
/// It is possible to entirely remove the connection name setting it to the
/// empty string, that is not a valid connection name since it serves to this
/// specific purpose.
///
/// The connection name can be inspected using CLIENT GETNAME.
///
/// Every new connection starts without an assigned name.
///
/// Tip: setting names to connections is a good way to debug connection leaks
/// due to bugs in the application using Redis.
#[derive(Debug, Default)]
pub struct ClientSetName {
    name: ByteString,
}

impl ClientSetName {
    /// Create a new `ClientSetName` command with optional `msg`.
    pub fn new(name: ByteString) -> ClientSetName {
        ClientSetName { name }
    }

    pub(crate) fn parse_frames(
        parse: &mut Parse,
    ) -> anyhow::Result<ClientSetName> {
        let name = parse.next_string()?;
        Ok(ClientSetName::new(name))
    }

    pub(crate) async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        ctx.connection.set_name(self.name).await;

        let response = Frame::Simple(ByteString::from_static("OK"));
        dst.write_frame(&response).await?;

        Ok(())
    }
}
