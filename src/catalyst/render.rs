use serde_yaml::Value;
use std::collections::HashMap;

use crate::catalyst::{
    config::Config,
    posts::Post,
    template::{apply, process_containers},
};

pub fn render_html(markdown_input: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();

    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_WIKILINKS);
    options.insert(Options::ENABLE_SUBSCRIPT);
    options.insert(Options::ENABLE_SUPERSCRIPT);

    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn process_markdown(config: &Config, file_path: &str) -> Post {
    use std::fs;

    let mut markdown_input =
        fs::read_to_string(file_path).expect("Failed to read markdown file");

    let mut frontmatter = String::new();

    if markdown_input.starts_with("---") {
        if let Some(end) = markdown_input[3..].find("---") {
            frontmatter = markdown_input[3..end + 3].to_string();
        }
    }

    markdown_input = process_containers(config, &markdown_input);

    let data =
        serde_yaml::from_str::<HashMap<String, Value>>(&frontmatter).unwrap();

    let content = render_html(&markdown_input);

    let html = apply(
        format!("{}/template.html", config.theme_dir).as_str(),
        &config,
        Some(&data),
        content,
    );

    // TODO: This is very hacky and needs to be fixed.
    // should use like Path and PathBuf instead of string manipulation
    let output_path = file_path
        .replace(&config.content_dir, &config.output_dir)
        .replace(".md", ".html");

    fs::create_dir_all(std::path::Path::new(&output_path).parent().unwrap())
        .expect("Failed to create output directory");

    fs::write(&output_path, &html).expect("Failed to write HTML output");

    let title = data
        .get("title")
        .expect(
            format!(
                "Frontmatter must contain a title field for file: {file_path}",
            )
            .as_str(),
        )
        .as_str()
        .unwrap()
        .to_string();

    let date = data
        .get("date")
        .expect(
            format!(
                "Frontmatter must contain a date field for file: {file_path}",
            )
            .as_str(),
        )
        .as_str()
        .unwrap()
        .to_string();

    let slug = file_path
        .replace(&config.content_dir, "")
        .replace(".md", "")
        .trim_start_matches('/')
        .to_string();

    Post { title, date, slug }
}
