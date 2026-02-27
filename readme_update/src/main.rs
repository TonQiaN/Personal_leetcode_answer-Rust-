use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::Path;

fn count_solved(root: &Path, prefix: &str) -> Result<usize> {
    let mut count = 0usize;

    let entries = fs::read_dir(root)
        .with_context(|| format!("Failed to read directory: {}", root.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("Failed to read entry under {}", root.display()))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        if !name.starts_with(prefix) {
            continue;
        }

        if path.join("src").join("main.rs").is_file() {
            count += 1;
        }
    }

    Ok(count)
}

fn looks_like_repo_root(dir: &Path) -> bool {
    let has_readme = dir.join("README.md").is_file();
    let has_problem_dir = fs::read_dir(dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .any(|entry| {
            let p = entry.path();
            if !p.is_dir() {
                return false;
            }
            let Some(name) = p.file_name().and_then(|s| s.to_str()) else {
                return false;
            };
            name.starts_with("leetcode_") || name.starts_with("niuke_")
        });

    has_readme && has_problem_dir
}

fn resolve_repo_root() -> Result<std::path::PathBuf> {
    // Find repo root from command line argument
    if let Some(arg_path) = env::args().nth(1) {
        let custom = Path::new(&arg_path).to_path_buf();
        if looks_like_repo_root(&custom) {
            return Ok(custom);
        }
        bail!("Provided path is not a valid repo root: {}", custom.display());
    }

    // Try current working directory
    let current = env::current_dir().context("Failed to get current working directory")?;
    if looks_like_repo_root(&current) {
        return Ok(current);
    }

    // Try parent directory
    if let Some(parent) = current.parent() {
        if looks_like_repo_root(parent) {
            return Ok(parent.to_path_buf());
        }
    }

    // If all else fails, bail
    bail!(
        "Cannot find repo root. Run this program in repo root/readme_update, or pass repo path as the first argument."
    )
}

fn main() -> Result<()> {
    // Resolve repo root
    let repo_root = resolve_repo_root()?;

    // Resolve README.md path
    let readme_path = repo_root.join("README.md");

    // Check if README.md exists
    if !readme_path.is_file() {
        bail!("README.md not found at {}", readme_path.display());
    }

    // Count solved problems
    let leetcode_count = count_solved(&repo_root, "leetcode_")?;
    let niuke_count = count_solved(&repo_root, "niuke_")?;

    let old_content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read {}", readme_path.display()))?;
    let mut found_leetcode_line = false;
    let mut found_niuke_line = false;
    let mut new_lines = Vec::new();

    for line in old_content.lines() {
        if line.contains("**LeetCode solved:**") {
            new_lines.push(format!(
                "- **LeetCode solved:** `{}` problems",
                leetcode_count
            ));
            found_leetcode_line = true;
            continue;
        }

        if line.contains("**Niuke solved:**") {
            new_lines.push(format!("- **Niuke solved:** `{}` problems", niuke_count));
            found_niuke_line = true;
            continue;
        }

        new_lines.push(line.to_string());
    }

    if !found_leetcode_line || !found_niuke_line {
        bail!(
            "README.md is missing one of these lines: '**LeetCode solved:**' or '**Niuke solved:**'"
        );
    }

    let mut new_content = new_lines.join("\n");
    if old_content.ends_with('\n') {
        new_content.push('\n');
    }

    if new_content != old_content {
        fs::write(&readme_path, new_content)
            .with_context(|| format!("Failed to write {}", readme_path.display()))?;
        println!(
            "Updated README.md counts: LeetCode = {}, Niuke = {}",
            leetcode_count, niuke_count
        );
    } else {
        println!(
            "README.md is already up to date: LeetCode = {}, Niuke = {}",
            leetcode_count, niuke_count
        );
    }

    Ok(())
}
