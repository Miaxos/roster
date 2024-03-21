use serde::{Deserialize, Serialize};
use ulid::serde::ulid_as_u128;
use ulid::Ulid;

/// Every [ResponseLAN] are inside a ResponseEnveloppe when going into the
/// network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseEnveloppe {
    #[serde(with = "ulid_as_u128")]
    request_id: Ulid,

    kind: ResponseLAN,
}

/// The list of every possible requests
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseLAN {
    Nothing,
}
