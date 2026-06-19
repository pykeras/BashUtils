use clap::Parser;
use std::fs;

use anyhow::{Context, Result};

use motils::idea::{self, Cli, Commands};
use motils::style::Colorize;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let app_dir = dirs::config_dir()
        .context("failed to get config directory")?
        .join("motils");

    fs::create_dir_all(&app_dir)?;

    let ipath = app_dir.join("ideas");

    if cli.list {
        return idea::print_list_ideas(&ipath);
    }

    match &cli.command {
        Some(Commands::Add) => {
            let idea = idea::add_idea(&ipath)?;
            let message = format!("Added idea: 󰍩 {}", idea.title).styled().green();
            println!("{message}");
        }
        Some(Commands::Detail { num }) => {
            idea::print_detail_idea(&ipath, *num)?;
        }
        Some(Commands::Rm { num }) => {
            let removed = idea::rm_idea(&ipath, *num)?;
            let message = format!("Removed idea: 󰍩 {}", removed.title).styled().cyan();
            println!("{message}");
        }
        None => {
            // SAFETY: because of `arg_required_else_help=true` on the Cli struct
            unreachable!()
        }
    }

    Ok(())
}
