use std::{
    collections::HashMap,
    convert::Infallible,
    env::args,
    error::Error,
    fs::File,
    io::Read,
    os::unix::process::CommandExt,
    process::Command,
};

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    default: Program,
    overrides: HashMap<String, Program>,
}

#[derive(Deserialize)]
struct Program {
    command: String,
    args: Vec<String>,
}

fn main() -> Result<Infallible, Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    assert!(args.len() >= 2, "At least one argument must be provided");

    let config_path = dirs::config_dir()
        .ok_or("Could not locate config directory")?
        .join("choosy.toml");

    let mut config_str: String = Default::default();
    File::open(config_path)?.read_to_string(&mut config_str)?;
    let config: Config = toml::from_str(&config_str)?;

    let (_, program) = config
        .overrides
        .iter()
        .filter_map(|(regex_str, program)| match Regex::new(regex_str) {
            Ok(re) => Some((re, program)),
            Err(e) => {
                eprintln!("Failed to parse regex {}, skipping: {}", regex_str, e);
                None
            }
        })
        .flat_map(|(re, program)| {
            re.find_iter(&args[1])
                .map(|m| (m.len(), program))
                .collect::<Vec<_>>()
        })
        .max_by_key(|(match_length, _)| *match_length)
        .unwrap_or((0, &config.default));

    Err(Command::new(&program.command).args(program.args.iter().chain(&args[1..])).exec())?;
    unreachable!("Should have execed or errored");
}
