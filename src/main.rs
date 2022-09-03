mod db;
mod wp_crawler;

use anyhow::Result;
use clap::Parser;
use wp_crawler::send_request_to_wordpress;

/// Let's crawl
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Wordpress site name to crawl
    #[clap(short, long)]
    sitename: String,

    /// Option to conver HTML to plain text
    #[clap(short, long)]
    plain_text: bool,
    /// Option to insert to database
    #[clap(short, long)]
    insert_to_db: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

pub struct Config {
    pub sitename: String,
    pub plain_text: bool,
    pub insert_to_db: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let sitename = args.sitename;

    let plain_text = args.plain_text;

    let insert_to_db = args.insert_to_db;

    let config = Config {
        sitename,
        plain_text,
        insert_to_db,
    };

    send_request_to_wordpress(config).await?;

    Ok(())
}
