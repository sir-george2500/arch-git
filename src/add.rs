use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn add(file_paths: &[&str]) -> io::Result<()> {
    // Check if the .git repository exists
    let git_dir = Path::new(".git");
    println!(".git exists: {}", git_dir.exists()); // Debug print to check if .git exists

    if !git_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            ".git directory not found. Are you in a git repository?",
        ));
    }

    // The rest of the function remains unchanged
    let object_dir = git_dir.join("objects");
    let index = git_dir.join("index");

    for file_path in file_paths {
        // Process files (same as before)
        let mut file = File::open(&file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let mut hasher = Sha1::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        let hash_hex = format!("{:x}", hash);

        let sub_dir = object_dir.join(&hash_hex[0..2]);
        let object_file = sub_dir.join(&hash_hex[2..]);

        if !sub_dir.exists() {
            fs::create_dir_all(&sub_dir)?;
        }

        if !object_file.exists() {
            let object_header = format!("blob {}\0", content.len());
            let mut object_data = Vec::new();
            object_data.extend(object_header.as_bytes());
            object_data.extend(&content);

            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&object_data)?;
            let compressed_data = encoder.finish()?;

            let mut object_file = File::create(&object_file)?;
            object_file.write_all(&compressed_data)?;
        }

        let relative_path = Path::new(file_path).to_str().unwrap();
        let index_entry = format!("{} {}\n", hash_hex, relative_path);

        let mut index_file = OpenOptions::new().create(true).append(true).open(&index)?;
        index_file.write_all(index_entry.as_bytes())?;

        println!("Added {} to the index.", file_path);
    }
    Ok(())
}
