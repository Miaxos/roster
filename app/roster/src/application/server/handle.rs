use monoio::time::Instant;
use tracing::info;

use super::cmd::Command;
use super::connection::Connection;
use super::context::Context;

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
    pub async fn run(&mut self, ctx: Context) -> anyhow::Result<()> {
        loop {
            // ----------------------------------------------------------------
            // This should belong to the transport layer
            // ---------------------------------------------------------------
            // TODO: Support pipelining
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
            // 100 ns
            let cmd = Command::from_frame(frame)?;

            // TODO: Command must impl rkyv to be able to be send over another
            // thread

            // ----------------------------------------------------------------
            // Sharding here

            // TODO: Sharding: here the command know if it's about a specific
            // key, so we are able to do the sharding here.
            //
            // Connection is not Send, but if the data is not in the good
            // thread, we still have to communicate the command and wait for the
            // response

            // TODO: Rework the apply because we do not have the initial
            // connection but we should have a stream to the `stream_w` part.
            cmd.apply(&mut self.connection, ctx.clone()).await?;
        }
    }
}
