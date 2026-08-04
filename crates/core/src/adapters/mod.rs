/// Adapters for different AI tools
/// Each adapter implements unified interface for tool-specific data
pub mod claude;
pub mod cursor;

use crate::models::AiTool;
use std::path::Path;

pub trait ToolAdapter {
    /// Get the tool type
    fn tool(&self) -> AiTool;

    /// Parse session data from tool
    fn parse_sessions(&self, tool_path: &Path) -> crate::Result<Vec<Session>>;

    /// Get configuration files
    fn get_config_files(&self, tool_path: &Path) -> crate::Result<Vec<String>>;
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub project: Option<String>,
    pub tokens: i64,
    pub cost: f64,
}
