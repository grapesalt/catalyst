use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub site_title: String,
    pub site_logo: String,
    pub content_dir: String,
    pub output_dir: String,
    pub theme_dir: String,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Self {
        let raw_config = fs::read_to_string(file_path).expect(
            format!("Failed to read config file {}", file_path).as_str(),
        );

        serde_yaml::from_str(&raw_config).expect("Failed to parse config file")
    }
}
