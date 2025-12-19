use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

use crate::catalyst::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildCache {
    pub posts: HashMap<String, SystemTime>,
}

impl BuildCache {
    pub fn load(config: &Config) -> Self {
        if let Ok(raw_cache) = fs::read_to_string(&config.cache) {
            serde_yaml::from_str(&raw_cache).unwrap_or_default()
        } else {
            BuildCache {
                posts: HashMap::new(),
            }
        }
    }

    pub fn save(&self, config: &Config) {
        if config.incremental {
            let _ =
                fs::write(&config.cache, serde_yaml::to_string(self).unwrap());
        }
    }
}

impl Default for BuildCache {
    fn default() -> Self {
        BuildCache {
            posts: HashMap::new(),
        }
    }
}
