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

    // 1. Get the commit count from the remote source repository
    match git::get_remote_commit_count(&cli.source_repo_url) {
        Ok(count) => {
            println!("Source repository has {} commits.", count);
            // TODO: Pass this count to the audit module
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    // 2. TODO: Get the commit count from the subtree in the target repo
    //    - Create a new function in `git.rs` for this.
    //    - It should run `git log <subtree_path> --oneline` and count the lines.

    // 3. TODO: Compare the counts and report the result
}
