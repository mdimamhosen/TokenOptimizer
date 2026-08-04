use super::{Session, ToolAdapter};
use crate::models::AiTool;
use std::path::Path;

pub struct CursorAdapter;

impl ToolAdapter for CursorAdapter {
    fn tool(&self) -> AiTool {
        AiTool::Cursor
    }

    fn parse_sessions(&self, _tool_path: &Path) -> crate::Result<Vec<Session>> {
        Ok(Vec::new())
    }

    fn get_config_files(&self, tool_path: &Path) -> crate::Result<Vec<String>> {
        let mut configs = Vec::new();
        let config_path = tool_path.join("settings.json");

        if config_path.exists() {
            configs.push(config_path.to_string_lossy().into_owned());
        }

        Ok(configs)
    }
}
