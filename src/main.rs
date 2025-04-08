use std::env;
use std::fs::File;
use std::io::Read;
use anyhow::{Result, Context, bail};
use toml::Value;

/// Validate that the RELAY_KEY is available either in the TOML file
/// or as an environment variable, with environment variables taking precedence.
pub fn validate_relay_key(config_path: &str) -> Result<()> {
    println!("Validating relay key configuration...");
    
    // Read the TOML file
    let mut config_content = String::new();
    File::open(config_path)
        .with_context(|| format!("Failed to open config file: {}", config_path))?
        .read_to_string(&mut config_content)
        .with_context(|| format!("Failed to read config file: {}", config_path))?;
    
    // Parse the TOML file
    let config: Value = toml::from_str(&config_content)
        .context("Failed to parse TOML configuration")?;
    
    // Check relay key configuration - first from environment, then from config file
    let relay_key = env::var("RELAY_KEY").ok().or_else(|| {
        config.get("relay")
            .and_then(|relay| relay.get("key"))
            .and_then(|key| key.as_str())
            .map(String::from)
    });
    
    if relay_key.is_none() {
        bail!("RELAY_KEY is not set in environment variables or in config.toml");
    }
    
    // Validate that the relay key is in the correct format (0x followed by 64 hex characters)
    if let Some(key) = &relay_key {
        if !key.starts_with("0x") || key.len() != 66 || !key[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            bail!("RELAY_KEY is not in the correct format. It should be 0x followed by 64 hex characters.");
        }
    }
    
    println!("✓ RELAY_KEY is valid");
    Ok(())
}

/// Example usage
fn example() -> Result<()> {
    // Get configuration path from environment variable or use default
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    
    // Validate configuration
    validate_relay_key(&config_path)?;
    
    println!("Configuration is valid");
    Ok(())
}

fn main() {
    example().unwrap();
}