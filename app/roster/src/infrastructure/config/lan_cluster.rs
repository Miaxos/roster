use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use url::Url;

/// Lan Clustering configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanClusterConfig {
    /// The bind address we are going to listen to for the LAN Clustering.
    pub bind_addr: SocketAddr,

    // RetryJoin list
    #[serde(default)]
    pub retry_join: Vec<RetryJoinList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryJoinList {
    pub leader_api_addr: Url,
}
