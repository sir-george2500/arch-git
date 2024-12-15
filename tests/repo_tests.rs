#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;

    use arch_git::init::init_repo;

    fn clean_up(repo_name: &str) {
        let cwd = env::current_dir().unwrap();
        let repo_path = cwd.join(repo_name);
        if repo_path.exists() {
            fs::remove_dir_all(repo_path).unwrap();
        }
    }

    #[test]
    fn test_init_repo_creates_directories() {
        let repo_name = "test_repo";

        // Ensure the repo does not already exist before starting the test
        clean_up(repo_name);

        // Call the function to initialize the repo
        init_repo(repo_name).unwrap();

        // Check that the main directory is created
        let cwd = env::current_dir().unwrap();
        let repo_path = cwd.join(repo_name);
        assert!(repo_path.exists(), "Repository directory was not created.");

        // Check that the .git directory is created
        let git_path = repo_path.join(".git");
        assert!(git_path.exists(), ".git directory was not created.");

        // Check that subdirectories in .git are created
        let objects_path = git_path.join("objects");
        assert!(objects_path.exists(), "Objects directory was not created.");

        let refs_path = git_path.join("refs");
        assert!(refs_path.exists(), "Refs directory was not created.");

        let hooks_path = git_path.join("hooks");
        assert!(hooks_path.exists(), "Hooks directory was not created.");

        let info_path = git_path.join("info");
        assert!(info_path.exists(), "Info directory was not created.");

        // Check that the HEAD file exists
        let heads_path = git_path.join("HEAD");
        assert!(heads_path.exists(), "HEAD file was not created.");

        // Check the content of the HEAD file
        let head_content = fs::read_to_string(heads_path).unwrap();
        assert_eq!(
            head_content, "ref: refs/heads/master\n",
            "HEAD file content is incorrect."
        );

        // Check that the config file exists
        let config_path = git_path.join("config");
        assert!(config_path.exists(), "Config file was not created.");

        // Check the content of the config file
        let config_content = fs::read_to_string(config_path).unwrap();
        assert!(
            config_content.contains("[core]"),
            "Config file content is incorrect."
        );

        // Check that the description file exists
        let description_path = git_path.join("description");
        assert!(
            description_path.exists(),
            "Description file was not created."
        );

        // Clean up after the test
        clean_up(repo_name);
    }

    #[test]
    fn test_init_repo_handles_existing_directory() {
        let repo_name = "existing_repo";
        let cwd = env::current_dir().unwrap();
        let repo_path = cwd.join(repo_name);

        // Create an existing directory before testing
        fs::create_dir_all(&repo_path).unwrap();

        // Try to initialize a repo in the already existing directory
        let result = init_repo(repo_name);

        // Ensure that the function completes without error
        assert!(
            result.is_ok(),
            "Initialization failed in existing directory."
        );

        // Clean up after the test
        clean_up(repo_name);
    }

    #[test]
    fn test_init_repo_creates_empty_repo() {
        let repo_name = "empty_repo";
        clean_up(repo_name);

        // Initialize the repository
        init_repo(repo_name).unwrap();

        // Verify that the directory is empty except for the .git directory
        let cwd = env::current_dir().unwrap();
        let repo_path = cwd.join(repo_name);
        let entries = fs::read_dir(repo_path).unwrap();
        let entries_count = entries.count();
        assert_eq!(
            entries_count, 1,
            "Repository directory should be empty except for .git."
        );

        // Clean up after the test
        clean_up(repo_name);
    }
}
