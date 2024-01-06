use bytestring::ByteString;
use tracing::info;

use super::parse::Parse;
use super::CommandExecution;
use crate::application::server::connection::Connection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// Get the value of key. If the key does not exist the special value nil is
/// returned. An error is returned if the value stored at key is not a string,
/// because GET only handles string values.
#[derive(Debug, Default)]
pub struct Get {
    /// the lookup key
    key: ByteString,
}

impl Get {
    pub(crate) fn parse_frames(parse: &mut Parse) -> anyhow::Result<Get> {
        let key = parse.next_string()?;
        Ok(Get { key })
    }
}

impl CommandExecution for Get {
    async fn apply(
        self,
        dst: &mut Connection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        let response = match ctx.storage.get_async(self.key, ctx.now()).await {
            Some(val) => Frame::Bulk(val),
            None => Frame::Null,
        };

        // info!(?response);

        // Write the response back to the client
        dst.write_frame(&response).await?;

        Ok(())
    }
}
