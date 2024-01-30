use super::super::parse::Parse;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// The command just returns the ID of the current connection. Every connection
/// ID has certain guarantees:
///
/// 1. It is never repeated, so if CLIENT ID returns the same number, the caller
///    can be sure that the underlying client did not disconnect and reconnect
///    the connection, but it is still the same connection.
///
/// 2. The ID is monotonically incremental. If the ID of a connection is greater
///    than the ID of another connection, it is guaranteed that the second
///    connection was established with the server at a later time.
///
/// This command is especially useful together with CLIENT UNBLOCK which was
/// introduced also in Redis 5 together with CLIENT ID. Check the CLIENT UNBLOCK
/// command page for a pattern involving the two commands.
#[derive(Debug, Default)]
pub struct ClientID {}

impl ClientID {
    /// Create a new `ClientID`
    pub fn new() -> ClientID {
        ClientID {}
    }

    pub(crate) fn parse_frames(_parse: &mut Parse) -> anyhow::Result<ClientID> {
        Ok(ClientID::new())
    }

    pub(crate) async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        let id = ctx.connection.id();

        let response = Frame::Integer(id);
        dst.write_frame(&response).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::BytesMut;
    use redis_async::resp::{RespCodec, RespValue};
    use redis_async::resp_array;
    use tokio_util::codec::Encoder;

    use crate::application::server::cmd::Command;
    use crate::application::server::frame::Frame;

    fn parse_cmd(obj: RespValue) -> anyhow::Result<Command> {
        let mut bytes = BytesMut::new();
        let mut codec = RespCodec;
        codec.encode(obj, &mut bytes).unwrap();

        let mut bytes = Cursor::new(bytes.freeze());
        let frame = Frame::parse(&mut bytes)?;
        let client_list = Command::from_frame(frame)?;
        Ok(client_list)
    }

    #[test]
    fn ensure_parsing() {
        let entry: RespValue = resp_array!["CLIENT", "ID"];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            Id(
                ClientID,
            ),
        )
        "###);
    }
}
