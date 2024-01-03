use self::client::Client;
use self::parse::Parse;
use self::ping::Ping;
use self::set::Set;
use self::unknown::Unknown;
use super::connection::Connection;
use super::context::Context;
use super::frame::Frame;

mod parse;

mod client;
mod ping;
mod set;
mod unknown;

/// Enumeration of supported Redis commands.
///
/// Methods called on `Command` are delegated to the command implementation.
#[derive(Debug)]
pub enum Command {
    Client(Client),
    Ping(Ping),
    Set(Set),
    Unknown(Unknown),
}

pub trait CommandExecution: Sized {
    /// Apply a command or a subcommand.
    async fn apply(
        self,
        dst: &mut Connection,
        ctx: Context,
    ) -> anyhow::Result<()>;
}

pub trait SubcommandRegistry {
    /// Parse a sub-command from an already parsed frame .
    ///
    /// # Returns
    ///
    /// On success, the command value is returned, otherwise, `Err` is returned.
    fn from_parse(parse: Parse) -> anyhow::Result<Command>;
}

impl Command {
    /// Parse a command from a received frame.
    ///
    /// The `Frame` must represent a Redis command supported by `roster` and
    /// be the array variant.
    ///
    /// # Returns
    ///
    /// On success, the command value is returned, otherwise, `Err` is returned.
    pub fn from_frame(frame: Frame) -> anyhow::Result<Command> {
        // The frame value is decorated with `Parse`. `Parse` provides a
        // "cursor" like API which makes parsing the command easier.
        //
        // The frame value must be an array variant. Any other frame variants
        // result in an error being returned.
        let mut parse = Parse::new(frame)?;

        // All redis commands begin with the command name as a string. The name
        // is read and converted to lower cases in order to do case sensitive
        // matching.
        let command_name = parse.next_string()?.to_lowercase();

        // Match the command name, delegating the rest of the parsing to the
        // specific command.
        let command = match &command_name[..] {
            "client" => {
                return Client::from_parse(parse);
            }
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            _ => {
                // The command is not recognized and an Unknown command is
                // returned.
                //
                // `return` is called here to skip the `finish()` call below. As
                // the command is not recognized, there is most likely
                // unconsumed fields remaining in the `Parse` instance.
                return Ok(Command::Unknown(Unknown::new(command_name)));
            }
        };

        // Check if there is any remaining unconsumed fields in the `Parse`
        // value. If fields remain, this indicates an unexpected frame format
        // and an error is returned.
        parse.finish()?;

        // The command has been successfully parsed
        Ok(command)
    }

    /// Apply the command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    pub(crate) async fn apply(
        self,
        dst: &mut Connection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        use Command::*;

        match self {
            Ping(cmd) => cmd.apply(dst, ctx).await,
            Unknown(cmd) => cmd.apply(dst, ctx).await,
            Client(cmd) => cmd.apply(dst, ctx).await,
            Set(cmd) => cmd.apply(dst, ctx).await,
        }
    }
}
