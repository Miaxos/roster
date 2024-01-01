use config::Config;
use serde::{Deserialize, Serialize};

/// Configuration file for the application.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cfg {
    #[serde(default)]
    pub api: Option<String>,
}

impl Cfg {
    /// Read the associated configuration env
    pub fn from_env() -> anyhow::Result<Cfg> {
        let settings = Config::builder()
            .add_source(
                config::Environment::with_prefix("ROSTER")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()?;

        let config = settings.try_deserialize::<Cfg>()?;

        Ok(config)
    }
}
