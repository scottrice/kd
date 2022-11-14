use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use colored::Colorize;
use itertools::Itertools;
use serde::Deserialize;

use anyhow::{Context, Result};

// `kd` lets you create easy-to-remember and ergonomic CLI commands at a
// per-project level. These can be checked into source control and shared by every
// engineer on the project.

// If the first argument to `kd` is either of these strings then print out the
// help text rather than invoke a program.
const HELP_TRIGGERS: &'static [&'static str] = &[
    "-h",
    "--help",
    "help"
];

const HELP_PRELUDE: &'static str = "\
kd - A per-project command runner.

USAGE:
  $ kd <subcommand> <subcommand arguments>
";

#[derive(Debug, Deserialize)]
struct SubcommandConfig {
    cmd: String,
    help: String,
}

fn print_help(config: Option<HashMap<String, SubcommandConfig>>) -> Result<()> {
    print!("{HELP_PRELUDE}");

    if config.is_none() {
        return Ok(());
    }
    let config_values = config
        .expect("Config should be Some since we just checked");

    // TODO: Skip this section if there are no subcommands in the file
    // (or give a helpful message)

    println!("");
    println!("AVAILABLE SUBCOMMANDS:");

    for command in config_values.keys().sorted() {
        let bolded_command = command.bold().underline();

        let command_definition = config_values
            .get(command)
            .expect("We should have been given a valid key");
        let command_help = &command_definition.help;

        // Newline at the start gives space between entries, as well as space
        // between the heading and the first entry.
        println!("");
        println!("  {bolded_command}");
        println!("    {command_help}");
    }

    Ok(())
}

fn find_config() -> Option<File> {
    let path = Path::new(".kdconfig");

    // TODO: Make this search up the directory hierarchy.
    // TODO: Right now we return a None any time we get an error while opening
    //   We should treat only file-not-found as a None, and other errors like
    //   permissions should be treated as an error state ("we have a kdconfig
    //   but we can't open it")
    File::open(path)
        .ok()
}

fn parse_config(file: File) -> Result<HashMap<String, SubcommandConfig>> {
    let reader = BufReader::new(file);

    let config: HashMap<String, SubcommandConfig> = serde_json::from_reader(reader)
        .context("Failed to parse config file as JSON")?;

    Ok(config)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let subcommand = &args[1];

    let config = find_config()
        .map(|path| parse_config(path))
        .transpose()
        .context("Failed to parse the config file")?;

    if HELP_TRIGGERS.contains(&subcommand.as_str()) {
        print_help(config)
            .context("Error while printing help")?;

        return Ok(());
    }

    // Our subcommand is a user-defined one, execute it.

    let exitcode = 1;
    exit(exitcode);
}
