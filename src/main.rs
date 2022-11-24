use std::env;
use log::info;
use std::error::Error;
use clap::Parser;
use env_logger::Env;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    action: String,

    #[clap(short, long, default_value_t = None)]
    post_id: u16,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logging()?;
    let args = Args::parse();
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