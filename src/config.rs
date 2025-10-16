// File: src/config.rs
// Description: Contains structs and logic for loading the global configuration
//              and asset lists from TOML files. Supports switching between
//              testnet and public/mainnet asset lists.

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub global: GlobalConfig,
    pub assets: AssetFiles,
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub environment: String,
    pub horizon_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetFiles {
    pub testnet_file: String,
    pub public_file: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetList {
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub label: String,
    pub code: String,
    pub issuer: String,
}

impl Config {
    /// Load the main Config TOML file
    pub fn load(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error reading {}: {}", path, e))?;
        toml::from_str(&content).map_err(|e| format!("TOML parse error: {}", e))
    }

    /// Load the asset list depending on the environment (testnet/public)
    pub fn load_assets(&self) -> Result<Vec<Asset>, String> {
        let file = match self.global.environment.as_str() {
            "testnet" => &self.assets.testnet_file,
            "public" => &self.assets.public_file,
            _ => &self.assets.testnet_file,
        };

        let content = fs::read_to_string(file)
            .map_err(|e| format!("Error reading {}: {}", file, e))?;
        let asset_list: AssetList = toml::from_str(&content)
            .map_err(|e| format!("TOML parse error: {}", e))?;
        Ok(asset_list.assets)
    }
}

