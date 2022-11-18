use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};
use json_comments::StripComments;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubcommandConfig {
    pub cmd: String,
    pub help: String,
}

pub struct KdConfig {
  pub root_path: PathBuf,
  pub commands_by_name: HashMap<String, SubcommandConfig>,
}

impl KdConfig {
  pub fn parse(path: PathBuf, file: File) -> Result<KdConfig> {
    let reader = BufReader::new(file);
    let parsed_command_mapping = KdConfig::parse_commands(reader)?;

    let kdconfig = KdConfig {
      root_path: path,
      commands_by_name: parsed_command_mapping,
    };
    Ok(kdconfig)
  }

  fn parse_commands<T: Read>(reader: T) -> Result<HashMap<String, SubcommandConfig>> {
    // Strip comments out of the JSON file, so that people can add comments
    // to their config files without triggering an error.
    let stripped = StripComments::new(reader);
    // Use serde to parse our mapping
    let result: HashMap<String, SubcommandConfig> = serde_json::from_reader(stripped)
      .context("while parsing config file.")?;

    // TODO: There should be a way to directly return the result of
    // `from_reader` but I think the error types get messed up.
    Ok(result)
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_json() {
      let data = r#"
        {
            "hello": {
              "cmd": "echo Hello World!",
              "help": "Prints a friendly message"
            }
        }"#;
      let result = KdConfig::parse_commands(data.as_bytes());
      assert!(result.is_ok());

      let mapping = result.unwrap();
      assert!(mapping.get("hello").is_some());
      assert!(mapping.get("world").is_none());
    }

    #[test]
    fn test_parsing_json_with_comments() {
      let data = r#"
        {
            // This is a helpful comment about a helpful command
            "hello": {
              /* Notice that we use the same phrase everyone else does?
                 That's on purpose. */
              "cmd": "echo Hello World!",
              "help": "Prints a friendly message"
            }
        }"#;
      let result = KdConfig::parse_commands(data.as_bytes());
      assert!(result.is_ok());

      let mapping = result.unwrap();
      assert!(mapping.get("hello").is_some());
      assert!(mapping.get("world").is_none());
    }
}