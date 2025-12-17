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
    Build,
    /// Run the site
    Run {
        #[arg(short = 's', long, default_value = "false")]
        serve: bool,
    },
    /// Create a new post with the given title
    Add {
        title: String,
        #[arg(short = 'd', long)]
        directory: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let config = catalyst::config::Config::load_from_file(&cli.config);

    match cli.command {
        Commands::Build => {
            catalyst::commands::build(&config);
        }
        Commands::Run { serve } => {
            catalyst::commands::run(&config, serve);
        }
        Commands::Add { title, directory } => {
            catalyst::commands::add(&config, &title, &directory);
        }
    }
}
