use crate::catalyst::{config::Config, posts::Post};

use glob::glob;

mod catalyst;

fn main() {
    let config = Config::load_from_file("config.yaml");

    let mut posts: Vec<Post> = Vec::new();

    // Process all markdown files in the content directory
    glob(&format!("{}/**/*.md", config.content_dir))
        .expect("Failed to read glob pattern")
        .for_each(|entry| {
            let path = entry.expect("Failed to read entry");
            posts.push(catalyst::render::process_markdown(
                &config,
                path.to_str().unwrap(),
            ));
        });

    catalyst::posts::generate_index(&config, &posts);
}
