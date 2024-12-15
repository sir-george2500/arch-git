use std::env;
mod init;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "init" {
        eprintln!("Usage: cargo run init <project-name>");
        return;
    }

    let repo_name = &args[2];
    if let Err(e) = init::init_repo(repo_name) {
        eprintln!("Failed to initialize repository: {}", e);
    }
}
