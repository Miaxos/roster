use std::time::Duration;

use bytes::Bytes;
use bytestring::ByteString;

use super::super::parse::{Parse, ParseError};
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::frame::Frame;

/// The CLIENT SETINFO command assigns various info attributes to the current
/// connection which are displayed in the output of CLIENT LIST and CLIENT INFO.
///
/// Client libraries are expected to pipeline this command after authentication
/// on all connections and ignore failures since they could be connected to an
/// older version that doesn't support them.
///
/// Currently the supported attributes are:
///
/// lib-name - meant to hold the name of the client library that's in use.
/// lib-ver - meant to hold the client library's version.
///
/// There is no limit to the length of these attributes. However it is not
/// possible to use spaces, newlines, or other non-printable characters that
/// would violate the format of the CLIENT LIST reply.
///
/// Note that these attributes are not cleared by the RESET command.
#[derive(Debug, Default)]
pub struct ClientSetInfo {
    lib_name: Option<Bytes>,
    lib_version: Option<Bytes>,
}

const LIB_NAME: Bytes = Bytes::from_static(b"LIB-NAME");
const LIB_VERSION: Bytes = Bytes::from_static(b"LIB-VER");

impl ClientSetInfo {
    /// Create a new `ClientSetInfo` command with optional `msg`.
    pub fn new(
        lib_name: Option<Bytes>,
        lib_version: Option<Bytes>,
    ) -> ClientSetInfo {
        ClientSetInfo {
            lib_name,
            lib_version,
        }
    }

    /// Parse a `ClientSetInfo` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `ClientSetInfo` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `ClientSetInfo` value on success. If the frame is malformed,
    /// `Err` is returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing `ClientSetInfo` and an optional
    /// lib_name & lib_version.
    ///
    /// ```text
    /// CLIENT SETINFO <LIB-NAME libname | LIB-VER libver>
    /// ```
    pub(crate) fn parse_frames(
        parse: &mut Parse,
    ) -> anyhow::Result<ClientSetInfo> {
        let mut lib_name = None;
        let mut lib_version = None;

        match (parse.next_bytes(), parse.next_bytes()) {
            (Ok(key), Ok(val)) => {
                if key == LIB_NAME {
                    lib_name = Some(val);
                } else if key == LIB_VERSION {
                    lib_version = Some(val);
                }
            }
            (Err(ParseError::EndOfStream), _)
            | (_, Err(ParseError::EndOfStream)) => {
                return Ok(ClientSetInfo::new(lib_name, lib_version))
            }
            (Err(e), _) | (_, Err(e)) => return Err(e.into()),
        }

        Ok(ClientSetInfo::new(lib_name, lib_version))
    }

    /// Apply the `ClientSetInfo` command and return the message.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    pub(crate) async fn apply(
        self,
        dst: &mut WriteConnection,
        ctx: Context,
    ) -> anyhow::Result<()> {
        monoio::time::sleep(Duration::from_secs(3)).await;
        /*
        let response = match self.msg {
            None => Frame::Simple("PONG".to_string()),
            Some(msg) => Frame::Bulk(msg),
        };

        info!(?response);

        // Write the response back to the client
        dst.write_frame(&response).await?;

            */

        let response = Frame::Simple(ByteString::from_static("OK"));
        dst.write_frame(&response).await?;

        Ok(())
    }
}
