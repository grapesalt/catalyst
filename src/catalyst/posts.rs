use std::fs;

use crate::catalyst::{config::Config, template::apply};

pub struct Post {
    pub title: String,
    pub date: String,
    pub slug: String,
}

fn generate_posts(posts: &[Post]) -> String {
    let mut posts_html = String::new();

    for post in posts {
        posts_html.push_str(&format!(
            "<li class=\"post-item\"><a class=\"post-link\" href=\"{}.html\"><span class=\"post-title\">{}</span></a> - <span class=\"post-date\">{}</span></li>\n",
            post.slug, post.title, post.date
        ));
    }

    format!("<ul>\n{}</ul>\n", posts_html)
}

pub fn generate_index(config: &Config, posts: &Vec<Post>) {
    fs::write(
        format!("{}/index.html", config.build),
        apply(
            &format!("{}/index.html", config.theme),
            &config,
            None,
            generate_posts(&posts),
        ),
    )
    .expect("Could not generate index.html");
}
