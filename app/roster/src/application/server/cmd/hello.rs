use crate::application::server::cmd::Parse;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// Switch to a different protocol, optionally authenticating and setting the
/// connection's name, or provide a contextual client report.
///
/// HELLO always replies with a list of current server and connection
/// properties, such as: versions, modules loaded, client ID, replication role
/// and so forth.
///
/// In Roster we only reply in RESP 3.
#[derive(Debug, Default)]
pub struct Hello {}

impl Hello {
    pub fn new() -> Hello {
        Hello {}
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> anyhow::Result<Hello> {
        parse.finish()?;
        Ok(Hello::new())
    }

    pub(crate) async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        let name = ctx.connection.name().await;

        let response = match name {
            Some(name) => Frame::Bulk(name.into_bytes()),
            None => Frame::Null,
        };
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
        let entry: RespValue = resp_array!["CLIENT", "GETNAME"];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            GetName(
                ClientGetName,
            ),
        )
        "###);
    }

    #[test]
    fn ensure_parsing_too_much() {
        let entry: RespValue = resp_array!["CLIENT", "GETNAME", "BLBL"];
        let client_cmd = parse_cmd(entry);
        assert!(client_cmd.is_err());
        let client_cmd = client_cmd.unwrap_err();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Other(
            "protocol error; expected end of frame, but there was more",
        )
        "###);
    }
}
