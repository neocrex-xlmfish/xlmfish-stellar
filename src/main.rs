// ==================== main.rs ====================
mod config;
mod stellar {
    pub mod horizon;
}

use config::Config;
use stellar::horizon::HorizonClient;

fn main() {
    // Load configuration
    let config = match Config::from_file("Config.toml") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    // Print configuration
    config.print(false);

    // Get Stellar configuration
    let stellar_config = match config.get_blockchain("stellar") {
        Some(cfg) => cfg,
        None => {
            eprintln!("❌ Stellar configuration not found");
            std::process::exit(1);
        }
    };

    // Create Horizon client
    let horizon = HorizonClient::new(&stellar_config.endpoint);

    // Example: Query a testnet account (Stellar testnet friendbot)
    let test_wallet = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
    
    println!("🔍 Fetching account info for: {}\n", test_wallet);
    
    match horizon.print_account_info(test_wallet) {
        Ok(_) => println!("✅ Successfully fetched account info!"),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}

