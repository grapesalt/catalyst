use std::{collections::HashMap, fs};

use crate::catalyst::{config::Config, template::apply};
use serde::Serialize;
use serde_yaml;

#[derive(Serialize, Debug)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub slug: String,
}

pub fn generate_index(config: &Config, posts: &Vec<Post>) {
    let mut data = HashMap::new();
    data.insert(
        "posts".to_string(),
        serde_yaml::to_value(posts).expect("serialize posts"),
    );

    fs::write(
        format!("{}/index.html", config.build),
        apply(
            &format!("{}/index.html", config.theme),
            &config,
            Some(&data),
            String::new(),
        ),
    )
    .expect("Could not generate index.html");
}
