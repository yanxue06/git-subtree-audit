#[derive(Debug, PartialEq)]
pub enum AuditResult {
    Match,
    SourceHasMore(usize), // Source has X more commits
    SubtreeHasMore(usize), // Subtree has X more commits
}

pub fn compare_commit_counts(source_count: usize, subtree_count: usize) -> AuditResult {
    if source_count == subtree_count {
        AuditResult::Match
    } else if source_count > subtree_count {
        AuditResult::SourceHasMore(source_count - subtree_count)
    } else {
        AuditResult::SubtreeHasMore(subtree_count - source_count)
    }
}

