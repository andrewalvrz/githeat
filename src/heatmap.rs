use std::collections::HashMap;
use crate::git_parser::CommitInfo;

pub struct Heatmap {
    pub file_counts: HashMap<String, usize>,
    pub author_counts: HashMap<String, usize>,
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
        file_counts.clear(); // Don't display file stats if in author mode
    }

    Heatmap { file_counts, author_counts }
}
