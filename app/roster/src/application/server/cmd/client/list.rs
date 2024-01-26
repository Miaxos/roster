use anyhow::bail;

use super::super::parse::Parse;
use crate::application::server::cmd::parse::ParseError;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// The CLIENT LIST command returns information and statistics about the client
/// connections server in a mostly human readable format.
///
/// You can use one of the optional subcommands to filter the list. The TYPE
/// type subcommand filters the list by clients' type, where type is one of
/// normal, master, replica, and pubsub. Note that clients blocked by the
/// MONITOR command belong to the normal class.
///
/// The ID filter only returns entries for clients with IDs matching the
/// client-id arguments.
#[derive(Debug, Default)]
pub struct ClientList {
    r#type: ClientType,
    ids: Option<Vec<u64>>,
}

#[derive(Debug, Default)]
pub enum ClientType {
    #[default]
    NORMAL,
    REPLICA,
    MASTER,
    PUBSUB,
}

impl ClientList {
    /// Create a new `ClientList`
    pub fn new(ty: ClientType, ids: Option<Vec<u64>>) -> ClientList {
        ClientList { r#type: ty, ids }
    }

    pub(crate) fn parse_frames(
        parse: &mut Parse,
    ) -> anyhow::Result<ClientList> {
        let ty_opt = parse.next_string().map(|x| x.to_lowercase());
        let ty = match ty_opt.as_ref().map(|x| &x[..]) {
            Ok("normal") => ClientType::NORMAL,
            Ok("replica") => ClientType::REPLICA,
            Ok("master") => ClientType::MASTER,
            Ok("pubsub") => ClientType::PUBSUB,
            Ok(_) => {
                bail!("should be either normal / replica / master / pubsub.");
            }
            Err(ParseError::EndOfStream) => ClientType::NORMAL,
            Err(err) => {
                bail!("{}", err);
            }
        };

        let mut ids = Vec::new();
        loop {
            match parse.next_int() {
                Ok(id) => {
                    ids.push(id);
                }
                Err(ParseError::EndOfStream) => {
                    break;
                }
                Err(err) => {
                    bail!(err);
                }
            }
        }
        Ok(ClientList::new(ty, Some(ids)))
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

    use bytes::{Bytes, BytesMut};
    use redis_async::resp::{RespCodec, RespValue};
    use redis_async::resp_array;
    use tokio_util::codec::Encoder;

    use crate::application::server::cmd::Command;
    use crate::application::server::frame::Frame;

    fn parse_cmd(obj: RespValue) -> Command {
        let mut bytes = BytesMut::new();
        let mut codec = RespCodec;
        codec.encode(obj, &mut bytes).unwrap();

        let mut bytes = Cursor::new(bytes.freeze());
        let frame = Frame::parse(&mut bytes).unwrap();
        let client_list = Command::from_frame(frame).unwrap();
        client_list
    }

    #[test]
    fn ensure_parsing_base() {
        let entry: RespValue = resp_array!["CLIENT", "LIST"];
        let client_cmd = parse_cmd(entry);
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: NORMAL,
                    ids: Some(
                        [],
                    ),
                },
            ),
        )
        "###);
    }

    #[test]
    fn ensure_parsing_normal() {
        let entry: RespValue = resp_array!["CLIENT", "LIST", "NORMAL"];
        let client_cmd = parse_cmd(entry);
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: NORMAL,
                    ids: Some(
                        [],
                    ),
                },
            ),
        )
        "###);
    }

    #[test]
    fn ensure_parsing_master() {
        let entry: RespValue = resp_array!["CLIENT", "LIST", "MASTER"];
        let client_cmd = parse_cmd(entry);
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: MASTER,
                    ids: Some(
                        [],
                    ),
                },
            ),
        )
        "###);
    }

    #[test]
    fn ensure_parsing_replica() {
        let entry: RespValue = resp_array!["CLIENT", "LIST", "REPLICA"];
        let client_cmd = parse_cmd(entry);
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: REPLICA,
                    ids: Some(
                        [],
                    ),
                },
            ),
        )
        "###);
    }

    #[test]
    fn ensure_parsing_pubsub() {
        let entry: RespValue = resp_array!["CLIENT", "LIST", "PUBSUB"];
        let client_cmd = parse_cmd(entry);
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: PUBSUB,
                    ids: Some(
                        [],
                    ),
                },
            ),
        )
        "###);
    }
}
