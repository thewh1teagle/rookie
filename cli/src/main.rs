use clap::Parser;
use rookie::{chromium_based, firefox_based, config, Cookie};
use serde_json;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    domains: Vec<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let domains: Vec<&str> = args.domains.iter().map(|d| d.as_str()).collect();
    let mut cookies: Vec<Cookie> = vec![];

    let chrome_configs = &[
        &config::CHROME_CONFIG,
        &config::BRAVE_CONFIG,
        &config::CHROMIUM_CONFIG,
        &config::EDGE_CONFIG,
        &config::OPERA_CONFIG,
        &config::OPERA_GX_CONFIG,
        &config::VIVALDI_CONFIG,
    ];

    // chromium
    for browser_config in chrome_configs {
        let res_cookies = chromium_based(&browser_config, args.path.to_owned().into(), Some(domains.to_owned()))?;
        cookies.extend(res_cookies);
        if let Ok(str) = serde_json::to_string_pretty(&cookies) {
            println!("{}", str);
            return Ok(());
        }
    }

    // mozilla
    let res_cookies = firefox_based(args.path.to_owned().into(), Some(domains.to_owned()))?;
    cookies.extend(res_cookies);
    if let Ok(str) = serde_json::to_string_pretty(&cookies) {
        println!("{}", str);
        return Ok(());
    }
    
    Ok(())
}