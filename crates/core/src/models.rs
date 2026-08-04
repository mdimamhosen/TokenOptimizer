use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Supported AI coding assistants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AiTool {
    ClaudeCode,
    Cursor,
    GitHubCopilot,
    Windsurf,
    Cline,
    Continue,
    Aider,
    OpenAiCodex,
    Generic,
}

impl AiTool {
    pub fn as_str(&self) -> &'static str {
        match self {
            AiTool::ClaudeCode => "claude_code",
            AiTool::Cursor => "cursor",
            AiTool::GitHubCopilot => "github_copilot",
            AiTool::Windsurf => "windsurf",
            AiTool::Cline => "cline",
            AiTool::Continue => "continue",
            AiTool::Aider => "aider",
            AiTool::OpenAiCodex => "openai_codex",
            AiTool::Generic => "generic",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "claude_code" => Some(AiTool::ClaudeCode),
            "cursor" => Some(AiTool::Cursor),
            "github_copilot" => Some(AiTool::GitHubCopilot),
            "windsurf" => Some(AiTool::Windsurf),
            "cline" => Some(AiTool::Cline),
            "continue" => Some(AiTool::Continue),
            "aider" => Some(AiTool::Aider),
            "openai_codex" => Some(AiTool::OpenAiCodex),
            "generic" => Some(AiTool::Generic),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AiTool::ClaudeCode => "Claude Code",
            AiTool::Cursor => "Cursor",
            AiTool::GitHubCopilot => "GitHub Copilot",
            AiTool::Windsurf => "Windsurf",
            AiTool::Cline => "Cline",
            AiTool::Continue => "Continue",
            AiTool::Aider => "Aider",
            AiTool::OpenAiCodex => "OpenAI Codex",
            AiTool::Generic => "Generic AI Tool",
        }
    }
}

/// Detected AI tool information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DetectedTool {
    pub id: String,
    pub tool: String,
    pub installed: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub config_path: Option<String>,
    pub last_detected: DateTime<Utc>,
}

/// Usage session record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub tool: String,
    pub project: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub tokens_used: i64,
    pub estimated_cost: f64,
}

/// Token usage entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TokenUsage {
    pub id: String,
    pub session_id: String,
    pub tool: String,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
    pub timestamp: DateTime<Utc>,
}

/// Project analysis
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectAnalysis {
    pub id: String,
    pub project_path: String,
    pub ai_readiness_score: i32,
    pub total_tokens: i64,
    pub session_count: i32,
    pub last_analyzed: DateTime<Utc>,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub category: String,
    pub issue: String,
    pub suggestion: String,
    pub estimated_savings: f64,
    pub priority: String,
}

/// Daily usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub tool: String,
    pub total_tokens: i64,
    pub session_count: i32,
    pub estimated_cost: f64,
}

/// Tool detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDetectionResult {
    pub tool: AiTool,
    pub installed: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub config_path: Option<String>,
}

/// Prompt analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptAnalysis {
    pub prompt: String,
    pub estimated_tokens: i64,
    pub efficiency_score: f64,
    pub suggestions: Vec<String>,
    pub estimated_savings: i64,
}

/// Real-time monitoring state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringState {
    pub active_tool: Option<String>,
    pub active_project: Option<String>,
    pub session_duration_secs: u64,
    pub current_tokens: i64,
    pub current_cost: f64,
}
