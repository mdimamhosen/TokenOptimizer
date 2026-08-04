//! AI Assist Core - Universal analytics engine for AI coding assistants
//!
//! Provides detection, analysis, and optimization capabilities for multiple AI tools.

pub mod adapters;
pub mod database;
pub mod detection;
pub mod error;
pub mod models;
pub mod optimization;
pub mod token_counter;

pub use error::{Error, Result};

use std::path::PathBuf;

/// Core configuration for AI Assist
#[derive(Debug, Clone)]
pub struct Config {
    /// Home directory path
    pub home: PathBuf,
    /// Data directory path (~/.ai-assist)
    pub data_dir: PathBuf,
    /// Database path
    pub db_path: PathBuf,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Config {
    /// Create new config with default paths
    pub fn new() -> Result<Self> {
        let home =
            dirs::home_dir().ok_or(Error::ConfigError("Cannot determine home directory".into()))?;
        let data_dir = home.join(".ai-assist");
        let db_path = data_dir.join("ai_assist.db");

        Ok(Config {
            home,
            data_dir,
            db_path,
            verbose: false,
        })
    }

    /// Ensure data directory exists
    pub async fn init(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.data_dir).await?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Failed to create default config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new();
        assert!(config.is_ok());
    }
}
