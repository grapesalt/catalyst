use std::fs;

use clap::{Parser, Subcommand};

mod catalyst;

#[derive(Parser)]
#[command(name = "catalyst")]
#[command(about = "A static site generator", long_about = None)]
struct Cli {
    /// Path to config file
    #[arg(short, long, default_value = "catalyst.yaml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the site
    Build {
        #[arg(short, long, default_value = "false")]
        purge: bool,
        #[arg(long = "disable-incremental", default_value = "false")]
        incremental: bool,
    },

    /// Watch the site for changes
    Watch {
        #[arg(short, long, default_value = "false")]
        purge: bool,
        #[arg(long = "disable-incremental", default_value = "false")]
        incremental: bool,
    },

    /// Create a new post with the given title
    Add {
        title: String,
        #[arg(short = 'f', long)]
        folder: Option<String>,
    },

    /// List all posts
    List,
}

fn purge_output(config: &catalyst::config::Config) {
    let _ = fs::remove_dir_all(&config.build);
    let _ = fs::remove_file(&config.cache);
}

fn main() {
    let cli = Cli::parse();
    let config = catalyst::config::Config::load_from_file(&cli.config);

    match cli.command {
        Commands::Build { purge, incremental } => {
            if purge {
                purge_output(&config);
            }

            catalyst::commands::build(
                &config,
                config.incremental && !incremental,
            );
        }
        Commands::Watch { purge, incremental } => {
            if purge {
                purge_output(&config);
            }

            catalyst::commands::watch(
                &config,
                config.incremental && !incremental,
            );
        }
        Commands::Add { title, folder } => {
            catalyst::commands::add(&config, &title, &folder);
        }
        Commands::List => {
            catalyst::commands::list(&config);
        }
    }
}
