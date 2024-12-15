use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn add(file_paths: &[&str]) -> io::Result<()> {
    //check if the .git repository exist
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            ".git directory not found. Are you in a git repository?",
        ));
    }

    let object_dir = git_dir.joint("objects");
    let index = git_dir.joint("index");

    for file_path in file_paths {
        // Read all the content in the file path
        let mut file = File::open(&file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        // Compute the SHA-1 Hash
        let mut hasher = Sha1::new();
        hasher.update(&content);
        let hash = htasher.finalize();
        let hash_hex = format("{:x}", hash);
    }
}
