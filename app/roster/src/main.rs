mod infrastructure;

use infrastructure::config::Cfg;
use infrastructure::instruments::Instruments;
use tracing::info;

fn main() -> anyhow::Result<()> {
    // Initialize config
    let _config = Cfg::from_env()?;

    // Initialize tracing
    let _ = Instruments::new()?;

    info!("hello world");
    Ok(())
}
