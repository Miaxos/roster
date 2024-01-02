use tracing::info;

use super::cmd::Command;
use super::connection::Connection;

/// Per-connection handler. Reads requests from `connection` and applies the
/// commands to `db`.
pub struct Handler {
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    ///
    /// When `Listener` receives an inbound connection, the `TcpStream` is
    /// passed to `Connection::new`, which initializes the associated buffers.
    /// `Connection` allows the handler to operate at the "frame" level and keep
    /// the byte level protocol parsing details encapsulated in `Connection`.
    pub connection: Connection,
}

impl Handler {
    /// Process a single connection.
    ///
    /// Request frames are read from the socket and processed. Responses are
    /// written back to the socket.
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let frame_opt = self.connection.read_frame().await?;

        // If `None` is returned from `read_frame()` then the peer closed
        // the socket. There is no further work to do and the task can be
        // terminated.
        let frame = match frame_opt {
            Some(frame) => frame,
            None => return Ok(()),
        };

        // Convert the redis frame into a command struct. This returns an
        // error if the frame is not a valid redis command or it is an
        // unsupported command.
        let cmd = Command::from_frame(frame)?;

        info!(?cmd);

        cmd.apply(&mut self.connection).await?;

        Ok(())
    }
}
