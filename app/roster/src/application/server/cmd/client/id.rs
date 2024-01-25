use super::super::parse::Parse;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// The command just returns the ID of the current connection. Every connection
/// ID has certain guarantees:
///
/// 1. It is never repeated, so if CLIENT ID returns the same number, the caller
///    can be sure that the underlying client did not disconnect and reconnect
///    the connection, but it is still the same connection.
///
/// 2. The ID is monotonically incremental. If the ID of a connection is greater
///    than the ID of another connection, it is guaranteed that the second
///    connection was established with the server at a later time.
///
/// This command is especially useful together with CLIENT UNBLOCK which was
/// introduced also in Redis 5 together with CLIENT ID. Check the CLIENT UNBLOCK
/// command page for a pattern involving the two commands.
#[derive(Debug, Default)]
pub struct ClientID {}

impl ClientID {
    /// Create a new `ClientID`
    pub fn new() -> ClientID {
        ClientID {}
    }

    pub(crate) fn parse_frames(_parse: &mut Parse) -> anyhow::Result<ClientID> {
        Ok(ClientID::new())
    }

    pub(crate) async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        let id = ctx.connection.id();

        let response = Frame::Integer(id);
        dst.write_frame(&response).await?;

        Ok(())
    }
}
