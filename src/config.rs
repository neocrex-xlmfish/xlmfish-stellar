// src/config.rs
// Handles loading chains and asset lists

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ChainsConfig {
    pub chains: HashMap<String, ChainInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ChainInfo {
    pub chain_name: String,
    pub api_endpoint: String,
    pub asset_list: String,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub label: String,
    pub code: String,
    pub issuer: String,
}

/// Load the chains configuration
pub fn load_chains_config(path: &str) -> Result<ChainsConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: ChainsConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Load asset list for a chain
pub fn load_assets(path: &str) -> Result<Vec<Asset>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let assets_file: AssetsFile = toml::from_str(&content)?;
    Ok(assets_file.asset)
}

#[derive(Debug, Deserialize)]
struct AssetsFile {
    pub asset: Vec<Asset>,
}
