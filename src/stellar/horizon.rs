// ==================== stellar/horizon.rs ====================
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    pub balance: String,
    pub asset_type: String,
    #[serde(default)]
    pub asset_code: Option<String>,
    #[serde(default)]
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInfo {
    pub id: String,
    pub sequence: String,
    pub balances: Vec<Balance>,
    #[serde(default)]
    pub subentry_count: u32,
}

pub struct HorizonClient {
    endpoint: String,
    client: reqwest::blocking::Client,
}

impl HorizonClient {
    /// Create a new Horizon client
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Get account information for a given wallet address
    pub fn get_account_info(&self, wallet_address: &str) -> Result<AccountInfo, HorizonError> {
        let url = format!("{}/accounts/{}", self.endpoint, wallet_address);
        
        let response = self.client
            .get(&url)
            .send()
            .map_err(|e| HorizonError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(HorizonError::ApiError(
                response.status().as_u16(),
                format!("Failed to fetch account: {}", response.status())
            ));
        }

        let account: AccountInfo = response
            .json()
            .map_err(|e| HorizonError::ParseError(e.to_string()))?;

        Ok(account)
    }

    /// Pretty print account information
    pub fn print_account_info(&self, wallet_address: &str) -> Result<(), HorizonError> {
        let info = self.get_account_info(wallet_address)?;
        
        println!("⭐ === Stellar Account Info ===");
        println!("📧 Address: {}", info.id);
        println!("🔢 Sequence: {}", info.sequence);
        println!("📊 Subentries: {}", info.subentry_count);
        println!("\n💰 Balances:");
        
        for balance in &info.balances {
            match balance.asset_type.as_str() {
                "native" => {
                    println!("  ⭐ XLM (Native): {} XLM", balance.balance);
                }
                "credit_alphanum4" | "credit_alphanum12" => {
                    let code = balance.asset_code.as_deref().unwrap_or("UNKNOWN");
                    let issuer = balance.asset_issuer.as_deref().unwrap_or("UNKNOWN");
                    println!("  💎 {}: {} (Issuer: {}...)", 
                        code, 
                        balance.balance,
                        &issuer[..8]
                    );
                }
                _ => {
                    println!("  🔷 {}: {}", balance.asset_type, balance.balance);
                }
            }
        }
        println!();
        
        Ok(())
    }
}

#[derive(Debug)]
pub enum HorizonError {
    NetworkError(String),
    ApiError(u16, String),
    ParseError(String),
}

impl std::fmt::Display for HorizonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HorizonError::NetworkError(e) => write!(f, "Network Error: {}", e),
            HorizonError::ApiError(code, msg) => write!(f, "API Error {}: {}", code, msg),
            HorizonError::ParseError(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl std::error::Error for HorizonError {}

