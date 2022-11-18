use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Context, Result};
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

    let parsed_command_mapping: HashMap<String, SubcommandConfig> = serde_json::from_reader(reader)
        .context("Failed to parse config file as JSON")?;

    let kdconfig = KdConfig {
      root_path: path,
      commands_by_name: parsed_command_mapping,
    };
    Ok(kdconfig)
  }
}