use ai_assist_core::{database::Database, detection::ToolDetector, Config};
use clap::{Parser, Subcommand};
use colored::*;
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "AI Assist")]
#[command(about = "Universal analytics platform for AI coding assistants", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize AI Assist
    Init,

    /// Scan system for AI tools
    Scan,

    /// Show token usage statistics
    Usage {
        #[arg(short, long)]
        tool: Option<String>,
        #[arg(short, long)]
        limit: Option<i64>,
    },

    /// Open dashboard
    Dashboard,

    /// Analyze project for optimization
    Analyze {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Watch real-time usage
    Watch {
        #[arg(short, long)]
        interval: Option<u64>,
    },

    /// Generate detailed report
    Report {
        #[arg(short, long)]
        format: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_level(true)
        .compact()
        .init();

    let config = Config::new()?;
    config.init().await?;

    match cli.command {
        Commands::Init => cmd_init(&config).await?,
        Commands::Scan => cmd_scan(&config).await?,
        Commands::Usage { tool, limit } => cmd_usage(&config, tool, limit).await?,
        Commands::Dashboard => cmd_dashboard(&config).await?,
        Commands::Analyze { path } => cmd_analyze(&config, &path).await?,
        Commands::Watch { interval } => cmd_watch(&config, interval).await?,
        Commands::Report { format } => cmd_report(&config, format).await?,
    }

    Ok(())
}

async fn cmd_init(config: &Config) -> anyhow::Result<()> {
    println!("{}", "Initializing AI Assist...".bold().cyan());

    let detector = ToolDetector::new(config.home.clone());
    let tools = detector.detect_all().await?;

    println!(
        "\n{} AI Tools Detected:\n",
        tools.len().to_string().green().bold()
    );

    for tool in &tools {
        let status = if tool.installed {
            "✓".green()
        } else {
            "✗".red()
        };

        println!("  {} {}", status, tool.tool.display_name().bold());
        if let Some(path) = &tool.path {
            println!("     Path: {}", path.bright_black());
        }
    }

    println!("\n{}", "Database initialized at:".cyan());
    println!("  {}", config.db_path.display().to_string().bright_black());

    Ok(())
}

async fn cmd_scan(config: &Config) -> anyhow::Result<()> {
    println!("{}", "Scanning for AI tools...".bold().cyan());

    let detector = ToolDetector::new(config.home.clone());
    let results = detector.detect_all().await?;

    let db = Database::new(&config.db_path).await?;

    for result in &results {
        let tool = ai_assist_core::models::DetectedTool {
            id: uuid::Uuid::new_v4().to_string(),
            tool: result.tool.as_str().to_string(),
            installed: result.installed,
            path: result.path.clone(),
            version: result.version.clone(),
            config_path: result.config_path.clone(),
            last_detected: chrono::Utc::now(),
        };

        db.save_detected_tool(&tool).await?;

        let status = if result.installed {
            "✓".green()
        } else {
            "✗".red()
        };
        println!("  {} {}", status, result.tool.display_name().bold());
    }

    db.close().await?;
    println!("\n{}", "Scan complete!".green().bold());

    Ok(())
}

async fn cmd_usage(
    config: &Config,
    _tool: Option<String>,
    _limit: Option<i64>,
) -> anyhow::Result<()> {
    let db = Database::new(&config.db_path).await?;

    println!("{}", "Token Usage Statistics".bold().cyan());
    println!();

    let total_tokens = db.get_total_tokens().await?;
    println!("  {} tokens total", total_tokens.to_string().green().bold());

    println!("\n{} Usage by Tool:", "Current".cyan().bold());

    let by_tool = db.get_token_usage_by_tool().await?;
    for (tool_name, tokens) in by_tool {
        println!(
            "  {} {}",
            tool_name.bold(),
            format!("{} tokens", tokens).green()
        );
    }

    db.close().await?;
    Ok(())
}

async fn cmd_dashboard(_config: &Config) -> anyhow::Result<()> {
    println!("{}", "Opening Dashboard...".bold().cyan());
    println!("{}", "Dashboard feature coming soon!".yellow());
    Ok(())
}

async fn cmd_analyze(_config: &Config, path: &Path) -> anyhow::Result<()> {
    println!(
        "{} {}",
        "Analyzing project:".cyan(),
        path.display().to_string().bold()
    );

    use ai_assist_core::optimization::OptimizationEngine;

    let score = OptimizationEngine::calculate_readiness_score(path);
    let recommendations = OptimizationEngine::analyze_project(path);

    println!(
        "\n{} {}/100",
        "AI Readiness Score:".bold(),
        score.to_string().green().bold()
    );

    if !recommendations.is_empty() {
        println!("\n{} Recommendations:\n", "Found".yellow().bold());

        for (i, rec) in recommendations.iter().enumerate() {
            let priority_color = match rec.priority.as_str() {
                "high" => rec.priority.as_str().red(),
                "medium" => rec.priority.as_str().yellow(),
                _ => rec.priority.as_str().blue(),
            };

            println!("{}. {} [{}]", i + 1, rec.issue.bold(), priority_color);
            println!("   Suggestion: {}", rec.suggestion.bright_black());
            println!(
                "   Potential savings: {}%\n",
                rec.estimated_savings.to_string().green()
            );
        }
    } else {
        println!("\n{}", "No issues found!".green().bold());
    }

    Ok(())
}

async fn cmd_watch(_config: &Config, _interval: Option<u64>) -> anyhow::Result<()> {
    println!("{}", "Starting real-time monitoring...".bold().cyan());
    println!("{}", "Watch feature coming soon!".yellow());
    Ok(())
}

async fn cmd_report(_config: &Config, _format: Option<String>) -> anyhow::Result<()> {
    println!("{}", "Generating report...".bold().cyan());
    println!("{}", "Report feature coming soon!".yellow());
    Ok(())
}
