mod cli;
mod command;
mod issue;
mod parse;
mod request;

use std::collections::HashMap;

use request::Request;

fn main() {
    let args = cli::init();
    let directory = args.get_directory();

    let request = Request::new(args.get_token());
    let files = command::get_tracked_files();
    let mut file_to_issues = HashMap::new();

    let issues = request.get_issues().expect("Failed to get issues");

    for path in files {
        if path.starts_with(&directory) {
            let result = parse::find_issues(&path, &issues);
            if let Ok(vector) = result {
                file_to_issues.insert(path.clone(), vector);
            }
        }
    }

    let total = parse::count_issues(&file_to_issues);

    if total > 0 && cli::prompt_to_continue() {
        cli::output_and_send_issues(&request, &file_to_issues);
    }
}
