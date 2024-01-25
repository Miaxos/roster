#![allow(clippy::await_holding_refcell_ref)]
// TODO(@miaxos): Remove it

use std::cell::RefCell;
use std::os::fd::IntoRawFd;
use std::rc::Rc;
use std::time::Duration;

use sharded_thread::shard::Shard;

use super::cmd::Command;
use super::connection::{ReadConnection, WriteConnection};
use super::context::Context;
use super::frame::Frame;
use crate::application::server::cmd::CommandExecution;

/// Per-connection handler. Reads requests from `connection` and applies the
/// commands.
pub struct Handler {
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    pub connection: WriteConnection,
    pub connection_r: ReadConnection,
    pub shard: Rc<Shard<ConnectionMsg>>,
}

/// Current connection that is going to be send
#[derive(Debug)]
pub struct ConnectionMsg {
    pub fd: i32,
    pub current_command: Command,
    pub rest_frame: Vec<Frame>, // rest_buffer
}

impl Handler {
    /// Request frames are read from the socket and processed. Responses are
    /// written back to the socket.
    pub async fn run(self, ctx: Context) -> anyhow::Result<()> {
        self.run_internal(ctx, None).await
    }

    #[allow(dead_code)]
    pub async fn continue_run(
        self,
        ctx: Context,
        current_command: Command,
    ) -> anyhow::Result<()> {
        self.run_internal(ctx, Some(current_command)).await
    }

    #[inline]
    async fn run_internal(
        self,
        ctx: Context,
        current_command: Option<Command>,
    ) -> anyhow::Result<()> {
        let Handler {
            mut connection,
            connection_r,
            shard,
        } = self;
        let (tx, mut rx) = local_sync::mpsc::unbounded::channel();

        let (shutdown_tx, shutdown_rx) = local_sync::oneshot::channel::<()>();
        let (shutdown_ok_tx, shutdown_ok_rx) = local_sync::oneshot::channel();

        let accepting_frames_handle = monoio::spawn(async move {
            let connection_r = Rc::new(RefCell::new(connection_r));

            let conn = connection_r.clone();
            let reading_frames = async move {
                loop {
                    let frame_opt = conn.borrow_mut().read_frame().await?;

                    // If `None` is returned from `read_frame()` then the peer
                    // closed the socket. There is no further work
                    // to do and the task can be terminated.
                    let frame = match frame_opt {
                        Some(frame) => frame,
                        None => return Ok::<_, anyhow::Error>(()),
                    };

                    tx.send(frame).unwrap();
                }
            };

            let result = monoio::select! {
                r = reading_frames => {
                    r
                }
                r = shutdown_rx => {
                    match r {
                      Ok(_) => {
                        shutdown_ok_tx.send(connection_r).map_err(|_| anyhow::anyhow!("counldn't send"))?;
                        Ok(())
                      },
                      Err(err) => {
                            Err(anyhow::anyhow!(err))
                        }
                    }
                }
            };

            result
        });

        let answer_in_order_handle = monoio::spawn(async move {
            if let Some(current_command) = current_command {
                current_command.apply(&mut connection, ctx.clone()).await?;
            }

            while let Some(frame) = rx.recv().await {
                // Convert the redis frame into a command struct. This returns
                // an error if the frame is not a valid redis
                // command or it is an unsupported command.
                // 100 ns
                let cmd = Command::from_frame(frame)?;

                // ----------------------------------------------------------------
                // Sharding here

                let hash = cmd.hash_key();

                /*
                let is_in_slot =
                    hash.map(|hash| ctx.is_in_slot(hash)).unwrap_or(true);
                */
                let is_in_slot = true;

                if !is_in_slot {
                    let hash = hash.unwrap();
                    // We need to reunite it, so we need to shutdown the reading
                    // We can have the own on write_connection, so we need to
                    // get the read_connection
                    shutdown_tx.send(()).map_err(|_| {
                        anyhow::anyhow!("couldn't send shutdown")
                    })?;

                    let tcp = connection.reunite(
                        Rc::try_unwrap(shutdown_ok_rx.await?)
                            .unwrap()
                            .into_inner(),
                    );
                    // Sad, but it seems the destructor of tcp are not running
                    // if we do not put this. To investigate
                    // on monoio repo later.
                    monoio::time::sleep(Duration::from_nanos(1)).await;
                    let shard_target = ctx.slot_nb(hash).unwrap();

                    shard.send_to(
                        ConnectionMsg {
                            fd: tcp.into_raw_fd(),
                            current_command: cmd,
                            rest_frame: Vec::new(),
                        },
                        shard_target,
                    )?;

                    break;
                } else {
                    // Connection is not Send, but if the data is not in the
                    // good thread, we still have to
                    // communicate the command and wait for
                    // the response
                    cmd.apply(&mut connection, ctx.clone()).await?;
                }
            }
            Ok::<_, anyhow::Error>(())
        });

        monoio::select! {
            r = accepting_frames_handle => {
                r
            }
            r = answer_in_order_handle => {
                r
            }
        }
    }
}
