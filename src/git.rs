use std::process::{Command, Stdio};
use tempfile::Builder;

// ... (existing get_remote_commit_count function) ...

pub fn get_local_subtree_commit_count(subtree_path: &str) -> Result<usize, String> {
    // Step 1: Find the merge commit hash created by git subtree
    let merge_commit_output = Command::new("git")
        .arg("log")
        .arg(&format!("--grep=git-subtree-dir: {}", subtree_path))
        .arg("--format=%H")
        .arg("-n")
        .arg("1")
        .output()
        .map_err(|e| format!("Failed to execute git log: {}", e))?;

    if !merge_commit_output.status.success() {
        let error_message = String::from_utf8_lossy(&merge_commit_output.stderr);
        return Err(format!(
            "git log for subtree merge commit failed: {}",
            error_message
        ));
    }

    let merge_commit_hash = String::from_utf8_lossy(&merge_commit_output.stdout)
        .trim()
        .to_string();
    if merge_commit_hash.is_empty() {
        return Err(format!(
            "No git subtree merge commit found for path: {}",
            subtree_path
        ));
    }

    // Step 2: Resolve the second parent hash of the merge commit
    let second_parent_output = Command::new("git")
        .arg("rev-parse")
        .arg(&format!("{}^2", merge_commit_hash))
        .output()
        .map_err(|e| format!("Failed to execute git rev-parse: {}", e))?;

    if !second_parent_output.status.success() {
        let error_message = String::from_utf8_lossy(&second_parent_output.stderr);
        return Err(format!(
            "git rev-parse for second parent failed: {}",
            error_message
        ));
    }

    let second_parent_hash = String::from_utf8_lossy(&second_parent_output.stdout)
        .trim()
        .to_string();

    let count_output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(&second_parent_hash)
        .output()
        .map_err(|e| format!("Failed to execute git rev-list: {}", e))?;

    if !count_output.status.success() {
        let error_message = String::from_utf8_lossy(&count_output.stderr);
        return Err(format!(
            "git rev-list for commit count failed: {}",
            error_message
        ));
    }

    let count_str = String::from_utf8_lossy(&count_output.stdout)
        .trim()
        .to_string();
    let commit_count = count_str
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse subtree commit count: {}", e))?;

    Ok(commit_count)
}

pub fn get_remote_commit_count(repo_url: &str) -> Result<usize, String> {
    let temp_dir = Builder::new()
        .prefix("git-subtree-audit-")
        .tempdir()
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;

    let clone_status = Command::new("git")
        .arg("clone")
        .arg("--bare") 
        .arg(repo_url)
        .arg(temp_dir.path())
        .stdout(Stdio::null()) 
        .stderr(Stdio::null()) 
        .status()
        .map_err(|e| format!("Failed to execute git clone: {}", e))?;

    if !clone_status.success() {
        return Err(format!(
            "Failed to clone repository: {}. Check URL and permissions.",
            repo_url
        ));
    }

    let output = Command::new("git")
        .current_dir(temp_dir.path())
        .arg("rev-list")
        .arg("--count")
        .arg("HEAD")
        .output()
        .map_err(|e| format!("Failed to execute git rev-list: {}", e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git rev-list failed: {}", error_message));
    }

    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let commit_count = count_str
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse commit count: {}", e))?;

    Ok(commit_count)
}
