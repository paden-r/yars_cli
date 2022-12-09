use std::{env, string};
use log::info;
use std::error::Error;
use clap::{Parser, Subcommand};
use serde_json::json_expect_expr_comma;

mod command_handler;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    List {},
    Get {
        #[arg(short, long)]
        id: u16,

        #[arg(short, long)]
        file_name: String,
    },
    Add {
        title: String,

        file_path: String,

        rank: String,

        summary: String,
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
        summary: String,
    },

    Delete {
        id: u16
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    match &cli.action {
        Some(Commands::List {}) => {
            info!("Listing all!");
            command_handler::get_post_list().await;
        }
        Some(Commands::Get { id, file_name }) => {
            info!("{:?} {:?}", id, file_name);
        }
        Some(Commands::Add { title, file_path, rank, summary }) => {
            command_handler::add_post(title.to_owned(), file_path.to_owned(), rank.to_owned(), summary.to_owned()).await;
        }
        Some(Commands::Update { id, title, file_path, rank, summary }) => {
            info!("{:?} {:?}", title, file_path);
            info!("{:?} {:?}", rank, summary);
        }
        Some(Commands::Delete { id }) => {
            info!("{:?}", id);
        }
        None => {}
    }

    Ok(())
}
