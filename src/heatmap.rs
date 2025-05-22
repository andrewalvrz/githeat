use std::collections::HashMap;
use crate::git_parser::CommitInfo;
use std::fs::File;
use std::io::Write;
use serde::Serialize;

#[derive(Serialize)]
pub struct Heatmap {
    pub file_counts: HashMap<String, usize>,
    pub author_counts: HashMap<String, usize>,
    pub total_commits: usize,
}

impl Heatmap {
    pub fn export(&self, format: &str) -> Result<(), Box<dyn std::error::Error>> {
        match format {
            "json" => {
                let json = serde_json::to_string_pretty(self)?;
                std::fs::write("githeat_export.json", json)?;
            }
            "md" => {
                let mut file = File::create("githeat_export.md")?;
                writeln!(file, "# Githeat Report\n")?;
                for (k, v) in &self.file_counts {
                    writeln!(file, "- `{}`: **{}** commits", k, v)?;
                }
            }
            _ => eprintln!("Unsupported export format: {}", format),
        }
        Ok(())
    }
}

pub fn generate_heatmap(commits: Vec<CommitInfo>, by_author: bool) -> Heatmap {
    let mut file_counts = HashMap::new();
    let mut author_counts = HashMap::new();

    for commit in commits {
        for file in commit.files {
            *file_counts.entry(file).or_insert(0) += 1;
        }
        *author_counts.entry(commit.author).or_insert(0) += 1;
    }

    if by_author {
        file_counts.clear();
    }

    let total_commits = file_counts.values().sum::<usize>() + author_counts.values().sum::<usize>();

    Heatmap { file_counts, author_counts, total_commits }
}
