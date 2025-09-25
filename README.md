## An extension to verify proper preservation of git history 

### Pitch

Let A and B be repositories. Say you are merging A into B, and you want to preserve git history of B after its been merged in (with some non-squashing, merging git command). There are about 10,235 commits in A, its rather troubling to cross reference that many commits. Is there a better way to confirm that I've preserved the entire history of A in B? 

### Introducing Git Subtree Audit 🌲

**git-subtree-audit** is a lightweight Git extension that verifies subtree merges automatically. Instead of manually counting or scanning thousands of commits, it codifies invariants about history preservation into a single CLI command.

**Exact verification:** Ensures every commit from repo A exists in repo B’s subtree history.
**Flexible checks:** Supports both strict hash matching and patch-ID checks (to account for rewritten commits).
**Simple usage:** Just point it to your source repo, target repo, and the subtree path — the tool handles the rest.

### Why this matters

**Confidence:** No need to trust manual inspection — the tool guarantees no history is lost.
**Productivity:** Reduce merge review time from hours (or days) to seconds.
**Reliability:** Especially critical when merging SDKs, shared libraries, or repos with sensitive audit requirements.


### Installation 

Run ```git subtree-audit``` in your terminal 

### Usage

Let’s say you have two repos:

repo A → the path of source repo you want to merge.

repo B → the target repo that contains a subtree copy of A.

sdk/ → the path inside Repo B where Repo A was merged.

In the root of repo B, run ```git subtree-audit <github-url-of-repo-A> . <path-to-merged-in-repo-relative-to-root>```