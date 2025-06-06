use git2::{Repository, DiffFormat};
use chrono::Utc;

#[allow(dead_code)]
pub struct CommitInfo {
    pub author: String,
    pub files: Vec<String>,
    pub timestamp: i64,
}

pub fn parse_repo(
    since_days: Option<u32>,
    since_date: Option<i64>,
    ext_filter: Option<String>,
    path_filter: Option<String>,
) -> Result<Vec<CommitInfo>, git2::Error> {
    let repo = match Repository::discover(".") {
    Ok(repo) => repo,
    Err(_) => {
        eprintln!("❌ No Git repository found. Please run inside a Git repo.");
        std::process::exit(1);
    }
};


    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let now = Utc::now().timestamp();
    let threshold = since_date.or_else(|| since_days.map(|d| now - (d as i64 * 86400)));
    let mut commits = Vec::new();

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let time = commit.time().seconds();

        if let Some(t) = threshold {
            if time < t {
                continue;
            }
        }

        let author = commit.author().name().unwrap_or("unknown").to_string();
        let tree = commit.tree()?;
        let parent_tree = commit.parents().next().map(|p| p.tree().ok()).flatten();

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
        let mut files = Vec::new();

        diff.print(DiffFormat::NameOnly, |_, _, line| {
            let filepath = String::from_utf8_lossy(line.content()).trim().to_string();
            let path_matches = path_filter.as_ref().map_or(true, |p| filepath.contains(p));
            let ext_matches = ext_filter.as_ref().map_or(true, |ext| filepath.ends_with(ext));
            if path_matches && ext_matches {
                files.push(filepath);
            }
            true
        })?;

        commits.push(CommitInfo { author, files, timestamp: time });
    }

    Ok(commits)
}
