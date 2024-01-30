use anyhow::bail;
use bytestring::ByteString;

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
    #[allow(dead_code)]
    r#type: ClientType,
    #[allow(dead_code)]
    ids: Option<Vec<u64>>,
}

#[derive(Debug, Default)]
pub enum ClientType {
    #[default]
    Normal,
    Replica,
    Master,
    Pubsub,
}

impl ClientList {
    /// Create a new `ClientList`
    pub fn new(ty: ClientType, ids: Option<Vec<u64>>) -> ClientList {
        ClientList { r#type: ty, ids }
    }

    pub(crate) fn parse_frames(
        parse: &mut Parse,
    ) -> anyhow::Result<ClientList> {
        let mut ty = ClientType::Normal;
        let mut ids = Vec::new();

        loop {
            let key_filter = parse.next_string().map(|x| x.to_lowercase());
            let key_filter_str = key_filter.as_ref().map(|x| &x[..]);
            match key_filter_str {
                Ok("type") => {
                    let ty_opt = parse.next_string().map(|x| x.to_lowercase());
                    ty = match ty_opt.as_ref().map(|x| &x[..]) {
                        Ok("normal") => ClientType::Normal,
                        Ok("replica") => ClientType::Replica,
                        Ok("master") => ClientType::Master,
                        Ok("pubsub") => ClientType::Pubsub,
                        Ok(_) => {
                            bail!(
                                "Unknown client type, should be either normal \
                                 / replica / master / pubsub."
                            );
                        }
                        Err(ParseError::EndOfStream) => ClientType::Normal,
                        Err(err) => {
                            bail!("{}", err);
                        }
                    };
                }
                Ok("id") => loop {
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
                },
                Ok(rest) => {
                    bail!("Unknown filter type '{rest}'");
                }
                Err(ParseError::EndOfStream) => {
                    break;
                }
                Err(err) => {
                    bail!("{}", err);
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
        let connections = ctx.supervisor.get_normal_connection().await;

        // TODO(@miaxos): lot of things missing here
        let mut conn_frames = Vec::with_capacity(connections.len());
        for conn in connections {
            conn_frames.push(Frame::Simple(ByteString::from(format!(
                "id={id} addr={addr} laddr={laddr} fd={fd} name={name}",
                id = &conn.id,
                addr = &conn.addr,
                laddr = &conn.laddr,
                fd = &conn.fd,
                name = &conn.name().await.unwrap_or(ByteString::new()),
            ))));
        }

        let response = Frame::Array(conn_frames);
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
    fn ensure_parsing_base() {
        let entry: RespValue = resp_array!["CLIENT", "LIST"];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: Normal,
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
        let entry: RespValue = resp_array!["CLIENT", "LIST", "TYPE", "NORMAL"];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: Normal,
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
        let entry: RespValue = resp_array!["CLIENT", "LIST", "TYPE", "MASTER"];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: Master,
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
        let entry: RespValue = resp_array!["CLIENT", "LIST", "TYPE", "REPLICA"];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: Replica,
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
        let entry: RespValue = resp_array!["CLIENT", "LIST", "TYPE", "PUBSUB"];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: Pubsub,
                    ids: Some(
                        [],
                    ),
                },
            ),
        )
        "###);
    }

    #[test]
    fn ensure_parsing_fail() {
        let entry: RespValue = resp_array!["CLIENT", "LIST", "TYPE", "FAIL"];
        let client_cmd = parse_cmd(entry);
        assert!(client_cmd.is_err());
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Err(
            "Unknown client type, should be either normal / replica / master / pubsub.",
        )
        "###);
    }

    #[test]
    fn ensure_parsing_normal_id() {
        let entry: RespValue =
            resp_array!["CLIENT", "LIST", "TYPE", "NORMAL", "ID", 1, 2];
        let client_cmd = parse_cmd(entry).unwrap();
        insta::assert_debug_snapshot!(client_cmd, @r###"
        Client(
            List(
                ClientList {
                    type: Normal,
                    ids: Some(
                        [
                            1,
                            2,
                        ],
                    ),
                },
            ),
        )
        "###);
    }
}
