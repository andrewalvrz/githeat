use git2::{Repository, DiffFormat};
use chrono::Utc;

pub struct CommitInfo {
    pub author: String,
    pub files: Vec<String>,
    pub timestamp: i64,
}

pub fn parse_repo(since_days: Option<u32>) -> Result<Vec<CommitInfo>, git2::Error> {
    let repo = Repository::discover(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let threshold = since_days.map(|d| Utc::now().timestamp() - (d as i64 * 86400));
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
            files.push(String::from_utf8_lossy(line.content()).trim().to_string());
            true
        })?;

        commits.push(CommitInfo { author, files, timestamp: time });
    }

    Ok(commits)
}
