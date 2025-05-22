mod git_parser;
mod heatmap;
mod tui;

use clap::Parser;
use git_parser::parse_repo;
use heatmap::generate_heatmap;
use tui::render_heatmap;
use chrono::{NaiveDate, Utc, TimeZone};

#[derive(Parser)]
#[command(
    name = "githeat",
    version,
    about = "🔥 Terminal-based Git contribution heatmap — by Andrew Alvarez"
)]
struct Args {
    #[arg(short, long, help = "Show heatmap grouped by author")]
    by_author: bool,

    #[arg(short, long, help = "Only include commits from the last N days")]
    since: Option<u32>,

    #[arg(long, help = "Only include commits after this YYYY-MM-DD date")]
    since_date: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let since_date = match &args.since_date {
        Some(date_str) => {
            match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                Ok(date) => {
                    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
                    let utc_time = Utc.from_local_datetime(&datetime).unwrap();
                    Some(utc_time.timestamp())
                }
                Err(_) => {
                    eprintln!("❌ Invalid date format. Use YYYY-MM-DD.");
                    std::process::exit(1);
                }
            }
        }
        None => None,
    };

    let commits = parse_repo(args.since, since_date)?;
    let heatmap = generate_heatmap(commits, args.by_author);
    render_heatmap(&heatmap)?;
    Ok(())
}
