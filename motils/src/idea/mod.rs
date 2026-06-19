use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use std::{fs, io, io::Write, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::style::Colorize;

#[derive(Parser)]
#[command(
    name = "idea",
    about = "A simple CLI idea manager (motifs file in Config dir)"
)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// List all ideas
    #[arg(short = 'l', long = "list")]
    pub list: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new idea (will prompt for title and description)
    Add,
    /// Show the detail of an idea by its number
    Detail {
        /// The number (index) of the idea
        num: u8,
    },
    /// Remove an idea by its number
    Rm {
        /// The number (index) of the idea
        num: u8,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Idea {
    pub title: String,
    pub description: String,
    pub added: DateTime<Local>,
}

impl Idea {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            added: Local::now(),
        }
    }
}

fn prompt(msg: &str) -> Result<String> {
    print!("{}: ", msg);
    io::stdout().flush().context("failed to flush stdout")?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("failed to read from stdin")?;
    Ok(input.trim().to_string())
}

pub fn add_idea(ipath: &Path) -> Result<Idea> {
    let title = prompt("Title")?;
    if title.is_empty() {
        anyhow::bail!("Title cannot be empty");
    }

    let description = prompt("Description")?;

    let idea = Idea::new(&title, &description);

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(ipath)
        .context("can't open idea file for appending")?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    wtr.serialize(&idea).context("can't write idea to file")?;
    wtr.flush()?;

    Ok(idea)
}

pub fn rm_idea(ipath: &Path, num: u8) -> Result<Idea> {
    let mut data = {
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .read(true)
            .open(ipath)
            .context("can't read idea file")?;
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file)
            .deserialize()
            .collect::<Result<Vec<Idea>, _>>()
            .context("can't read file content, issue in file")?
    };

    let idx = num.checked_sub(1).context("Idea num must be > 0")? as usize;
    if idx >= data.len() {
        anyhow::bail!("Idea {} doesn't exist", num)
    }

    let rm = data.remove(idx);

    // Rewrite the file without the removed idea
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(ipath)?;
    data.iter()
        .try_for_each(|i| wtr.serialize(i).context("issue while updating the file"))?;
    wtr.flush()?;

    Ok(rm)
}

pub fn read_ideas(ipath: &Path) -> Result<Vec<Idea>> {
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .read(true)
        .open(ipath)
        .context("can't read idea file")?;

    let out = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file)
        .deserialize()
        .collect::<Result<Vec<Idea>, _>>()
        .context("can't read idea file content, issue in file")?;
    Ok(out)
}

pub fn print_list_ideas(ipath: &Path) -> Result<()> {
    let ideas = read_ideas(ipath)?;

    if ideas.is_empty() {
        println!("No ideas yet. Add one with `idea add`.");
        return Ok(());
    }

    ideas.iter().enumerate().for_each(|(idx, idea)| {
        let message = format!("  󰍩 [{}] {}", idx + 1, idea.title)
            .styled()
            .magenta();
        println!("{message}");
    });
    Ok(())
}

pub fn print_detail_idea(ipath: &Path, num: u8) -> Result<()> {
    let ideas = read_ideas(ipath).context("can't read idea file content, issue in file")?;
    let idx = num.checked_sub(1).context("Idea num must be > 0")? as usize;
    if idx >= ideas.len() {
        anyhow::bail!("Idea {} doesn't exist", num);
    }
    let idea = &ideas[idx];

    // Detail view shows Title (bold) + Description + Date
    let title_msg = format!("  󰍩 [{}] {}", num, idea.title)
        .styled()
        .magenta()
        .bold();
    println!("{title_msg}");

    let desc_msg = format!("      {}", idea.description).styled().magenta();
    println!("{desc_msg}");

    let date_msg = format!("      (Added: {})", idea.added.format("%Y-%m-%d %H:%M"))
        .styled()
        .magenta(); // Kept magenta to strictly follow "all ideas should be magenta"
    println!("{date_msg}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let ipath = dir.path().join("ideas");
        (dir, ipath)
    }

    // Helper to add an idea directly without triggering interactive stdin prompts
    fn add_idea_direct(ipath: &Path, title: &str, desc: &str) -> Result<Idea> {
        let idea = Idea::new(title, desc);

        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(ipath)
            .context("can't open idea file for appending")?;

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);
        wtr.serialize(&idea).context("can't write idea to file")?;
        wtr.flush()?;

        Ok(idea)
    }

    #[test]
    fn test_add_and_read() {
        let (_dir, ipath) = setup();

        add_idea_direct(&ipath, "Idea 1", "Description 1").unwrap();
        add_idea_direct(&ipath, "Idea 2", "Description 2").unwrap();

        let ideas = read_ideas(&ipath).unwrap();
        assert_eq!(ideas.len(), 2);
        assert_eq!(ideas[0].title, "Idea 1");
        assert_eq!(ideas[1].title, "Idea 2");
    }

    #[test]
    fn test_rm_idea() {
        let (_dir, ipath) = setup();
        add_idea_direct(&ipath, "Idea A", "Desc A").unwrap();
        add_idea_direct(&ipath, "Idea B", "Desc B").unwrap();
        add_idea_direct(&ipath, "Idea C", "Desc C").unwrap();

        let removed = rm_idea(&ipath, 2).unwrap();
        assert_eq!(removed.title, "Idea B");

        let ideas = read_ideas(&ipath).unwrap();
        assert_eq!(ideas.len(), 2);
        assert_eq!(ideas[0].title, "Idea A");
        assert_eq!(ideas[1].title, "Idea C");
    }

    #[test]
    fn test_rm_idea_out_of_bounds() {
        let (_dir, ipath) = setup();
        add_idea_direct(&ipath, "Idea 1", "Desc 1").unwrap();

        let res = rm_idea(&ipath, 5);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("doesn't exist"));
    }
}
