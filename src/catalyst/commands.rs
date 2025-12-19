use glob::glob;
use notify::{RecursiveMode, Watcher};
use std::{
    collections::HashSet, env, ffi::OsStr, fs, path::PathBuf,
    sync::mpsc::channel, thread,
};

use crate::catalyst::{
    config::Config,
    posts::{self, Post},
    render,
};

pub fn build(config: &Config) -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    glob(&format!("{}/**/*.md", config.content_dir))
        .expect("Failed to read glob pattern")
        .for_each(|entry| {
            let path = entry.expect("Failed to read entry");
            posts.push(render::process_markdown(
                &config,
                path.to_str().unwrap(),
            ));
        });

    posts::generate_index(&config, &posts);

    posts
}

pub fn add(config: &Config, title: &String, directory: &Option<String>) {
    let slug = title
        .to_lowercase()
        .replace(" ", "-")
        .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");

    if directory.is_some() {
        fs::create_dir_all(&format!(
            "{}/{}",
            config.content_dir,
            directory.as_ref().unwrap()
        ))
        .expect("Failed to create category directory");
    }

    let post_path = if directory.is_some() {
        format!(
            "{}/{}/{}.md",
            config.content_dir,
            directory.as_ref().unwrap(),
            slug
        )
    } else {
        format!("{}/{}.md", config.content_dir, slug)
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
    let content_dir = env::current_dir().unwrap().join(&config.content_dir);

    for post in posts {
        let full_path = content_dir.join(format!("{}.md", post.slug));
        paths.insert(full_path);
    }
    paths
}
pub fn run(config: &Config, serve: bool) {
    let mut posts = build(config);

    let mut post_paths = slugs_to_path(config, &posts);

    let config_clone = config.clone();

    thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher =
            notify::recommended_watcher(tx).expect("Failed to create watcher");

        watcher
            .watch(
                PathBuf::from(&config_clone.content_dir).as_path(),
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

                            render::process_markdown(
                                &config_clone,
                                stripped_path,
                            );

                            // Checks if the changed file is a new post
                            // Does not rebuild index.html if a title or date changes
                            // But how likely is that?
                            if !post_paths.contains(&path) {
                                println!(
                                    "New post detected. Building index.html..."
                                );
                                posts.push(render::process_markdown(
                                    &config_clone,
                                    stripped_path,
                                ));
                                post_paths.insert(path.clone());
                                posts::generate_index(&config_clone, &posts);
                            }
                        }
                    }
                }

                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });

    println!("Watching for changes. Press Ctrl+C to stop.");

    if serve {
        let server = file_serve::Server::new(&config.output_dir);
        let url = format!("http://{}", server.addr());

        println!("Server started on {}...", url);

        server.serve().unwrap();

        open::that(url).unwrap();
    } else {
        loop {
            thread::park();
        }
    }
}
