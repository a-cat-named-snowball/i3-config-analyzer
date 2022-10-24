use anyhow::anyhow;
use clap::Parser;
use dirs::config_dir;
use regex::Regex;
use std::collections::HashSet;
use which::which;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show only executables that are missing from your system
    #[arg(short, long)]
    missing: bool,

    /// Path of i3 config file, uses default location if omitted
    #[arg(short, long, value_name = "I3 CONFIG FILE")]
    path: Option<std::path::PathBuf>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Executable {
    name: String,
    found: bool,
}

fn print_all(execs: &HashSet<Executable>) {
    execs.iter().for_each(|exe| {
        println!("{}", exe.name);
    })
}

fn print_missing(execs: &HashSet<Executable>) {
    execs.iter().filter(|exe| !exe.found).for_each(|exe| {
        println!("{}", exe.name);
    })
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = match args.path {
        None => {
            let mut path = config_dir().map_or_else(
                || Err(anyhow!("Default i3 path could not be found. XDG directories may be configured improperly. Try passing the path manually with the -p flag.")),
                |v| Ok(v)
            )?;
            path.push("i3");
            path.push("config");
            path
        }
        Some(p) => {
            let mut path = std::path::PathBuf::new();
            path.push(p);
            path
        }
    };

    let cfg = std::fs::read_to_string(path.as_os_str()).map_err(|_| anyhow!("Invalid path"))?;

    let re = Regex::new(r#"(exec (-.*? )?"?|\$\()(?P<name>.*?)( |"|$)"#).unwrap();

    let mut execs = HashSet::new();
    cfg.split("\n").for_each(|line| {
        for caps in re.captures_iter(line) {
            let name = caps["name"].to_string();
            execs.insert(Executable {
                found: which(&name).map_or_else(|_| false, |_| true),
                name,
            });
        }
    });

    if args.missing {
        print_missing(&execs);
    } else {
        print_all(&execs);
    }

    Ok(())
}
