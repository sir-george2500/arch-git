use std::env;
use std::fs;
use std::io::Write;

pub fn init_repo(repo_name: &str) -> std::io::Result<()> {
    let cwd = env::current_dir()?;
    let repo_path = cwd.join(repo_name);

    // create the current the main repository directory
    fs::create_dir_all(&repo_path)?;

    // create the .git directory
    let git_path = repo_path.join(".git");
    fs::create_dir_all(&git_path)?;

    // create subdirectories in the .git directory
    let objects_path = git_path.join("objects");
    fs::create_dir_all(&objects_path)?;

    let refs_path = git_path.join("refs");
    fs::create_dir_all(&refs_path)?;

    let hooks_path = git_path.join("hooks");
    fs::create_dir_all(&hooks_path)?;

    let info_path = git_path.join("info");
    fs::create_dir_all(&info_path)?;

    let heads_path = git_path.join("HEAD");
    let mut head_file = fs::File::create(&heads_path)?;
    head_file.write_all(b"ref: refs/heads/master\n")?;

    // create the config file
    let config_path = git_path.join("config");
    let mut config_file = fs::File::create(&config_path)?;
    config_file.write_all(b"[core]\nrepositoryformatversion = 0\nfilemode = true\nbare = false\nlogallrefupdates = true\n")?;

    // create the description file
    let description_path = git_path.join("description");
    let mut description_file = fs::File::create(&description_path)?;
    write!(
        description_file,
        "Unnamed repository; edit this file 'description' to name the repository."
    )?;

    println!(
        "Initialized empty Git repository in {}",
        repo_path.display()
    );

    Ok(())
}
