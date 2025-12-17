use serde_yaml::Value;

use crate::catalyst::config;

use std::{collections::HashMap, fs};

pub fn apply(
    template_path: &str,
    config: &config::Config,
    data: Option<&HashMap<String, Value>>,
    content: String,
) -> String {
    let mut template = fs::read_to_string(&template_path)
        .expect("Failed to read template file");

    template = template.replace("{{site_title}}", &config.site_title);
    template = template.replace("{{site_logo}}", &config.site_logo);
    template = template.replace("{{content}}", &content);

    // If there is frontmatter data, replace placeholders
    if let Some(data) = data {
        for (key, value) in data.iter() {
            template = template.replace(
                format!("{{{}}}", key).as_str(),
                value.as_str().unwrap(),
            );
        }
    }

    template
}
