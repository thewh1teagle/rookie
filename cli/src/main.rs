use clap::Parser;
use rookie::{chromium_based, firefox_based, Cookie};
use serde_json;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    key_path: Option<String>,

    #[arg(short, long)]
    domains: Vec<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let domains: Vec<&str> = args.domains.iter().map(|d| d.as_str()).collect();
    let mut cookies: Vec<Cookie> = vec![];

    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            use rookie::config;
            let chrome_configs = &[
                &config::CHROME_CONFIG,
                &config::BRAVE_CONFIG,
                &config::CHROMIUM_CONFIG,
                &config::EDGE_CONFIG,
                &config::OPERA_CONFIG,
                &config::OPERA_GX_CONFIG,
                &config::VIVALDI_CONFIG,
            ];
            for browser_config in chrome_configs {
                let res_cookies = chromium_based(&browser_config, args.path.clone().into(), Some(domains.to_owned()))?;
                cookies.extend(res_cookies);
            }
        } else {
            if let Some(key_path) = args.key_path {
                let res_cookies = chromium_based(PathBuf::from(key_path), args.path.to_owned().into(), Some(domains.to_owned()))?;
                cookies.extend(res_cookies);
            }
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