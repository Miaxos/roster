use super::parse::Parse;
use super::CommandExecution;
use crate::application::server::cmd::unknown::Unknown;
use crate::application::server::cmd::{Command, SubcommandRegistry};
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;

mod set_info;

#[derive(Debug)]
pub enum Client {
    SetInfo(set_info::ClientSetInfo),
}

impl SubcommandRegistry for Client {
    fn from_parse(mut parse: Parse) -> anyhow::Result<Command> {
        let sub_command_name = parse.next_string()?.to_lowercase();

        let command = match &sub_command_name[..] {
            "setinfo" => Command::Client(Client::SetInfo(
                set_info::ClientSetInfo::parse_frames(&mut parse)?,
            )),
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
}

impl CommandExecution for Client {
    async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        match self {
            Client::SetInfo(cmd) => cmd.apply(dst, ctx).await,
        }
    }
}
