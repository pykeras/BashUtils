use anyhow::{Context, Result};
use std::{fs, process::Command};

use crate::idea;
use crate::style::{Colorize, Styled};
use crate::todo::Priority;

pub fn get_hostname() -> Result<String> {
    let output = Command::new("hostname")
        .output()
        .context("can't read the result of `hostname` command")?;

    std::str::from_utf8(&output.stdout)
        .map(|s| s.trim().to_string())
        .context("failed to parse hostname as utf8")
}

fn get_cores() -> Result<u16> {
    std::thread::available_parallelism()
        .map(|n| n.get() as u16)
        .context("failed to get CPU core count")
}

pub fn get_cpu_usage() -> Result<f32> {
    let content = fs::read_to_string("/proc/loadavg").context("can't read from `/proc/loadavg`")?;
    let minute_avg = content
        .split_whitespace()
        .next()
        .context("loadavg is empty")?
        .parse::<f32>()
        .context("failed to parse loadavg as float")?;

    let cores = get_cores()? as f32;

    Ok((minute_avg / cores * 100_f32).min(100.0))
}

pub fn get_disk_usage() -> Result<Vec<String>> {
    let output = Command::new("df")
        .arg("-h")
        .args(["/", "/home"])
        .output()
        .context("can't read the result of `df -h / /home`")?;

    let res = std::str::from_utf8(&output.stdout)
        .context("failed to parse `df -h / /home` output as utf8")?;

    let lines = res
        .lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let [_, size, used, _, pct, mntpoint, ..] = parts.as_slice() else {
                return None;
            };
            Some((*mntpoint, *used, *size, *pct))
        })
        .collect::<Vec<_>>();

    let max_len = lines
        .iter()
        .map(|(mntpoint, ..)| mntpoint.len())
        .max()
        .unwrap_or(0);

    let mut res = Vec::with_capacity(lines.len());
    for (mntpoint, used, size, pct) in lines {
        let pct_int = pct
            .strip_suffix("%")
            .context("missing % in df output")?
            .parse::<u8>()
            .context("failed to parse pct")?;

        let usage_msg = format!("{used} used of {size} ({pct})");
        let colored_msg = match pct_int {
            0..30 => usage_msg.styled().green(),
            30..70 => usage_msg.styled().yellow(),
            _ => usage_msg.styled().red(),
        };
        let line = format!(
            "{}{}{}",
            mntpoint.styled().cyan(),
            " ".repeat(max_len - mntpoint.len() + 4),
            colored_msg
        );

        res.push(line);
    }
    Ok(res)
}

fn format_bytes(bytes: f32) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.0}{}", size, UNITS[unit_idx])
}
pub fn get_ram_usage() -> Result<Styled<String>> {
    let output = Command::new("free")
        .arg("-b")
        .output()
        .context("can't read result of `free -b` command")?;
    let res =
        std::str::from_utf8(&output.stdout).context("failed to parse `free -b` result as utf8")?;

    let mem_line = res.lines().nth(1).context("missing Mem line")?;
    let mem_parts: Vec<_> = mem_line.split_whitespace().collect();

    let [_, total, used, ..] = mem_parts.as_slice() else {
        anyhow::bail!("unexpected `free` output format");
    };

    let used = used
        .parse::<f32>()
        .context("failed to parse used section")?;
    let total = total
        .parse::<f32>()
        .context("failed to parse total section")?;
    let pct = (used / total) * 100_f32;

    let msg = format!(
        "{} / {} ({:.0})%",
        format_bytes(used),
        format_bytes(total),
        pct
    );
    let colored_msg = match pct {
        v if v < 30. => msg.styled().green(),
        v if v < 70. => msg.styled().yellow(),
        _ => msg.styled().red(),
    };

    Ok(colored_msg)
}

pub fn get_uptime() -> Result<String> {
    let content = fs::read_to_string("/proc/uptime").context("can't read `/proc/uptime`")?;

    let secs = content
        .split_whitespace()
        .next()
        .context("empty uptime")?
        .parse::<f64>()
        .context("failed to parse uptime")?;

    let total_secs = secs as u64;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;

    Ok(format!("{days}d {hours}h {mins}m"))
}

pub fn get_network() -> Result<Vec<(String, String)>> {
    let output = Command::new("ip")
        .args(["-o", "addr", "show"])
        .output()
        .context("can't read the output of `ip -o addr show` command")?;
    let res = std::str::from_utf8(&output.stdout)
        .context("failed to parse `ip -o addr show` output as utf8")?;
    let lines = res
        .lines()
        .filter_map(|l| {
            let virtual_iface = ["lo", "docker", "veth", "br-", "tun", "tap"];
            let mut parts = l.split_whitespace();
            // info: .nth will consume anything before provided index
            // consumes 0, returns 1
            let iface_name = parts
                .nth(1)
                .context("failed to get iface column from `ip -o addr show` command")
                .ok()?;
            // info: ip is the 4th element of the line, iface_name consumed first 2 items
            // consumes 2, returns 3
            let ip = parts
                .nth(1)
                .context("failed to get ip column from `ip -o addr show` command")
                .ok()?;

            if virtual_iface.iter().any(|&v| iface_name.contains(v)) {
                None
            } else {
                Some((
                    iface_name.to_string(),
                    ip.split_once("/")
                        .map(|(addr, _)| addr.to_string())
                        .unwrap_or_else(|| ip.to_string()),
                ))
            }
        })
        .collect::<Vec<_>>();
    Ok(lines)
}

fn get_probability(p: Priority, highest_present: Priority) -> u8 {
    let base: u8 = match p {
        Priority::Block => 10,
        Priority::High => 9,
        Priority::Medium => 7,
        Priority::Low => 3,
    };
    let bump = match highest_present {
        Priority::Block => 0,  // Nothing missing
        Priority::High => 1,   // Block missing: +10%
        Priority::Medium => 3, // Block+High missing: +30%
        Priority::Low => 7,    // Only low present: +70%
    };

    std::cmp::min(10, base + bump)
}

pub fn get_todos() -> Result<Vec<Styled<String>>> {
    use crate::todo::read_todos;
    use rand::RngExt;

    let mut rng = rand::rng();

    let app_dir = dirs::config_dir()
        .context("failed to get default config directory for user")?
        .join("motils");
    fs::create_dir_all(&app_dir)
        .context("failed to create `motils` directory in user config directory")?;

    let mut tasks = read_todos(&app_dir.join("todo")).context("failed to get list of Todos")?;
    if tasks.is_empty() {
        return Ok(Vec::new());
    }
    tasks.sort();

    let highest_present = tasks[0].priority;
    let mut result = Vec::new();

    for t in tasks {
        let prob = get_probability(t.priority, highest_present);
        let should_display = prob >= 10 || rng.random_ratio(prob as u32, 10);
        if should_display {
            let message = format!("{} {}", t.priority.icon(), t.description).styled();
            result.push(t.priority.style_message(message));
            if result.len() == 7 {
                break;
            }
        }
    }
    Ok(result)
}

pub fn get_ideas() -> Result<Vec<String>> {
    let app_dir = dirs::config_dir()
        .context("failed to get config directory")?
        .join("motils");

    let ipath = app_dir.join("ideas");
    let ideas = idea::read_ideas(&ipath)?;

    let mut out = Vec::new();
    for (idx, i) in ideas.iter().enumerate() {
        let message = format!("󰍩 [{}] {}", idx + 1, i.title)
            .styled()
            .magenta()
            .to_string();
        out.push(message);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Priority;

    #[test]
    fn test_probability_bumps() {
        assert_eq!(get_probability(Priority::Block, Priority::Block), 10);
        assert_eq!(get_probability(Priority::Low, Priority::Block), 3);

        assert_eq!(get_probability(Priority::Medium, Priority::High), 8); // 7 + 1
        assert_eq!(get_probability(Priority::Low, Priority::High), 4); // 3 + 1

        assert_eq!(get_probability(Priority::Low, Priority::Low), 10); // 3 + 7 = 10 (capped by std::cmp::min)
    }
}
