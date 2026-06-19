use chrono::{DateTime, Local};
use clap::{Parser, Subcommand, ValueEnum};
use std::{fmt::Display, fs, path::Path};

use anyhow::{Context, Result};

use crate::style::{Colorize, Styled};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    name = "todo",
    about = "A simple CLI todo manager (motils file in Config dir)"
)]
#[command(subcommand_required = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo with a priority (default: low)
    Add {
        /// The task description
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        task: Vec<String>,
        /// The priority (visibility of the task)
        #[arg(
            short = 'p',
            long = "priority",
            value_enum,
            default_value_t = Priority::Low,
            long_help = "\
Priority level of the task.\n\
\t- block       100% chance to be displayed\n\
\t- high        90% chance to be displayed\n\
\t- medium      70% chance to be displayed\n\
\t- low        30% chance to be displayed\n\
if there is no higher priority then the percentage will bump.
"
        )]
        priority: Priority,
    },
    /// List all pending todos, or use `list -d` to list done todos.
    List {
        #[arg(short = 'd', long = "done", default_value_t = false)]
        done: bool,
    },
    /// Mark a todo as done by its number (index)
    Done { num: u8 },
    /// Clear todos that are done(default), or use `clear -a` to remove everything.
    Clear {
        #[arg(short = 'a', long = "all", default_value_t = false)]
        all: bool,
    },
    /// Remove a todo by its number (done or not)
    Rm {
        /// The number (index) of the todo
        num: u8,
    },
}

#[derive(ValueEnum, Serialize, Deserialize, Copy, Clone, Debug, Ord, PartialEq, PartialOrd, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Block,
    High,
    Medium,
    Low,
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block => write!(f, "block"),
            Self::High => write!(f, "high"),
            Self::Medium => write!(f, "medium"),
            Self::Low => write!(f, "low"),
        }
    }
}

impl Priority {
    pub fn icon(&self) -> char {
        match self {
            Self::Block => '󰹆',
            Self::High => '',
            Self::Medium => '󰾅',
            Self::Low => '󰰍',
        }
    }
    pub fn style_message(&self, msg: Styled<String>) -> Styled<String> {
        match self {
            Self::Block => msg.bold().red(),
            Self::High => msg.red(),
            Self::Medium => msg.yellow(),
            Self::Low => msg.cyan(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Todo {
    pub priority: Priority,
    pub description: String,
    pub added: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Done {
    priority: Priority,
    description: String,
    added: DateTime<Local>,
    finished: DateTime<Local>,
}

impl Todo {
    pub fn new(desc: &str, priority: Priority) -> Self {
        Self {
            priority,
            description: desc.to_string(),
            added: chrono::Local::now(),
        }
    }
}

pub fn add_task(tpath: &Path, task: &str, priority: &Priority) -> Result<Todo> {
    let todo = Todo::new(task, *priority);

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(tpath)
        .context("can't open todo file for appending")?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    wtr.serialize(&todo).context("can't write task to file")?;
    wtr.flush()?;

    Ok(todo)
}

pub fn rm_task(tpath: &Path, num: u8) -> Result<Todo> {
    let mut data = {
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .read(true)
            .open(tpath)
            .context("can't read todo file")?;
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file)
            .deserialize()
            .collect::<Result<Vec<Todo>, _>>()
            .context("can't read file content, issue in file")?
    };

    let idx = num.checked_sub(1).context("Task num must be > 0")? as usize;
    if idx >= data.len() {
        anyhow::bail!("Task {} doesn't exist", num)
    }

    let rm = data.remove(idx);

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(tpath)?;
    data.iter()
        .try_for_each(|t| wtr.serialize(t).context("issue while updating the file"))?;
    wtr.flush()?;

    Ok(rm)
}

pub fn mark_done(tpath: &Path, dpath: &Path, num: u8) -> Result<()> {
    let mut data = {
        let file = fs::OpenOptions::new()
            .read(true)
            .open(tpath)
            .context("can't read todo file")?;

        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file)
            .deserialize()
            .collect::<Result<Vec<Todo>, _>>()
            .context("can't parse todo content as csv")?
    };

    let idx = num.checked_sub(1).context("Task num must be > 0")? as usize;
    if idx >= data.len() {
        anyhow::bail!("Task {} doesn't exist", num)
    }

    let task = data.remove(idx);

    let done = Done {
        priority: task.priority,
        description: task.description,
        added: task.added,
        finished: chrono::Local::now(),
    };

    let dfile = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dpath)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(dfile);
    wtr.serialize(done).context("issue marking task as done")?;
    wtr.flush()?;

    let temp_path = tpath.with_extension("tmp");
    {
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_path)
            .context("can't open temp file for writing")?;

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);

        for t in data {
            wtr.serialize(t).context("issue while updating the file")?;
        }
        wtr.flush()?;
    }

    fs::rename(&temp_path, tpath).context("failed to replace original todo file")?;

    Ok(())
}

pub fn read_todos(tpath: &Path) -> Result<Vec<Todo>> {
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .read(true)
        .open(tpath)
        .context("can't read todo file")?;

    let out = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file)
        .deserialize()
        .collect::<Result<Vec<Todo>, _>>()
        .context("can't read todo file content, issue in file")?;
    Ok(out)
}

pub fn print_list_tasks(tpath: &Path, dpath: &Path, done: bool) -> Result<()> {
    if done {
        let file = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .open(dpath)
            .context("can't read done tasks file")?;
        let dones = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file)
            .deserialize()
            .collect::<Result<Vec<Done>, _>>()
            .context("can't read done file content, issue in file")?;

        for done in &dones {
            let duration = done
                .finished
                .signed_duration_since(done.added)
                .to_std()
                .context("duration should be positive and fit")?;
            let message = format!(
                "  {} {} => 󱍧 in ({})",
                done.priority.icon(),
                done.description,
                humantime::format_duration(duration)
            )
            .styled()
            .magenta();
            println!("{message}");
        }
    } else {
        read_todos(tpath)
            .context("can't read todo file content, issue in file")?
            .iter()
            .enumerate()
            .for_each(|(idx, todo)| {
                let message = format!(
                    "  {} [{}] {} ({})",
                    todo.priority.icon(),
                    idx + 1,
                    todo.description,
                    todo.added.format("%m-%d %H:%M")
                )
                .styled();
                let message = todo.priority.style_message(message);
                println!("{message}");
            })
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup() -> (TempDir, PathBuf, PathBuf) {
        let dir = TempDir::new().unwrap();
        let tpath = dir.path().join("todo");
        let dpath = dir.path().join("done");
        (dir, tpath, dpath)
    }

    #[test]
    fn test_add_an_read() {
        let (_dir, tpath, _) = setup();

        add_task(&tpath, "Write unit test", &Priority::High).unwrap();
        add_task(&tpath, "Clean up", &Priority::Medium).unwrap();

        let todos = read_todos(&tpath).unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos.first().unwrap().description, "Write unit test");
        assert_eq!(todos.first().unwrap().priority, Priority::High);
        assert_eq!(todos.get(1).unwrap().description, "Clean up");
        assert_eq!(todos.get(1).unwrap().priority, Priority::Medium);
    }

    #[test]
    fn test_rm_task() {
        let (_dir, tpath, _) = setup();
        add_task(&tpath, "Task 1", &Priority::Block).unwrap();
        add_task(&tpath, "Task 2", &Priority::Medium).unwrap();
        add_task(&tpath, "Task 3", &Priority::Low).unwrap();

        let removed = rm_task(&tpath, 2).unwrap();
        assert_eq!(removed.description, "Task 2");

        let todos = read_todos(&tpath).unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].description, "Task 1");
        assert_eq!(todos[1].description, "Task 3");
    }

    #[test]
    fn test_rm_task_out_of_bounds() {
        let (_dir, tpath, _) = setup();
        add_task(&tpath, "Task 1", &Priority::Low).unwrap();

        let res = rm_task(&tpath, 5);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("doesn't exist"));
    }

    #[test]
    fn test_mark_done() {
        let (_dir, tpath, dpath) = setup();

        add_task(&tpath, "Task A", &Priority::Block).unwrap();
        add_task(&tpath, "Task B", &Priority::Low).unwrap();

        mark_done(&tpath, &dpath, 1).unwrap();

        let todos = read_todos(&tpath).unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].description, "Task B");

        let dfile = fs::OpenOptions::new().read(true).open(&dpath).unwrap();
        let dones: Vec<Done> = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(dfile)
            .deserialize()
            .map(|res| res.unwrap())
            .collect();

        assert_eq!(dones.len(), 1);
        assert_eq!(dones[0].description, "Task A");
        assert_eq!(dones[0].priority, Priority::Block);

        assert!(dones[0].finished > dones[0].added);
    }

    #[test]
    fn test_mark_done_fails_safely() {
        let (_dir, tpath, dpath) = setup();
        add_task(&tpath, "Task to lose", &Priority::High).unwrap();

        fs::create_dir(&dpath).unwrap();

        // This should fail because dpath is a directory
        let result = mark_done(&tpath, &dpath, 1);
        assert!(result.is_err());

        // The task must STILL be in the todo file
        let todos = read_todos(&tpath).unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].description, "Task to lose");
    }
}
