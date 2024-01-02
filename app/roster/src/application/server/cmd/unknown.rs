use tracing::info;

use crate::application::server::connection::Connection;
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

    /// Returns the command name
    pub(crate) fn get_name(&self) -> &str {
        &self.command_name
    }

    /// Responds to the client, indicating the command is not recognized.
    pub(crate) async fn apply(
        self,
        dst: &mut Connection,
    ) -> anyhow::Result<()> {
        let response = Frame::Error(format!(
            "ERR unknown command '{}'",
            self.command_name
        ));

        info!(?response);

        dst.write_frame(&response).await?;
        Ok(())
    }
}
