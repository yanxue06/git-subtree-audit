## An extension to verify proper preservation of git history 

### Pitch

Let A and B be repositories. Say you are merging A into B, and you want to preserve git history of B after its been merged in (with some non-squashing, merging git command). There are about 10,235 commits in A, its rather troubling to cross reference that many commits. Is there a better way to confirm that I've preserved the entire history of A in B? 

### Introducing Git Subtree Audit 🌲

**git-subtree-audit** is a lightweight Git extension that verifies subtree merges automatically.
Instead of manually counting or scanning thousands of commits, it codifies invariants about history preservation into a single CLI command.

**Exact verification:** Ensures every commit from repo A exists in repo B’s subtree history.

### Why this matters

**Confidence:** No need to trust manual inspection — the tool guarantees no history is lost. 

**Productivity:** Reduce merge review time from hours (or days) to seconds.

**Reliability:** Especially critical when merging SDKs, shared libraries, or repos with sensitive audit requirements.

### **Installation**

#### Option 1: Download Pre-built Binary (Recommended)

Download the latest release from [GitHub Releases](https://github.com/yanxue06/git-subtree-audit/releases).

**macOS (Apple Silicon M1/M2/M3)**
```bash
curl -L https://github.com/yanxue06/git-subtree-audit/releases/latest/download/git-subtree-audit-macos-arm64 -o git-subtree-audit
chmod +x git-subtree-audit
sudo mv git-subtree-audit /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -L https://github.com/yanxue06/git-subtree-audit/releases/latest/download/git-subtree-audit-macos-x86_64 -o git-subtree-audit
chmod +x git-subtree-audit
sudo mv git-subtree-audit /usr/local/bin/
```

**Linux**
```bash
curl -L https://github.com/yanxue06/git-subtree-audit/releases/latest/download/git-subtree-audit-linux-x86_64 -o git-subtree-audit
chmod +x git-subtree-audit
sudo mv git-subtree-audit /usr/local/bin/
```

**Windows**
```powershell
Download from: https://github.com/yanxue06/git-subtree-audit/releases/latest/download/git-subtree-audit-windows-x86_64.exe
# Save to a folder in your PATH, or add the folder to PATH
```

#### Option 2: Install from Cargo

```bash
cargo install --git https://github.com/yanxue06/git-subtree-audit

```

### **Usage**

Just point it to your source repo, target repo, and the subtree path — the tool handles the rest.

For examples, if you have a repo A and a repo B and:

repo A is the path of source repo you want to merge repo B is the target repo that contains a subtree copy of A. sdk/ → the path inside Repo B where Repo A was merged.

In the root of repo B, run ```git subtree-audit <github-url-of-repo-A> . <path-to-merged-in-repo-relative-to-root>```

In this case ```<path-to-merged-in-repo-relative-to-root>``` would just be ```sdk```
