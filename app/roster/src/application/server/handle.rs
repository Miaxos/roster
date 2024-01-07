use super::cmd::Command;
use super::connection::{ReadConnection, WriteConnection};
use super::context::Context;

/// Per-connection handler. Reads requests from `connection` and applies the
/// commands.
pub struct Handler {
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    pub connection: WriteConnection,
    pub connection_r: ReadConnection,
}

impl Handler {
    /// Request frames are read from the socket and processed. Responses are
    /// written back to the socket.
    pub async fn run(self, ctx: Context) -> anyhow::Result<()> {
        let (tx, mut rx) = local_sync::mpsc::unbounded::channel();

        let mut connection_r = self.connection_r;
        let mut connection = self.connection;

        let accepting_frames_handle = monoio::spawn(async move {
            loop {
                let frame_opt = connection_r.read_frame().await?;

                // If `None` is returned from `read_frame()` then the peer
                // closed the socket. There is no further work
                // to do and the task can be terminated.
                let frame = match frame_opt {
                    Some(frame) => frame,
                    None => return Ok::<_, anyhow::Error>(()),
                };

                tx.send(frame).unwrap();
            }
        });

        let answer_in_order_handle = monoio::spawn(async move {
            while let Some(frame) = rx.recv().await {
                // Convert the redis frame into a command struct. This returns
                // an error if the frame is not a valid redis
                // command or it is an unsupported command.
                // 100 ns
                let cmd = Command::from_frame(frame)?;

                // ----------------------------------------------------------------
                // Sharding here

                // TODO: Sharding: here the command know if it's about a
                // specific key, so we are able to do the
                // sharding here.

                // Connection is not Send, but if the data is not in the good
                // thread, we still have to communicate the command and wait for
                // the response
                cmd.apply(&mut connection, ctx.clone()).await?;
            }
            Ok::<_, anyhow::Error>(())
        });

        monoio::select! {
            r = accepting_frames_handle => {
                return r;
            }
            r = answer_in_order_handle => {
                return r;
            }
        }
    }
}
