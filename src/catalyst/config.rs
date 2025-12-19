use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub title: String,
    pub logo: String,
    pub entries: String,
    pub build: String,
    pub theme: String,
    #[serde(default)]
    pub containers: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContainerConfig {
    pub nargs: usize,
    pub template: String,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Self {
        let raw_config = fs::read_to_string(file_path)
            .expect(&format!("Failed to read config file {file_path}"));

        serde_yaml::from_str(&raw_config).expect("Failed to parse config file")
    }
}

impl ContainerConfig {
    pub fn load_from_file(file_path: &str) -> HashMap<String, Self> {
        let raw_yaml = fs::read_to_string(file_path)
            .expect(&format!("Failed to read container file {file_path}"));

        serde_yaml::from_str::<HashMap<String, Self>>(&raw_yaml)
            .expect("Failed to parse container file")
    }
}
