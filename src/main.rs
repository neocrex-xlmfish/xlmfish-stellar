// ==================== main.rs ====================
mod config;

use config::Config;

fn main() {
    // Load configuration from TOML file
    let config = match Config::from_file("Config.toml") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    // Print all configurations with assets loaded
    config.print(true);

    // Example: Work with a specific blockchain
    if let Some(stellar) = config.get_blockchain("stellar") {
        println!("⭐ === Working with Stellar ===");
        println!("🔌 Endpoint: {}", stellar.endpoint);
        
        match stellar.load_assets() {
            Ok(assets) => {
                println!("✅ Loaded {} assets for Stellar", assets.asset.len());
                for asset in &assets.asset {
                    println!("  💎 {} ({}) from {}", 
                        asset.label, 
                        asset.code, 
                        asset.issuer
                    );
                }
            }
            Err(e) => eprintln!("❌ Failed to load Stellar assets: {}", e),
        }
    }

    // Example: Iterate through all blockchains
    println!("\n🔄 === Processing All Chains ===");
    for bc in config.get_all_blockchains() {
        println!("Processing {} on {} network...", bc.blockchain, bc.network);
        // Add your blockchain-specific logic here
    }
}

