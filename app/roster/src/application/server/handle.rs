use tracing::info;

use super::cmd::Command;
use super::connection::Connection;

/// Per-connection handler. Reads requests from `connection` and applies the
/// commands.
pub struct Handler {
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    pub connection: Connection,
}

impl Handler {
    /// Process a single connection.
    ///
    /// Request frames are read from the socket and processed. Responses are
    /// written back to the socket.
    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            // TODO: Support pipelining
            let frame_opt = self.connection.read_frame().await?;

            // If `None` is returned from `read_frame()` then the peer closed
            // the socket. There is no further work to do and the task can be
            // terminated.
            let frame = match frame_opt {
                Some(frame) => frame,
                None => return Ok(()),
            };

            info!(?frame);

            // Convert the redis frame into a command struct. This returns an
            // error if the frame is not a valid redis command or it is an
            // unsupported command.
            let cmd = Command::from_frame(frame)?;

            // TODO: Sharding: here the command know if it's about a specific
            // key, so we are able to do the sharding here.
            //
            // Either we send the connection & the cmd to the proper thread

            info!(?cmd);

            cmd.apply(&mut self.connection).await?;
        }
    }
}
