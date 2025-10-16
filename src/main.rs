// File: src/main.rs
// Description: Main application entry point. Loads configuration and
//              assets, then prints them in a human-readable format.

mod config;
use config::Config;

fn main() -> Result<(), String> {
    // Load global config
    let config = Config::load("Config.toml")?;

    println!("Environment: {}", config.global.environment);
    println!("Horizon URL: {}", config.global.horizon_url);

    // Load asset list
    let assets = config.load_assets()?;
    println!("Assets for {}:", config.global.environment);
    for asset in assets {
        println!(" - {}: {} ({})", asset.label, asset.code, asset.issuer);
    }

    Ok(())
}

