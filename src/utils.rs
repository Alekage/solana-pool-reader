use reqwest::Client;
use std::time::Duration;

/// Creates a configured HTTP client with timeout
pub fn create_http_client(timeout: Duration) -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(timeout)
        .user_agent("Solana-Pool-Reader/1.0")
        .build()
}

/// Validates Solana token mint addresses
pub fn validate_token_mint(mint: &str) -> bool {
    // Basic validation - Solana addresses are base58 encoded and typically 32-44 characters
    if mint.is_empty() || mint.len() > 44 {
        return false;
    }
    
    // Check if it contains only base58 characters
    mint.chars().all(|c| {
        c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l'
    })
}

/// Formats error response with additional context
pub fn format_error_response(error: &str, token_a: &str, token_b: &str) -> serde_json::Value {
    serde_json::json!({
        "error": error,
        "tokens": {
            "token_mint_a": token_a,
            "token_mint_b": token_b
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })
}


