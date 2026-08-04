/// Data extractors for different AI tools
use std::path::Path;

pub struct DataExtractor;

impl DataExtractor {
    pub fn extract_usage_from_tool(_tool_path: &Path) -> anyhow::Result<()> {
        Ok(())
    }
}
