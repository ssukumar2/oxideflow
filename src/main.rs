//! oxideflow — log file analyzer CLI.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod parser;
mod filter;
pub mod stats;
pub mod timefilter;

#[derive(Parser)]
#[command(name = "oxideflow")]
#[command(about = "Log file analyzer CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Filter a log file by level, regex, or both
    Filter {
        /// Path to the log file
        #[arg(short, long)]
        file: PathBuf,

        /// Filter by log level (INFO, WARN, ERROR, DEBUG)
        #[arg(short, long)]
        level: Option<String>,

        /// Regex pattern to match against log messages
        #[arg(short, long)]
        pattern: Option<String>,

         /// Filter lines containing this time prefix (e.g. "2026-04-16 10:00")
        #[arg(short = 't', long)]
        since: Option<String>,


        /// Output as JSON instead of plain text
        #[arg(short, long)]
        json: bool,
    },

    /// Print summary statistics about a log file
    Stats {
        /// Path to the log file
        #[arg(short, long)]
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Filter { file, level, pattern, since, json } => {
            let lines = parser::read_file(&file)
                .with_context(|| format!("failed to read {}", file.display()))?;

            let mut filtered = filter::apply(&lines, level.as_deref(), pattern.as_deref())
                .context("filter failed")?;
            
            if let Some(ref prefix) = since {
                filtered = filter::filter_by_time_prefix(&filtered, prefix);
            }

            if json {
                let as_json = serde_json::to_string_pretty(&filtered)
                    .context("failed to serialize to json")?;
                println!("{}", as_json);
            } else {
                for line in &filtered {
                    println!("{}", line);
                }
            }

            eprintln!("matched {}/{} lines", filtered.len(), lines.len());
        }

        Commands::Stats { file } => {
            let lines = parser::read_file(&file)
                .with_context(|| format!("failed to read {}", file.display()))?;

            let stats = filter::summarize(&lines);
            let out = serde_json::to_string_pretty(&stats)?;
            println!("{}", out);
        }
    }

    Ok(())
}