mod command;
mod issue;
mod parse;
mod request;

use std::collections::HashMap;

use request::Request;

fn main() {
    if !command::is_git_repo() {
        panic!("Must be in a git directory!");
    }

    let request = Request::new();
    let files = command::get_tracked_files();
    let mut file_to_issues = HashMap::new();

    let issues = request.get_issues().expect("Failed to get issues");

    for path in files {
        let result = parse::read_file(&path, &issues, &request);
        file_to_issues
            .insert(path.clone(), parse::handle_result(&path, result));
    }
}
