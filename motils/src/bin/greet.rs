use std::thread;

use anyhow::Result;
use motils::{style::Colorize, system};

fn main() -> Result<()> {
    thread::scope(|s| -> Result<()> {
        let hostname_h = s.spawn(system::get_hostname);
        let uptime_h = s.spawn(system::get_uptime);
        let ram_h = s.spawn(system::get_ram_usage);
        let cpu_h = s.spawn(system::get_cpu_usage);
        let disk_h = s.spawn(system::get_disk_usage);
        let net_h = s.spawn(system::get_network);
        let todos_h = s.spawn(system::get_todos);
        let ideas_h = s.spawn(system::get_ideas);

        let hostname = hostname_h.join().unwrap()?;
        let uptime = uptime_h.join().unwrap()?;
        let ram = ram_h.join().unwrap()?;
        let cpu_pct = cpu_h.join().unwrap()?;
        let disk = disk_h.join().unwrap()?;
        let net = net_h.join().unwrap()?;
        let todos = todos_h.join().unwrap()?;
        let ideas = ideas_h.join().unwrap()?;

        let system_title = "\r 󰌢 SYSTEM:".styled().bold_white_on_blue();

        let hostname = format!("{}: {}", "\r  hostname".styled().cyan(), hostname);
        let uptime = format!("{}: {}", "\r  uptime".styled().cyan(), uptime);
        let ram = format!("{}: {}", "\r  ram".styled().cyan(), ram);

        let cpu_pct_color = match cpu_pct {
            v if v < 50_f32 => format!("{v:.2}%").styled().green(),
            v if v < 80_f32 => format!("{v:.2}%").styled().yellow(),
            v => format!("{v:.2}%").styled().red(),
        };
        let cpu = format!("{}: {}", "\r  cpu".styled().cyan(), cpu_pct_color);

        let disk = format!(
            "{}:\n    {}",
            "\r  disk".styled().cyan(),
            disk.join("\n    ")
        );
        println!("{system_title}\n{hostname}\n{uptime}\n{cpu}\n{ram}\n{disk}\n");

        let network_title = "\r 󰖩 NETWORK:".styled().bold_white_on_blue();
        println!("{network_title}");
        for (iname, ip) in net {
            println!("  {}    {}", iname.styled().cyan(), ip.styled().magenta())
        }

        let todo_title = "\n\r  TODOs:".styled().bold_white_on_blue();
        println!("{todo_title}");
        for t in todos {
            println!("\r  {t}")
        }

        let ideas_title = "\n\r 󰍩 IDEAs:".styled().bold_white_on_blue();
        println!("{ideas_title}");
        for i in ideas {
            println!("\r  {i}")
        }
        Ok(())
    })
}
