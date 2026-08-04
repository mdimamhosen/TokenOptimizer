use crate::models::{AiTool, ToolDetectionResult};
use crate::Result;
use std::path::PathBuf;
use tracing::{debug, info};

/// AI tool detector
pub struct ToolDetector {
    home_dir: PathBuf,
}

impl ToolDetector {
    pub fn new(home_dir: PathBuf) -> Self {
        ToolDetector { home_dir }
    }

    /// Detect all installed AI tools
    pub async fn detect_all(&self) -> Result<Vec<ToolDetectionResult>> {
        let mut results = Vec::new();

        let tools = vec![
            (AiTool::ClaudeCode, "claude-code"),
            (AiTool::Cursor, "cursor"),
            (AiTool::GitHubCopilot, "github-copilot"),
            (AiTool::Windsurf, "windsurf"),
            (AiTool::Cline, "cline"),
            (AiTool::Continue, "continue"),
            (AiTool::Aider, "aider"),
            (AiTool::OpenAiCodex, "openai-codex"),
        ];

        for (tool, _name) in tools {
            if let Ok(result) = self.detect_tool(tool).await {
                results.push(result);
            }
        }

        info!("Detected {} AI tools", results.len());
        Ok(results)
    }

    /// Detect specific tool
    pub async fn detect_tool(&self, tool: AiTool) -> Result<ToolDetectionResult> {
        let (path, version, config_path) = match tool {
            AiTool::ClaudeCode => self.detect_claude_code().await?,
            AiTool::Cursor => self.detect_cursor().await?,
            AiTool::GitHubCopilot => self.detect_github_copilot().await?,
            AiTool::Windsurf => self.detect_windsurf().await?,
            AiTool::Cline => self.detect_cline().await?,
            AiTool::Continue => self.detect_continue().await?,
            AiTool::Aider => self.detect_aider().await?,
            AiTool::OpenAiCodex => self.detect_openai_codex().await?,
            AiTool::Generic => (None, None, None),
        };

        let installed = path.is_some();
        debug!(
            "Tool detection: {} - installed: {}",
            tool.display_name(),
            installed
        );

        Ok(ToolDetectionResult {
            tool,
            installed,
            path,
            version,
            config_path,
        })
    }

    async fn detect_claude_code(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        #[cfg(target_os = "windows")]
        {
            let app_data = self.home_dir.join(".claude");
            if app_data.exists() {
                return Ok((Some(app_data.to_string_lossy().into_owned()), None, Some(app_data.to_string_lossy().into_owned())));
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let app_dir = self.home_dir.join(".claude");
            if app_dir.exists() {
                return Ok((Some(app_dir.to_string_lossy().into_owned()), None, Some(app_dir.to_string_lossy().into_owned())));
            }
        }

        Ok((None, None, None))
    }

    async fn detect_cursor(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        #[cfg(target_os = "windows")]
        {
            let cursor_dir = self.home_dir.join("AppData/Local/cursor");
            if cursor_dir.exists() {
                let config_path = self.home_dir.join(".cursor");
                return Ok((
                    Some(cursor_dir.to_string_lossy().into_owned()),
                    None,
                    Some(config_path.to_string_lossy().into_owned()),
                ));
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let cursor_dir = self.home_dir.join(".cursor");
            if cursor_dir.exists() {
                return Ok((
                    Some(cursor_dir.to_string_lossy().into_owned()),
                    None,
                    Some(cursor_dir.to_string_lossy().into_owned()),
                ));
            }
        }

        Ok((None, None, None))
    }

    async fn detect_github_copilot(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        #[cfg(target_os = "windows")]
        {
            let copilot_dir = self.home_dir.join("AppData/Local/GitHub Copilot");
            if copilot_dir.exists() {
                return Ok((Some(copilot_dir.to_string_lossy().into_owned()), None, None));
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let copilot_dir = self.home_dir.join(".github-copilot");
            if copilot_dir.exists() {
                return Ok((Some(copilot_dir.to_string_lossy().into_owned()), None, None));
            }
        }

        Ok((None, None, None))
    }

    async fn detect_windsurf(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        #[cfg(target_os = "windows")]
        {
            let windsurf_dir = self.home_dir.join("AppData/Local/windsurf");
            if windsurf_dir.exists() {
                return Ok((Some(windsurf_dir.to_string_lossy().into_owned()), None, None));
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let windsurf_dir = self.home_dir.join(".windsurf");
            if windsurf_dir.exists() {
                return Ok((Some(windsurf_dir.to_string_lossy().into_owned()), None, None));
            }
        }

        Ok((None, None, None))
    }

    async fn detect_cline(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        let cline_dir = self.home_dir.join(".cline");
        if cline_dir.exists() {
            return Ok((
                Some(cline_dir.to_string_lossy().into_owned()),
                None,
                Some(cline_dir.to_string_lossy().into_owned()),
            ));
        }
        Ok((None, None, None))
    }

    async fn detect_continue(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        let continue_dir = self.home_dir.join(".continue");
        if continue_dir.exists() {
            return Ok((
                Some(continue_dir.to_string_lossy().into_owned()),
                None,
                Some(continue_dir.to_string_lossy().into_owned()),
            ));
        }
        Ok((None, None, None))
    }

    async fn detect_aider(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        let aider_dir = self.home_dir.join(".aider");
        if aider_dir.exists() {
            return Ok((
                Some(aider_dir.to_string_lossy().into_owned()),
                None,
                Some(aider_dir.to_string_lossy().into_owned()),
            ));
        }
        Ok((None, None, None))
    }

    async fn detect_openai_codex(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
        let codex_dir = self.home_dir.join(".openai");
        if codex_dir.exists() {
            return Ok((
                Some(codex_dir.to_string_lossy().into_owned()),
                None,
                Some(codex_dir.to_string_lossy().into_owned()),
            ));
        }
        Ok((None, None, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detector_creation() {
        let detector = ToolDetector::new(PathBuf::from("/tmp"));
        assert_eq!(detector.home_dir, PathBuf::from("/tmp"));
    }
}
