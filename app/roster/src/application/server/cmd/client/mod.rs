use super::parse::{Parse, ParseError};
use super::CommandExecution;
use crate::application::server::cmd::unknown::Unknown;
use crate::application::server::cmd::{Command, SubcommandRegistry};
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

mod id;
mod info;
mod list;
mod set_info;
mod set_name;

#[derive(Debug)]
pub enum Client {
    Help,
    SetInfo(set_info::ClientSetInfo),
    SetName(set_name::ClientSetName),
    List(list::ClientList),
    Id(id::ClientID),
    Info(info::ClientInfo),
}

// TODO(@miaxos): This is a simple implementation of the HELP to have the
// associated test, but the idea is to change it to an autogenerated help based
// on commands available and the documentation of the structure.
const HELP_TEXT: &str = r#"CLIENT <subcommand> [<arg> [value] [opt] ...]. subcommands are:
ID
    Return the ID of the current connection.
INFO
    Return information about the current client connection.
LIST [options ...]
    Return information about client connections. Options:
    * TYPE (NORMAL|MASTER|REPLICA|PUBSUB)
      Return clients of specified type.
SETNAME <name>
    Assign the name <name> to the current connection.
SETINFO <option> <value>
    Set client meta attr. Options are:
    * LIB-NAME: the client lib name.
    * LIB-VER: the client lib version.
HELP
    Print this help.
"#;

impl SubcommandRegistry for Client {
    fn from_parse(mut parse: Parse) -> anyhow::Result<Command> {
        let sub_cmd = match parse.next_string() {
            Ok(elt) => elt,
            Err(ParseError::EndOfStream) => {
                return Ok(Command::Client(Client::Help))
            }
            Err(err) => {
                return Err(err.into());
            }
        };

        let sub_command_name = sub_cmd.to_lowercase();

        let command = match &sub_command_name[..] {
            "setinfo" => Command::Client(Client::SetInfo(
                set_info::ClientSetInfo::parse_frames(&mut parse)?,
            )),
            "setname" => Command::Client(Client::SetName(
                set_name::ClientSetName::parse_frames(&mut parse)?,
            )),
            "id" => Command::Client(Client::Id(id::ClientID::parse_frames(
                &mut parse,
            )?)),
            "info" => Command::Client(Client::Info(
                info::ClientInfo::parse_frames(&mut parse)?,
            )),
            "list" => Command::Client(Client::List(
                list::ClientList::parse_frames(&mut parse)?,
            )),
            "help" => Command::Client(Client::Help),
            _ => {
                // The command is not recognized and an Unknown command is
                // returned.
                //
                // `return` is called here to skip the `finish()` call below. As
                // the command is not recognized, there is most likely
                // unconsumed fields remaining in the `Parse` instance.
                return Ok(Command::Unknown(Unknown::new(sub_command_name)));
            }
        };

        // Check if there is any remaining unconsumed fields in the `Parse`
        // value. If fields remain, this indicates an unexpected frame format
        // and an error is returned.
        parse.finish()?;

        // The command has been successfully parsed
        Ok(command)
    }

    async fn help(
        dst: &mut WriteConnection,
        _ctx: Context,
    ) -> anyhow::Result<()> {
        let response = Frame::Array(
            HELP_TEXT
                .split_terminator('\n')
                .map(|x| Frame::Simple(x.into()))
                .collect(),
        );
        dst.write_frame(&response).await?;
        Ok(())
    }
}

impl CommandExecution for Client {
    async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        match self {
            Client::Help => Client::help(dst, ctx).await,
            Client::SetInfo(cmd) => cmd.apply(dst, ctx).await,
            Client::SetName(cmd) => cmd.apply(dst, ctx).await,
            Client::Id(cmd) => cmd.apply(dst, ctx).await,
            Client::Info(cmd) => cmd.apply(dst, ctx).await,
            Client::List(cmd) => cmd.apply(dst, ctx).await,
        }
    }
}
