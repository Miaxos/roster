use super::super::parse::Parse;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// The command returns information and statistics about the current client
/// connection in a mostly human readable format.
///
/// The reply format is identical to that of CLIENT LIST, and the content
/// consists only of information about the current client.
#[derive(Debug, Default)]
pub struct ClientInfo {}

impl ClientInfo {
    pub fn new() -> ClientInfo {
        ClientInfo {}
    }

    pub(crate) fn parse_frames(
        parse: &mut Parse,
    ) -> anyhow::Result<ClientInfo> {
        parse.finish()?;

        Ok(ClientInfo::new())
    }

    pub(crate) async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        let response = Frame::Simple(ctx.connection.format_conn().await);
        dst.write_frame(&response).await?;
        Ok(())
    }
}
