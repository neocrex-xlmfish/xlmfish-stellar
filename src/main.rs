use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Network {
    environment: String,
}

#[derive(Debug, Deserialize)]
struct Horizon {
    url: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    network: Network,
    horizon: Horizon,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read and parse the TOML configuration file
    let contents = fs::read_to_string("Config.toml")?;
    let config: Config = toml::from_str(&contents)?;

    // Output the results
    println!("Environment: {}", config.network.environment);
    println!("Horizon URL: {}", config.horizon.url);

    Ok(())
}

