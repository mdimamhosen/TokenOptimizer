use crate::models::*;
use crate::Result;
use chrono::Utc;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::path::Path;
use tracing::{debug, info};
use uuid::Uuid;

/// Database manager
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create database connection
    pub async fn new(db_path: &Path) -> Result<Self> {
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let database_url = if db_path.to_string_lossy().contains("sqlite:") {
            db_path.to_string_lossy().to_string()
        } else {
            format!("sqlite://{}", db_path.display())
        };

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        let db = Database { pool };
        db.migrate().await?;
        Ok(db)
    }

    /// Run migrations
    async fn migrate(&self) -> Result<()> {
        debug!("Running database migrations");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS detected_tools (
                id TEXT PRIMARY KEY,
                tool TEXT NOT NULL UNIQUE,
                installed BOOLEAN NOT NULL,
                path TEXT,
                version TEXT,
                config_path TEXT,
                last_detected TIMESTAMP NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                tool TEXT NOT NULL,
                project TEXT,
                started_at TIMESTAMP NOT NULL,
                ended_at TIMESTAMP,
                tokens_used INTEGER NOT NULL DEFAULT 0,
                estimated_cost REAL NOT NULL DEFAULT 0.0,
                FOREIGN KEY (tool) REFERENCES detected_tools(tool)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS token_usage (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                tool TEXT NOT NULL,
                prompt_tokens INTEGER NOT NULL,
                completion_tokens INTEGER NOT NULL,
                total_tokens INTEGER NOT NULL,
                timestamp TIMESTAMP NOT NULL,
                FOREIGN KEY (session_id) REFERENCES sessions(id),
                FOREIGN KEY (tool) REFERENCES detected_tools(tool)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS project_analysis (
                id TEXT PRIMARY KEY,
                project_path TEXT NOT NULL UNIQUE,
                ai_readiness_score INTEGER NOT NULL DEFAULT 0,
                total_tokens INTEGER NOT NULL DEFAULT 0,
                session_count INTEGER NOT NULL DEFAULT 0,
                last_analyzed TIMESTAMP NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_sessions_tool ON sessions(tool);
            CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project);
            CREATE INDEX IF NOT EXISTS idx_sessions_started_at ON sessions(started_at);
            CREATE INDEX IF NOT EXISTS idx_token_usage_tool ON token_usage(tool);
            CREATE INDEX IF NOT EXISTS idx_token_usage_timestamp ON token_usage(timestamp);
            "#,
        )
        .execute(&self.pool)
        .await?;

        info!("Database migrations completed");
        Ok(())
    }

    // Detected Tools Operations

    pub async fn save_detected_tool(&self, tool: &DetectedTool) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO detected_tools
            (id, tool, installed, path, version, config_path, last_detected)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&tool.id)
        .bind(&tool.tool)
        .bind(tool.installed)
        .bind(&tool.path)
        .bind(&tool.version)
        .bind(&tool.config_path)
        .bind(&tool.last_detected)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_detected_tools(&self) -> Result<Vec<DetectedTool>> {
        let tools = sqlx::query_as::<_, DetectedTool>(
            "SELECT id, tool, installed, path, version, config_path, last_detected FROM detected_tools"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tools)
    }

    pub async fn get_installed_tools(&self) -> Result<Vec<DetectedTool>> {
        let tools = sqlx::query_as::<_, DetectedTool>(
            "SELECT id, tool, installed, path, version, config_path, last_detected FROM detected_tools WHERE installed = true"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tools)
    }

    // Sessions Operations

    pub async fn create_session(&self, tool: &str, project: Option<&str>) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO sessions (id, tool, project, started_at, tokens_used, estimated_cost)
            VALUES (?, ?, ?, ?, 0, 0.0)
            "#,
        )
        .bind(&id)
        .bind(tool)
        .bind(project)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn end_session(&self, session_id: &str, tokens: i64, cost: f64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE sessions
            SET ended_at = ?, tokens_used = ?, estimated_cost = ?
            WHERE id = ?
            "#,
        )
        .bind(Utc::now())
        .bind(tokens)
        .bind(cost)
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_sessions(&self, limit: i64) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as::<_, Session>(
            "SELECT id, tool, project, started_at, ended_at, tokens_used, estimated_cost FROM sessions ORDER BY started_at DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    pub async fn get_sessions_by_tool(&self, tool: &str, limit: i64) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as::<_, Session>(
            "SELECT id, tool, project, started_at, ended_at, tokens_used, estimated_cost FROM sessions WHERE tool = ? ORDER BY started_at DESC LIMIT ?"
        )
        .bind(tool)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    // Token Usage Operations

    pub async fn record_token_usage(
        &self,
        session_id: &str,
        tool: &str,
        prompt_tokens: i64,
        completion_tokens: i64,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let total_tokens = prompt_tokens + completion_tokens;
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO token_usage
            (id, session_id, tool, prompt_tokens, completion_tokens, total_tokens, timestamp)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(session_id)
        .bind(tool)
        .bind(prompt_tokens)
        .bind(completion_tokens)
        .bind(total_tokens)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_total_tokens_by_tool(&self, tool: &str) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COALESCE(SUM(total_tokens), 0) FROM token_usage WHERE tool = ?"
        )
        .bind(tool)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_total_tokens(&self) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COALESCE(SUM(total_tokens), 0) FROM token_usage"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_token_usage_by_tool(&self) -> Result<Vec<(String, i64)>> {
        let results = sqlx::query(
            "SELECT tool, COALESCE(SUM(total_tokens), 0) as total FROM token_usage GROUP BY tool"
        )
        .fetch_all(&self.pool)
        .await?;

        let data = results
            .into_iter()
            .map(|row| (row.get::<String, _>("tool"), row.get::<i64, _>("total")))
            .collect();

        Ok(data)
    }

    // Project Analysis Operations

    pub async fn save_project_analysis(&self, analysis: &ProjectAnalysis) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO project_analysis
            (id, project_path, ai_readiness_score, total_tokens, session_count, last_analyzed)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&analysis.id)
        .bind(&analysis.project_path)
        .bind(analysis.ai_readiness_score)
        .bind(analysis.total_tokens)
        .bind(analysis.session_count)
        .bind(&analysis.last_analyzed)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_project_analysis(&self, project_path: &str) -> Result<Option<ProjectAnalysis>> {
        let analysis = sqlx::query_as::<_, ProjectAnalysis>(
            "SELECT id, project_path, ai_readiness_score, total_tokens, session_count, last_analyzed FROM project_analysis WHERE project_path = ?"
        )
        .bind(project_path)
        .fetch_optional(&self.pool)
        .await?;

        Ok(analysis)
    }

    pub async fn close(&self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    #[cfg(not(target_os = "windows"))]
    async fn test_database_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(&db_path).await;
        assert!(db.is_ok());
    }
}
