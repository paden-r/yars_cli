use std::{env, string};
use log::info;
use std::error::Error;
use clap::{Parser, Subcommand};
use env_logger::Env;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[command(subcommand)]
    action: Option<Commands>
}


#[derive(Subcommand)]
enum Commands {
    List {},
    Get {
        #[arg(short, long)]
        id: u16,

        #[arg(short, long)]
        file_name: String
    },
    Add {
        title: String,

        file_path: String,

        rank: String,

        #[arg(short, long, default_value_t = string::String::from(""))]
        summary: String
    },

    Update {
        id: u16,

        #[arg(short, long, default_value_t = string::String::from(""))]
        title: String,

        #[arg(short, long, default_value_t = string::String::from(""))]
        file_path: String,

        #[arg(short, long, default_value_t = string::String::from(""))]
        rank: String,

        #[arg(short, long, default_value_t = string::String::from(""))]
        summary: String
    },

    Delete {
        id: u16
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logging()?;
    let cli = Args::parse();

    match &cli.action {
        Some(Commands::List {}) => {
            info!("Listing all!");
        }
        Some(Commands::Get {id, file_name}) => {
            info!("{:?} {:?}", id, file_name);
        },
        Some(Commands::Add {title, file_path, rank, summary}) => {
            info!("{:?} {:?}", title, file_path);
            info!("{:?} {:?}", rank, summary);
        },
        Some(Commands::Update {id, title, file_path, rank, summary}) => {
            info!("{:?} {:?}", title, file_path);
            info!("{:?} {:?}", rank, summary);
        },
        Some(Commands::Delete {id}) => {
            info!("{:?}", id);
        }
        None => {}
    }

    Ok(())
}

fn setup_logging() -> std::result::Result<(), Box<dyn Error>> {
    let mut logger = env_logger::Builder::from_env(Env::default().default_filter_or("info"));
    logger.target(env_logger::Target::Stdout);
    logger.init();
    match env::var("RUST_LOG") {
        Ok(l) => {
            info!("Log Level: {}", l.to_uppercase())
        }
        Err(_) => {
            info!("Log Level: INFO")
        }
    }
    Ok(())
}