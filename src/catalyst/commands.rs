use notify::{RecursiveMode, Watcher};
use std::{
    collections::HashSet, env, ffi::OsStr, fs, path::PathBuf,
    sync::mpsc::channel,
};
use walkdir::WalkDir;

use crate::catalyst::{
    config::Config,
    posts::{self, Post},
    render,
};

pub fn build(config: &Config) -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    for file in WalkDir::new(&config.entries)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if file.path().extension() == Some(OsStr::new("md")) {
            posts.push(render::process_markdown(
                &config,
                file.path().to_str().unwrap(),
            ));
        }
    }

    posts::generate_index(&config, &posts);

    posts
}

pub fn add(config: &Config, title: &String, folder: &Option<String>) {
    let slug = title
        .to_lowercase()
        .replace(" ", "-")
        .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");

    if folder.is_some() {
        fs::create_dir_all(&format!(
            "{}/{}",
            config.entries,
            folder.as_ref().unwrap()
        ))
        .expect("Failed to create category directory");
    }

    let post_path = if folder.is_some() {
        format!(
            "{}/{}/{}.md",
            config.entries,
            folder.as_ref().unwrap(),
            slug
        )
    } else {
        format!("{}/{}.md", config.entries, slug)
    };

    let template = format!(
        "---\ntitle: {title}\ndate: {}\n---\n\n# {title}\n",
        chrono::Local::now().format("%Y-%m-%d"),
    );

    fs::write(&post_path, template).expect("Failed to create post");
    println!("Post created at: {}", post_path);
}

fn slugs_to_path(config: &Config, posts: &Vec<Post>) -> HashSet<PathBuf> {
    let mut paths: HashSet<PathBuf> = HashSet::new();
    let content_dir = env::current_dir().unwrap().join(&config.entries);

    for post in posts {
        let full_path = content_dir.join(format!("{}.md", post.slug));
        paths.insert(full_path);
    }
    paths
}

pub fn watch(config: &Config) {
    let mut posts = build(config);

    let mut post_paths = slugs_to_path(config, &posts);

    println!("Watching for changes. Press Ctrl+C to stop.");

    let (tx, rx) = channel();

    let mut watcher =
        notify::recommended_watcher(tx).expect("Failed to create watcher");

    watcher
        .watch(
            PathBuf::from(&config.entries).as_path(),
            RecursiveMode::Recursive,
        )
        .unwrap();

    let mut last_event_time = std::time::Instant::now();

    for res in rx {
        match res {
            Ok(event) => {
                // For some reason every file change triggers multiple events
                // Discount events that happen within 500ms of each other
                if last_event_time.elapsed().as_millis() < 500 {
                    continue;
                }

                last_event_time = std::time::Instant::now();

                for path in event.paths {
                    if path.extension() == Some(OsStr::new("md")) {
                        println!("Building {:?}", path);

                        // TODO: This basically assumes that the directory the user supplied is relative
                        // Which is probably the case but still, only siths deal in absolutes.

                        let stripped_path = path
                            .strip_prefix(std::env::current_dir().unwrap())
                            .unwrap()
                            .to_str()
                            .unwrap();

                        render::process_markdown(&config, stripped_path);

                        // Checks if the changed file is a new post
                        // Does not rebuild index.html if a title or date changes
                        // But how likely is that?
                        if !post_paths.contains(&path) {
                            println!(
                                "New post detected. Building index.html..."
                            );
                            posts.push(render::process_markdown(
                                &config,
                                stripped_path,
                            ));
                            post_paths.insert(path.clone());
                            posts::generate_index(&config, &posts);
                        }
                    }
                }
            }

            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

pub fn list(config: &Config) {
    let posts = build(&config);
    println!("Posts:");

    for post in posts {
        println!("{:<12}  {:<32}  {}", post.date, post.title, post.slug);
    }
}
