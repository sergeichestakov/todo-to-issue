use std::path::Path;

use request::Request;

mod command;
mod parse;
mod request;

fn main() {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }

    let request = Request::new();
    let files = command::get_tracked_files();

    for path in files {
        if let Err(e) = parse::read_file(&path, &request) {
            println!("Failed to read file {}. Received error {}", path, e);
        }
    }
}
