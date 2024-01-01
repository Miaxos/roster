mod infrastructure;
use infrastructure::config::Cfg;

fn main() -> anyhow::Result<()> {
    // Initialize config
    let _config = Cfg::from_env()?;

    println!("hello world");
    Ok(())
}
