// ==================== config.rs ====================
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asset {
    pub label: String,
    pub code: String,
    pub issuer: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetList {
    pub asset: Vec<Asset>,
}

impl AssetList {
    /// Load asset list from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| ConfigError::IoError(format!("Failed to read {:?}: {}", path.as_ref(), e)))?;
        
        let assets: AssetList = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        Ok(assets)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockchainConfig {
    pub blockchain: String,
    pub network: String,
    pub endpoint: String,
    pub asset_list: String,
}

impl BlockchainConfig {
    /// Load the asset list for this blockchain
    pub fn load_assets(&self) -> Result<AssetList, ConfigError> {
        AssetList::from_file(&self.asset_list)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub blockchain: Vec<BlockchainConfig>,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        Ok(config)
    }

    /// Future: Load configuration from database
    #[allow(dead_code)]
    pub fn from_database(_connection_string: &str) -> Result<Self, ConfigError> {
        // Placeholder for future DB implementation
        Err(ConfigError::NotImplemented("Database loading not yet implemented".to_string()))
    }

    /// Get a specific blockchain configuration by name
    pub fn get_blockchain(&self, name: &str) -> Option<&BlockchainConfig> {
        self.blockchain.iter().find(|bc| bc.blockchain == name)
    }

    /// Get all blockchain configurations
    pub fn get_all_blockchains(&self) -> &[BlockchainConfig] {
        &self.blockchain
    }

    /// Pretty print the configuration with assets
    pub fn print(&self, load_assets: bool) {
        println!("🔗 === Multi-Chain Configuration ===\n");
        for (idx, bc) in self.blockchain.iter().enumerate() {
            let chain_emoji = match bc.blockchain.as_str() {
                "stellar" => "⭐",
                "bitcoin" => "₿",
                "litecoin" => "Ł",
                _ => "🔷",
            };
            
            println!("{} Chain #{}: {}", chain_emoji, idx + 1, bc.blockchain.to_uppercase());
            println!("  🌐 Network:    {}", bc.network);
            println!("  🔌 Endpoint:   {}", bc.endpoint);
            println!("  📋 Asset List: {}", bc.asset_list);
            
            if load_assets {
                match bc.load_assets() {
                    Ok(assets) => {
                        println!("  💰 Assets ({}): ", assets.asset.len());
                        for asset in &assets.asset {
                            println!("    ✓ {} ({}) - Issuer: {}", 
                                asset.label, 
                                asset.code, 
                                &asset.issuer[..8]
                            );
                        }
                    }
                    Err(e) => {
                        println!("  ❌ Assets: Failed to load - {}", e);
                    }
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(String),
    ParseError(String),
    NotImplemented(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO Error: {}", e),
            ConfigError::ParseError(e) => write!(f, "Parse Error: {}", e),
            ConfigError::NotImplemented(e) => write!(f, "Not Implemented: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

