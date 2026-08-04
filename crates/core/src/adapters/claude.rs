use super::{Session, ToolAdapter};
use crate::models::AiTool;
use std::path::Path;

pub struct ClaudeAdapter;

impl ToolAdapter for ClaudeAdapter {
    fn tool(&self) -> AiTool {
        AiTool::ClaudeCode
    }

    fn parse_sessions(&self, _tool_path: &Path) -> crate::Result<Vec<Session>> {
        Ok(Vec::new())
    }

    fn get_config_files(&self, tool_path: &Path) -> crate::Result<Vec<String>> {
        let mut configs = Vec::new();
        let config_path = tool_path.join("config.json");

        if config_path.exists() {
            configs.push(config_path.to_string_lossy().into_owned());
        }

        Ok(configs)
    }
}
