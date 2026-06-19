# Motils - Greeting for terminal

### Todo Manager

- **todo**: _a simple CLI todo manager with priority-based task visibility_

- _Manage your daily tasks with style. Each task has a priority level that determines its visibility and display color._

- _Tasks are stored in CSV format in your config directory (`~/.config/motils/todo`) making them easy to backup and sync._

**USAGE:**

- _add a new task:_

```
$ todo add Write documentation
  ----OR----
# with priority
$ todo add -p high Fix critical bug
  ----OR----
# with trailing arguments
$ todo add -p medium Fix -- this bug now
```

- _list tasks:_

```
# list pending tasks
$ todo list
  ----OR----
# list completed tasks
$ todo list -d
  ----OR----
$ todo list --done
```

- _mark task as done:_

```
$ todo done 1
```

- _remove a task:_

```
$ todo rm 2
```

- _clear tasks:_

```
# clear completed tasks
$ todo clear
  ----OR----
# clear everything
$ todo clear -a
```

> `-p, --priority:` _Priority level: `block` (100%), `high` (90%), `medium` (70%), `low` (30%). Default: `low`_
>
> `-d, --done:` _Show completed tasks instead of pending ones_
>
> `-a, --all:` _Clear all tasks (both pending and completed)_

**Priority Icons:** _You must use NerdFont variant for this one_

_`Test environment`: `OS Linux`, `Rust 1.90+`_

### Idea Manager

- **idea**: _a simple CLI idea manager to capture and organize your thoughts as fast as possible_

- _Capture ideas with titles and detailed descriptions. Never lose a brilliant thought again._

- _Ideas are stored in CSV format in your config directory (`~/.config/motils/ideas`) with timestamps._

**USAGE:**

- _list all ideas:_

```
$ idea -l
  ----OR----
$ idea --list
```

- _add a new idea:_

```
$ idea add
# Interactive prompts:
# Title: My brilliant idea
# Description: This is the detailed description...
```

- _view idea details:_

```
$ idea detail 1
```

- _remove an idea:_

```
$ idea rm 2
```

> All ideas are displayed in **magenta** color with the `󰍩` icon for easy identification.

_`Test environment`: `OS Linux`, `Rust 1.90+`_

### System Dashboard

- **greet**: _a cli system information dashboard that displays your system stats at a glance_

- _Get a quick overview of your system: hostname, uptime, RAM usage, CPU usage, disk space, network interfaces, pending todos and ideas._

- _Runs asynchronously for fast performance - all system info is gathered in parallel._

**USAGE:**

```
$ greet
```

**Displays:**

- **System Info:**
  - Hostname
  - Uptime
  - RAM usage
  - CPU usage (color-coded: green < 50%, yellow < 80%, red >= 80%)
  - Disk usage

- **Network:**
  - Interface names with IP addresses

- **Todos:**
  - Quick view of pending tasks from todo manager

- **Ideas:**
  - Quick view of top idea from idea manager

_`Test environment`: `OS Linux`, `Rust 1.90+`_

**Recommendation:** _Add to your `~/.bashrc` or `~/.zshrc` to run on terminal startup:_

```bash
# Add this line to your shell config
greet
```

### Installation
_Easy approach:_
```bash
cargo install motils
```
_or just use the repo_

```bash
# Clone the repository
$ git clone https://github.com/pykeras/BashUtils
$ cd motils

# Build and install
$ cargo install --path .

# Or build in release mode
$ cargo build --release
# Binaries will be in ./target/release/
```

This will install three binaries:
- `greet` - System dashboard (default)
- `todo` - Todo manager
- `idea` - Idea manager
