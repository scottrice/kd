use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::{exit, Command};

use colored::Colorize;
use itertools::Itertools;
use serde::Deserialize;
use shell_words;

use anyhow::{anyhow, Context, Result};

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

fn execute_cmd(config: &SubcommandConfig) -> Result<i32> {
    let mut cmd = shell_words::split(&config.cmd)
        .context("while parsing the subcommand's `cmd` field")?;

    // TODO: Figure out if anything breaks when we have nonstandard commands
    let args = cmd.split_off(1);
    let program = &cmd[0];

    // Actually execute the cmd process
    let mut process = Command::new(program)
        .args(args)
        .spawn()
        .context("Error while spawning subprocess. Likely the program doesn't exist.")?;
    let status = process.wait()
        .context("Error while waiting for process to exit.")?;

    // TODO: Is it really an error case if the process was terminated by a
    // signal? Seems like a reasonable thing to happen, we should probably just
    // return 1 (check to see what happens to other processes when we ctrl+c)
    status.code().ok_or_else(|| anyhow!("Process terminated by signal"))
}

fn run_subcommand(config_optional: Option<HashMap<String, SubcommandConfig>>, subcommand: &str) -> Result<i32> {
    if let Some(config) = config_optional {
        // We do have a `.kdconfig`.
        if let Some(subcommand_config) = config.get(subcommand) {
            // And we have an entry for the subcommand the user entered.
            let exitcode = execute_cmd(subcommand_config)
                .context("while running subcommand")?;
            Ok(exitcode)
        } else {
            // And we don't have an entry for the subcommand the user wanted.
            println!("No subcommand found called {}.", subcommand);
            Ok(1)
        }
    } else {
        // We don't have a config. Likely the first time the user is using our
        // tool. Make sure to give them a good message.
        // TODO: Give them a good message.
        println!("You don't have a `.kdconfig` yet. Try making one.");
        Ok(1)
    }
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

    let exitcode = run_subcommand(config, subcommand)
        .context("while trying to run the subcommand")?;
    exit(exitcode);
}
