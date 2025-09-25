use std::process::{Command, Stdio};
use tempfile::Builder;

// ... (existing get_remote_commit_count function) ...

pub fn get_local_subtree_commit_count(subtree_path: &str) -> Result<usize, String> {
    let output = Command::new("git")
        .arg("rev-list")
        .arg("--all")
        .arg("--count")
        .arg(subtree_path) // Count commits only for this path
        .output()
        .map_err(|e| format!("Failed to execute git rev-list: {}", e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "git rev-list for subtree failed: {}",
            error_message
        ));
    }

    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let commit_count = count_str
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse subtree commit count: {}", e))?;

    Ok(commit_count)
}

// TODO: Implement utility functions for interacting with Git.
// This file should contain functions for:
// - Getting the list of commits from a repository.
// - Getting patch IDs for commits.
// - Any other Git-related operations needed for the audit.
// Consider using the `git2` crate for future, more advanced features.

pub fn get_remote_commit_count(repo_url: &str) -> Result<usize, String> {
    // Create a temporary directory for the clone
    let temp_dir = Builder::new()
        .prefix("git-subtree-audit-")
        .tempdir()
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;

    // Clone the repository into the temporary directory
    let clone_status = Command::new("git")
        .arg("clone")
        .arg("--bare") // More efficient, no working copy needed
        .arg(repo_url)
        .arg(temp_dir.path())
        .stdout(Stdio::null()) // Suppress success output
        .stderr(Stdio::null()) // Suppress success output
        .status()
        .map_err(|e| format!("Failed to execute git clone: {}", e))?;

    if !clone_status.success() {
        return Err(format!("Failed to clone repository: {}. Check URL and permissions.", repo_url));
    }

    // Get the commit count from the cloned repo
    let output = Command::new("git")
        .current_dir(temp_dir.path())
        .arg("rev-list")
        .arg("--all")
        .arg("--count")
        .output()
        .map_err(|e| format!("Failed to execute git rev-list: {}", e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "git rev-list failed: {}",
            error_message
        ));
    }

    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let commit_count = count_str
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse commit count: {}", e))?;

    Ok(commit_count)
}
