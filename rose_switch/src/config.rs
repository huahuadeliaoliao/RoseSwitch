use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mappings: HashMap<String, String>,
}

impl Config {
    pub fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(filename)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
