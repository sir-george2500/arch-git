use arch_git::add::add;
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn setup_git_dir() {
    let git_dir = ".git";
    fs::create_dir_all(format!("{}/objects", git_dir)).unwrap();
    File::create(format!("{}/index", git_dir)).unwrap();
}

fn cleanup_git_dir(git_dir: &str) {
    if Path::new(git_dir).exists() {
        fs::remove_dir_all(git_dir).unwrap();
    }
}

fn create_test_file(path: &str, content: &str) -> String {
    let mut file = File::create(path).unwrap();
    writeln!(file, "{}", content.trim_end()).unwrap();

    let mut hasher = Sha1::new();
    hasher.update(format!("{}\n", content.trim_end()).as_bytes());
    format!("{:x}", hasher.finalize())
}

#[test]
fn test_add_file_to_index() {
    // Set up
    let git_dir = ".git";
    setup_git_dir();

    // Create test file
    let file_path = "test_file.txt";
    let content = "Hello, Git!";
    let hash = create_test_file(file_path, content);

    let object_file = format!("{}/objects/{}/{}", git_dir, &hash[0..2], &hash[2..]);
    let index_file = format!("{}/index", git_dir);

    // Test add function
    add(&[file_path]).expect("Failed to add file to index");

    // Verify results
    assert!(
        Path::new(&object_file).exists(),
        "Object file was not created"
    );
    let index_content = fs::read_to_string(&index_file).unwrap();
    assert!(
        index_content.contains(&format!("{} {}", hash, file_path)),
        "Index file doesn't contain correct entry"
    );

    // Clean up
    fs::remove_file(file_path).unwrap();
    cleanup_git_dir(git_dir);
}

#[test]
fn test_add_with_no_git_directory() {
    // Ensure no .git directory exists
    cleanup_git_dir(".git");

    // Create test file
    let file_path = "test_file.txt";
    create_test_file(file_path, "Hello, Git!");

    // Test add function
    let result = add(&[file_path]);

    // Verify error
    assert!(result.is_err(), "Expected error but got success");
    assert_eq!(
        result.unwrap_err().to_string(),
        ".git directory not found. Are you in a git repository?"
    );

    // Clean up
    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_add_multiple_files() {
    // Set up
    let git_dir = ".git";
    setup_git_dir();

    // Create test files
    let file1_path = "test_file1.txt";
    let file2_path = "test_file2.txt";
    let hash1 = create_test_file(file1_path, "Content 1");
    let hash2 = create_test_file(file2_path, "Content 2");

    // Test add function
    add(&[file1_path, file2_path]).expect("Failed to add multiple files");

    // Verify results
    let index_content = fs::read_to_string(format!("{}/index", git_dir)).unwrap();
    assert!(
        index_content.contains(&format!("{} {}", hash1, file1_path)),
        "First file not found in index"
    );
    assert!(
        index_content.contains(&format!("{} {}", hash2, file2_path)),
        "Second file not found in index"
    );

    // Clean up
    fs::remove_file(file1_path).unwrap();
    fs::remove_file(file2_path).unwrap();
    cleanup_git_dir(git_dir);
}

#[test]
fn test_add_empty_file() {
    // Set up
    let git_dir = ".git";
    setup_git_dir();

    // Create empty test file
    let file_path = "empty_file.txt";
    let hash = create_test_file(file_path, "");

    // Test add function
    add(&[file_path]).expect("Failed to add empty file");

    // Verify results
    let index_content = fs::read_to_string(format!("{}/index", git_dir)).unwrap();
    assert!(
        index_content.contains(&format!("{} {}", hash, file_path)),
        "Empty file not found in index"
    );

    // Clean up
    fs::remove_file(file_path).unwrap();
    cleanup_git_dir(git_dir);
}

#[test]
fn test_add_nonexistent_file() {
    // Set up
    setup_git_dir();

    // Test add function with nonexistent file
    let result = add(&["nonexistent.txt"]);

    // Verify error
    assert!(result.is_err(), "Expected error for nonexistent file");

    // Clean up
    cleanup_git_dir(".git");
}

