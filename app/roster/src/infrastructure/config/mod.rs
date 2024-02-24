use std::net::SocketAddr;

use anyhow::Context;
use config::Config;
use serde::{Deserialize, Serialize};

mod lan_cluster;
pub use lan_cluster::LanClusterConfig;

/// Configuration file for the application.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cfg {
    /// The bind address we are going to listen to.
    pub bind_addr: SocketAddr,
    /// Maximum number of concurrent connections the redis server will accept.
    ///
    /// When this limit is reached, the server will stop accepting connections
    /// until an active connection terminates.
    pub max_connection: u16,
    /// The lan cluster describe a cluster which should only be available
    /// "locally" with a quite fast path between each members
    pub lan_cluster: LanClusterConfig,
}

impl Cfg {
    /// Read the associated configuration env
    pub fn from_env() -> anyhow::Result<Cfg> {
        let file_location = dotenv::var("CONFIG_FILE_LOCATION")
            .with_context(|| "`CONFIG_FILE_LOCATION` must be set.")?;

        let settings = Config::builder()
            .add_source(config::File::with_name(&file_location))
            .add_source(
                config::Environment::with_prefix("ROSTER")
                    .try_parsing(false)
                    .separator("_"),
            )
            .build()?;

        let config = settings.try_deserialize::<Cfg>()?;

        Ok(config)
    }
}
