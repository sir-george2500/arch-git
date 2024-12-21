use std::env;
mod add;
mod init;
mod status;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <command> [args]");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "init" => {
            if args.len() != 3 {
                eprintln!("Usage: cargo run init <project-name>");
                return;
            }
            let repo_name = &args[2];
            if let Err(e) = init::init_repo(repo_name) {
                eprintln!("Failed to initialize repository: {}", e);
            }
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run add <file-path> [<file-path>...]");
                return;
            }
            let file_paths: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
            if let Err(e) = add::add(&file_paths) {
                eprintln!("Failed to add files to the index: {}", e);
            }
        }
        "status" => {
            if let Err(e) = status::status() {
                eprintln!("Failed to get repository status: {}", e);
            }
        }
        _ => {
            eprintln!(
                "Unknown command: {}. Available commands: init, add, status",
                command
            );
        }
    }
}

