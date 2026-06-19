use clap::Parser;
use std::fs;

use anyhow::{Context, Result};

use motils::style::Colorize;
use motils::todo::{self, Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app_dir = dirs::config_dir()
        .context("failed to get config directory")?
        .join("motils");

    fs::create_dir_all(&app_dir)?;

    let tpath = app_dir.join("todo");
    let dpath = app_dir.join("done");

    match &cli.command {
        Commands::Add { task, priority } => {
            let todo = todo::add_task(&tpath, &task.join(" "), priority)?;
            let message = format!(
                "Added: {} {} ({})",
                todo.priority.icon(),
                todo.description,
                todo.added.format("%m-%d %H:%M")
            )
            .styled();
            let message = todo.priority.style_message(message);
            println!("{message}");
        }
        Commands::List { done } => todo::print_list_tasks(&tpath, &dpath, *done)?,

        Commands::Rm { num } => {
            let removed = todo::rm_task(&tpath, *num)?;
            println!("Removed: {}", removed.description);
        }
        Commands::Done { num } => {
            todo::mark_done(&tpath, &dpath, *num)?;
            println!("Marked as done");
        }

        Commands::Clear { all } => {
            if *all {
                fs::remove_file(tpath).ok();
                fs::remove_file(dpath).ok();
            } else {
                fs::remove_file(dpath).ok();
            }
            println!("Cleared")
        }
    }
    Ok(())
}
