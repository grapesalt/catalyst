use serde_yaml::Value;

use crate::catalyst::config::{self, ContainerConfig};

use std::{collections::HashMap, fs};

pub fn apply(
    template_path: &str,
    config: &config::Config,
    data: Option<&HashMap<String, Value>>,
    content: String,
) -> String {
    let mut template = fs::read_to_string(&template_path)
        .expect("Failed to read template file");

    template = template.replace("{{site_title}}", &config.title);
    template = template.replace("{{site_logo}}", &config.logo);
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

fn apply_container(
    container_config: &ContainerConfig,
    container_string: &String,
) -> String {
    let mut filled_template = fs::read_to_string(&container_config.template)
        .expect("Failed to read container template file");

    // Collect all lines, skipping first and last (:::container_name and :::)
    let lines: Vec<&str> = container_string.lines().skip(1).collect();

    let end = if lines.len() > 0 { lines.len() - 1 } else { 0 };

    let mut args: Vec<&str> = Vec::new();
    let mut content_lines: Vec<&str> = Vec::new();

    for (i, line) in lines[0..end].iter().enumerate() {
        if i < container_config.nargs {
            args.push(line.trim());
        } else {
            content_lines.push(*line);
        }
    }

    let content = content_lines.join("\n");

    // TODO: Add regex support for embed links
    // Currently, some links (e.g., YouTube, Spotify) require specific embed URLs
    // rather than standard links. Should implement regex-based transformation
    // on the user side instead of just {{i}}.
    for (i, arg) in args.iter().enumerate() {
        filled_template = filled_template.replace(&format!("{{{{{i}}}}}"), arg);
    }

    filled_template.replace("{{content}}", &content)
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

                let filled_container =
                    apply_container(container_config, &container_string);
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
