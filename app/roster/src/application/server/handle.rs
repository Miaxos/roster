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
            // TODO: Support pipelining
            // 300us
            let now = Instant::now();
            let frame_opt = self.connection.read_frame().await?;
            let elasped = now.elapsed();
            //  dbg!(elasped);

            // If `None` is returned from `read_frame()` then the peer closed
            // the socket. There is no further work to do and the task can be
            // terminated.
            let frame = match frame_opt {
                Some(frame) => frame,
                None => return Ok(()),
            };

            // info!(?frame);

            // Convert the redis frame into a command struct. This returns an
            // error if the frame is not a valid redis command or it is an
            // unsupported command.
            // 100 ns
            let now = Instant::now();
            let cmd = Command::from_frame(frame)?;
            let elasped = now.elapsed();
            // dbg!(elasped);

            // TODO: Sharding: here the command know if it's about a specific
            // key, so we are able to do the sharding here.
            //
            // Either we send the connection & the cmd to the proper thread

            // info!(?cmd);

            // 400us
            let now = Instant::now();
            cmd.apply(&mut self.connection, ctx.clone()).await?;
            let elasped = now.elapsed();
            // dbg!(elasped);
        }
    }
}
