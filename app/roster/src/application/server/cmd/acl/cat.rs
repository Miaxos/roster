use bytestring::ByteString;

use super::super::parse::Parse;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;

/// The command shows the available ACL categories if called without arguments.
/// If a category name is given, the command shows all the Redis commands in the
/// specified category.
///
/// ACL CAT [category]
#[derive(Debug, Default)]
pub struct AclCat {
    #[allow(dead_code)]
    category: Option<ByteString>,
}

impl AclCat {
    /// Create a new `AclCat` command with optional `msg`.
    pub fn new(category: Option<ByteString>) -> AclCat {
        AclCat { category }
    }

    /// Parse a `AclCat` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `AclCat` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `AclCat` value on success. If the frame is malformed,
    /// `Err` is returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing `ACL CAT` and an optional
    /// category.
    ///
    /// ```text
    /// ACL CAT [category]
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parse) -> anyhow::Result<AclCat> {
        let category = parse.next_string().ok();

        Ok(AclCat::new(category))
    }

    /// Apply the `AclCat` command and return the message.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    pub(crate) async fn apply(
        self,
        _dst: &mut WriteConnection,
        _ctx: Context,
    ) -> anyhow::Result<()> {
        unimplemented!()

        /*
        let response = Frame::Simple(ByteString::from_static("OK"));
        dst.write_frame(&response).await?;
        Ok(())
        */
    }
}
