use bytestring::ByteString;
use tracing::info;

use super::CommandExecution;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// Represents an "unknown" command. This is not a real `Redis` command.
#[derive(Debug)]
pub struct Unknown {
    command_name: String,
}

impl Unknown {
    /// Create a new `Unknown` command which responds to unknown commands
    /// issued by clients
    pub(crate) fn new(key: impl ToString) -> Unknown {
        Unknown {
            command_name: key.to_string(),
        }
    }
}

impl CommandExecution for Unknown {
    async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        let response = Frame::Error(ByteString::from(format!(
            "ERR unknown command '{}'",
            self.command_name
        )));

        info!(?response);

        dst.write_frame(&response).await?;
        Ok(())
    }
}
