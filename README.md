# `kd` - A simple project command runner

`kd` allows projects to check common commands into version control which then gets shared with every developer on the project. Great for things like build commands or test running commands. These commands are easily discoverable via `kd help`.

## Setup

To use `kd`, all you need is a `.kdconfig`. This file is a simple JSON file with a dictionary of command name to definition. An example might look like this:

```
{
  "build": {
    "cmd": "make all",
    "help": "Build everything"
  }
}
```

## Command Definition Reference

| Field | Description |
| --- | --- |
| `cmd` | The command that should be run when `kd foo` gets invoked. |
| `help` | The text that is shown for this subcommand when `kd help` or `kd --help` gets run (longer form help should be provided by your tool through `kd mysubcommand --help`) |


## Passing arguments to subcommands

`kd` provides two different ways of passing CLI arguments from `kd` itself to subcommands. By putting these magic strings in the `cmd` field of a subcommand, `kd` will replace those magic strings with the arguments used to invoke `kd` itself.

| Magic string | Description |
| --- | --- |
| `{{ARGS}}` | This will pass the arguments directly to the subcommand in as `argv`. |
| `{{ARGSFILE}}` | This will write the arguments to a temp file (one argument per line) and pass the temp file name to the subcommand |

Imagine a situation in which our goal is to allow people to run `kd test path/to/some/file` to run all tests in that file. For the sake of this example lets say that we have a script in the repo which will run a test file at `scripts/runtest`. This can be accomplished like so:

```
{
  "test": {
    "cmd": "scripts/runtest {{ARGS}}",
    "help": "Runs a test file"
  }
}
```

## Comparison to other tools

The original inspiration for `kd` was Arcanist (aka `arc`) used at Facebook/Meta. It was the main entrypoint for most developer tools at the company, and it was really convenient to have a single tool that would work in any repo and give access to common commands in said repo.

That being said, there are a couple other tools that solve similar problems:

* `make` - The classic example, as most projects use `make all` or `make install` for their build/install commands. Due to `make`s history as a build tool it also does a lot of dependency tracking, which gets in the way when you want something other than a build tool.
* `just` - A very similar tool to `kd` - it solves the same problems in much the same way. The main drawback is the syntax of its configuration file, which is inspired by make and includes a lot of dependency-like features. The goal of `kd`, by contrast, is to be as dumb as possible and therefore simple to set up.

## Why is it called "kd"?

Because `k` and `d` are at very convenient locations on a standard QWERTY keyboard.