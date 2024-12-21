use sha1::{Digest, Sha1};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Function to compute the SHA-1 hash of a file
fn compute_hash(file_path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

/// Get the current branch name from .git/HEAD
fn get_current_branch() -> io::Result<String> {
    let head_path = Path::new(".git/HEAD");
    let mut head_file = fs::File::open(head_path)?;
    let mut content = String::new();
    head_file.read_to_string(&mut content)?;

    if content.starts_with("ref: ") {
        let branch = content.trim_start_matches("ref: refs/heads/").trim();
        Ok(branch.to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "HEAD file is not pointing to a branch.",
        ))
    }
}

/// Parse the .git/index file
fn parse_index() -> io::Result<Vec<(String, String)>> {
    let index_path = Path::new(".git/index");
    if !index_path.exists() {
        return Ok(Vec::new()); // No index file means no staged files
    }

    let mut file = fs::File::open(index_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let mut entries = Vec::new();

    // For simplicity, assume each line in the index is formatted as "hash path"
    for line in String::from_utf8_lossy(&content).lines() {
        if let Some((hash, path)) = line.split_once(' ') {
            entries.push((hash.to_string(), path.to_string()));
        }
    }
    Ok(entries)
}

/// Walk the directory tree and collect file paths
fn walk_directory(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            files.extend(walk_directory(&path)?);
        }
    }
    Ok(files)
}

/// Implementation of the status command
pub fn status() -> io::Result<()> {
    let current_branch = get_current_branch()?;
    println!("On branch {}\n", current_branch);

    let index_entries = parse_index()?;
    let index_paths: Vec<String> = index_entries.iter().map(|(_, path)| path.clone()).collect();
    let _index_hashes: Vec<String> = index_entries.iter().map(|(hash, _)| hash.clone()).collect();

    let working_files = walk_directory(Path::new("."))?;
    let mut untracked_files = Vec::new();
    let mut changed_files = Vec::new();

    for file in working_files {
        let relative_path = file
            .strip_prefix(".")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if let Ok(hash) = compute_hash(&file) {
            if let Some(index_hash) = index_entries
                .iter()
                .find(|(_, path)| path == &relative_path)
                .map(|(hash, _)| hash)
            {
                if &hash != index_hash {
                    changed_files.push(relative_path);
                }
            } else {
                untracked_files.push(relative_path);
            }
        }
    }

    // Files in the index but missing in the working directory
    let deleted_files: Vec<_> = index_paths
        .iter()
        .filter(|path| !Path::new(path).exists())
        .collect();

    // Print staged changes
    if !changed_files.is_empty() || !deleted_files.is_empty() {
        println!("Changes to be committed:");
        for file in &changed_files {
            println!("\tmodified:   {}", file);
        }
        for file in &deleted_files {
            println!("\tdeleted:    {}", file);
        }
    }

    // Print untracked files
    if !untracked_files.is_empty() {
        println!("\nUntracked files:");
        for file in &untracked_files {
            println!("\t{}", file);
        }
    }

    Ok(())
}

