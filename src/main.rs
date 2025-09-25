use clap::Parser;

mod audit;
mod git;

/// A git extension to verify proper preservation of git history
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The URL of the source repository to audit
    source_repo_url: String,

    /// The path to the target repository (should be ".")
    target_repo_path: String,

    /// The relative path to the subtree within the target repository
    subtree_path: String,
}

fn main() {
    let cli = Cli::parse();

    println!("Auditing source repo: {}", cli.source_repo_url);
    println!("Auditing subtree path: {}", cli.subtree_path);

    // 1. Get the commit count from the remote source repository
    let source_count = match git::get_remote_commit_count(&cli.source_repo_url) {
        Ok(count) => {
            println!("Source repository has {} commits.", count);
            count
        }
        Err(e) => {
            eprintln!("Error getting source commit count: {}", e);
            std::process::exit(1);
        }
    };

    // 2. Get the commit count from the local subtree
    let subtree_count = match git::get_local_subtree_commit_count(&cli.subtree_path) {
        Ok(count) => {
            println!("Subtree has {} commits.", count);
            count
        }
        Err(e) => {
            eprintln!("Error getting subtree commit count: {}", e);
            std::process::exit(1);
        }
    };

    // 3. Compare the counts and report the result
    let audit_result = audit::compare_commit_counts(source_count, subtree_count);

    match audit_result {
        audit::AuditResult::Match => {
            println!("✅ Success: Commit counts match!");
        }
        audit::AuditResult::SourceHasMore(diff) => {
            println!(
                "❌ Failure: Source repository has {} more commits than the subtree.",
                diff
            );
        }
        audit::AuditResult::SubtreeHasMore(diff) => {
            println!(
                "⚠️ Warning: Subtree has {} more commits than the source repository.",
                diff
            );
        }
    }
}
