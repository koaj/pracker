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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let sitename = args.sitename;

    let plain_text = args.plain_text;

    send_request_to_wordpress(sitename.as_str(), plain_text).await?;

    Ok(())
}
