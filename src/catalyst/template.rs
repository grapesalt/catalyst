use serde_yaml::Value;
use std::{collections::HashMap, fs, sync::Arc};

use crate::catalyst::config::{self, ContainerConfig};

use lazy_static::lazy_static;
use minijinja::{Environment, context};

lazy_static! {
    static ref ENV: Arc<Environment<'static>> = Arc::new(Environment::new());
}

pub fn apply(
    template_path: &str,
    config: &config::Config,
    data: Option<&HashMap<String, Value>>,
    content: String,
) -> String {
    let template = fs::read_to_string(template_path)
        .expect("Failed to read template file");

    let ctx = context! {
        config => config,
        content => content,
        page => data
    };

    ENV.render_str(&template, ctx)
        .expect("template render failed")
}

fn parse_container_args(lines: &[&str]) -> (HashMap<String, Value>, String) {
    let mut param_lines: Vec<String> = Vec::new();
    let mut i: usize = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            i += 1;
            break;
        }
        if line.contains(':') {
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap_or("").trim();
            let val = parts.next().unwrap_or("").trim();
            param_lines.push(format!("{}: {}", key, val));
            i += 1;
        } else {
            param_lines.clear();
            i = 0;
            break;
        }
    }

    let mut params: HashMap<String, Value> = HashMap::new();
    if !param_lines.is_empty() {
        let raw_args = param_lines.join("\n");
        if let Ok(parsed) =
            serde_yaml::from_str::<HashMap<String, Value>>(&raw_args)
        {
            params = parsed;
        }
    }

    let content = if param_lines.is_empty() {
        lines.join("\n")
    } else {
        lines[i..].join("\n")
    };

    (params, content)
}

fn apply_container(
    container_name: &String,
    container_config: &ContainerConfig,
    container_string: &String,
) -> String {
    let lines_all: Vec<&str> = container_string.lines().skip(1).collect();
    let end = if !lines_all.is_empty() {
        lines_all.len().saturating_sub(1)
    } else {
        0
    };
    let lines = &lines_all[..end];

    let (params, content) = parse_container_args(lines);
    let raw_tmpl =
        fs::read_to_string(&container_config.template).expect(&format!(
            "Failed to read container template: {}",
            &container_config.template
        ));

    let ctx = context! {
        container => context! { name => container_name },
        params => params,
        content => content,
    };

    ENV.render_str(&raw_tmpl, ctx)
        .expect("template render failed")
}

pub fn process_containers(
    config: &config::Config,
    markdown_input: &str,
) -> String {
    if config.containers.is_none() {
        return markdown_input.to_string();
    }

    let container_configs =
        ContainerConfig::load_from_file(&config.containers.as_ref().unwrap());

    let mut output = String::new();
    let mut lines = markdown_input.lines();

    while let Some(line) = lines.next() {
        if line.starts_with(":::") {
            let container_name = line[3..].trim();
            if let Some(container_config) =
                container_configs.get(container_name)
            {
                let mut container_string = String::new();
                container_string.push_str(line);
                container_string.push('\n');

                while let Some(container_line) = lines.next() {
                    container_string.push_str(container_line);
                    container_string.push('\n');
                    if container_line.trim() == ":::" && container_line != line
                    {
                        break;
                    }
                }

                let filled_container = apply_container(
                    &container_name.to_string(),
                    &container_config,
                    &container_string,
                );
                output.push_str(&filled_container);
            } else {
                output.push_str(line);
                output.push('\n');
            }
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}
