mod command;
mod parse;
mod request;

use request::Request;

fn main() {
    if !command::is_git_repo() {
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
