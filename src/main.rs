mod kdconfig;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use itertools::Itertools;
use rand::Rng;
use shell_words;

use kdconfig::{KdConfig, SubcommandConfig};

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

fn print_help(maybe_config: Option<KdConfig>) -> Result<()> {
    print!("{HELP_PRELUDE}");

    if maybe_config.is_none() {
        return Ok(());
    }
    let config = maybe_config
        .expect("Config should be Some since we just checked");

    // TODO: Skip this section if there are no subcommands in the file
    // (or give a helpful message)

    println!("");
    println!("AVAILABLE SUBCOMMANDS:");

    for command in config.commands_by_name.keys().sorted() {
        let bolded_command = command.bold().underline();

        let command_definition = config
            .commands_by_name
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

fn find_config() -> Option<(PathBuf, File)> {
    let mut path = std::env::current_dir()
        .expect("Cannot access current directory");
    let filename = Path::new(".kdconfig");

    loop {
        path.push(filename);

        match File::open(&path).map_err(|e| e.kind() ) {
            Ok(file) => {
                // Remove filename from the path, leaving the directory.
                path.pop();
                return Some((path, file));
            }
            // File not found will be given when there is no .kdconfig in the
            // current directory. In that scenario, just move up a directory
            Err(std::io::ErrorKind::NotFound) => {
                // one pop for the filename we added, one for the directory
                // If one of these calls fails, it means we have reached the
                // root of the filesystem and didn't find anything.
                if !(path.pop() && path.pop()) {
                    return None;
                }
            }
            // TODO: We found a file but can't access it (likely for
            // permissions reasons). We should give the user a better error
            // message here, but right now we return the same thing as if we
            // couldn't find a kdconfig at all.
            Err(_) => return None,
        }
    }
}

fn subcommand_args() -> Vec<String> {
    let mut all_args: Vec<String> = env::args().collect();

    // Remove the first two arguments to `kd`. argv[0] is obviously the path to
    // kd itself, and argv[1] is the subcommand. Neither get forwarded on.
    all_args.split_off(2)
}

fn write_to_temp_file(args: Vec<String>) -> PathBuf {
    let mut rng = rand::thread_rng();

    let filename = format!("kd-{}", rng.gen::<u32>());
    let mut argsfile = env::temp_dir();
    argsfile.push(filename);

    // TODO: We throw error handling out the window here, which is not good.
    // Make this handle errors gracefully.
    let mut writer = File::create(&argsfile).unwrap();
    for arg in args {
        writeln!(&mut writer, "{}", arg)
            .expect("Failed while writing argument to argsfile");
    }

    argsfile
}

fn replace_magic_strings_with_kd_args(raw_args: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for arg in raw_args {
        if arg == "{{ARGS}}" {
            let mut kdargs = subcommand_args();
            result.append(&mut kdargs);
        } else if arg == "{{ARGSFILE}}" {
            // TODO: Should we be worried about not being able to turn our path
            // into a string?
            let tmppath = write_to_temp_file(subcommand_args())
                .into_os_string()
                .into_string()
                .unwrap();
            result.push(tmppath);
        } else {
            // Static argument - just add it in place
            result.push(arg);
        }
    }

    result
}

fn execute_cmd(config: &SubcommandConfig, config_directory: &Path) -> Result<i32> {
    let mut cmd = shell_words::split(&config.cmd)
        .context("while parsing the subcommand's `cmd` field")?;

    // TODO: Figure out if anything breaks when we have nonstandard commands
    let raw_args = cmd.split_off(1);
    let program = &cmd[0];

    let args = replace_magic_strings_with_kd_args(raw_args);

    // Actually execute the cmd process
    let mut process = Command::new(program)
        .args(args)
        .current_dir(config_directory)
        .spawn()
        .context("Error while spawning subprocess. Likely the program doesn't exist.")?;
    let status = process.wait()
        .context("Error while waiting for process to exit.")?;

    // TODO: Is it really an error case if the process was terminated by a
    // signal? Seems like a reasonable thing to happen, we should probably just
    // return 1 (check to see what happens to other processes when we ctrl+c)
    status.code().ok_or_else(|| anyhow!("Process terminated by signal"))
}

fn run_subcommand(maybe_config: Option<KdConfig>, subcommand: &str) -> Result<i32> {
    if let Some(config) = maybe_config {
        // We do have a `.kdconfig`.
        if let Some(subcommand_config) = config.commands_by_name.get(subcommand) {
            // And we have an entry for the subcommand the user entered.
            let exitcode = execute_cmd(subcommand_config, &config.root_path)
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

    let nearest_config_file = find_config();
    let config = nearest_config_file
        .map(|(path, file)| KdConfig::parse(path, file))
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
