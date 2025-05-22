mod git_parser;
mod heatmap;
mod tui;

use clap::Parser;
use git_parser::parse_repo;
use heatmap::generate_heatmap;
use tui::render_heatmap;

#[derive(Parser)]
#[command(name = "githeat", version, about = "Git contribution heatmap for your terminal - by Andrew Alvarez")]
struct Args {
    #[arg(short, long, help = "Show heatmap grouped by author")]
    by_author: bool,

    #[arg(short, long, help = "Only include commits from the last N days")]
    since: Option<u32>,

    

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let commits = parse_repo(args.since)?;
    let heatmap = generate_heatmap(commits, args.by_author);
    render_heatmap(&heatmap)?;
    Ok(())
}
