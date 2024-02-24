use serde::{Deserialize, Serialize};
use ulid::serde::ulid_as_u128;
use ulid::Ulid;

/// Every [RequestLAN] are inside a RequestEnveloppe when going into the
/// network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestEnveloppe {
    #[serde(with = "ulid_as_u128")]
    request_id: Ulid,

    kind: RequestLAN,
}

/// The list of every possible requests
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestLAN {
    Nothing,
}
