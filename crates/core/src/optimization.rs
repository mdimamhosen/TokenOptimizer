use crate::models::Recommendation;
use std::path::Path;
use walkdir::WalkDir;

/// Optimization engine for analyzing workflows
pub struct OptimizationEngine;

impl OptimizationEngine {
    /// Analyze project for optimization opportunities
    pub fn analyze_project(project_path: &Path) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        recommendations.extend(Self::check_large_directories(project_path));
        recommendations.extend(Self::check_configuration_files(project_path));
        recommendations.extend(Self::check_context_files(project_path));

        recommendations
    }

    /// Check for large directories that shouldn't be indexed
    fn check_large_directories(project_path: &Path) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        let mut problematic_dirs = Vec::new();

        let excluded = vec![
            "node_modules",
            "venv",
            ".venv",
            "__pycache__",
            ".git",
            "build",
            "dist",
            "target",
            ".next",
            "out",
        ];

        for entry in WalkDir::new(project_path)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if excluded.iter().any(|e| e == &name_str.as_ref()) {
                        problematic_dirs.push(name_str.into_owned());
                    }
                }
            }
        }

        if !problematic_dirs.is_empty() {
            let dirs = problematic_dirs.join(", ");
            recommendations.push(Recommendation {
                id: uuid::Uuid::new_v4().to_string(),
                category: "large_directories".to_string(),
                issue: format!(
                    "Found large directories that shouldn't be indexed: {}",
                    dirs
                ),
                suggestion: format!("Exclude these directories from AI tool context: {}", dirs),
                estimated_savings: 40.0,
                priority: "high".to_string(),
            });
        }

        recommendations
    }

    /// Check configuration files for issues
    fn check_configuration_files(project_path: &Path) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        let config_files = vec![
            ("CLAUDE.md", "Claude Code"),
            (".cursorrules", "Cursor"),
            (".cline", "Cline"),
            ("continue.config.ts", "Continue"),
        ];

        for (file, tool) in config_files {
            let file_path = project_path.join(file);
            if file_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    if content.len() > 10000 {
                        recommendations.push(Recommendation {
                            id: uuid::Uuid::new_v4().to_string(),
                            category: "config_size".to_string(),
                            issue: format!(
                                "{} configuration file is too large ({} bytes)",
                                tool,
                                content.len()
                            ),
                            suggestion: format!(
                                "Split {} into smaller, focused files. Keep it under 5KB.",
                                file
                            ),
                            estimated_savings: 25.0,
                            priority: "medium".to_string(),
                        });
                    }
                }
            }
        }

        recommendations
    }

    /// Check context and instruction files
    fn check_context_files(project_path: &Path) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        let context_files = vec!["README.md", "ARCHITECTURE.md", "docs/context.md"];

        for file in context_files {
            let file_path = project_path.join(file);
            if file_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    if content.contains("TODO") || content.contains("FIXME") {
                        recommendations.push(Recommendation {
                            id: uuid::Uuid::new_v4().to_string(),
                            category: "outdated_docs".to_string(),
                            issue: "Documentation contains TODO/FIXME markers".to_string(),
                            suggestion:
                                "Keep documentation up-to-date. Remove or resolve TODO markers."
                                    .to_string(),
                            estimated_savings: 15.0,
                            priority: "low".to_string(),
                        });
                        break;
                    }
                }
            }
        }

        recommendations
    }

    /// Generate AI readiness score
    pub fn calculate_readiness_score(project_path: &Path) -> i32 {
        let mut score = 100;
        let recommendations = Self::analyze_project(project_path);

        for rec in recommendations {
            score -= match rec.priority.as_str() {
                "high" => 10,
                "medium" => 5,
                "low" => 2,
                _ => 0,
            };
        }

        score.clamp(0, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_readiness_score() {
        let temp_dir = TempDir::new().unwrap();
        let score = OptimizationEngine::calculate_readiness_score(temp_dir.path());
        assert!((0..=100).contains(&score));
    }
}
