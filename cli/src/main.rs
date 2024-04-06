use clap::Parser;
use rookie::{any_browser, common::enums::Cookie, load};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    path: Option<String>,

    #[arg(short, long)]
    key_path: Option<String>,

    #[arg(short, long)]
    domains: Vec<String>,
}

fn print_cookies(cookies: Vec<Cookie>) {
    if let Ok(str) = serde_json::to_string_pretty(&cookies) {
        println!("{str}");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    pretty_env_logger::init();
    let domains = args.domains.iter().map(String::as_str).collect();
    if let Some(path) = args.path {
        let cookies = any_browser(path.as_str(), Some(domains), args.key_path.as_deref())?;
        print_cookies(cookies);
        return Ok(());
    }
    // without key and path
    let cookies = load(Some(domains))?;
    print_cookies(cookies);
    Ok(())
}
