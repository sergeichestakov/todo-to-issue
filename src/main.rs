mod cli;
mod command;
mod issue;
mod parse;
mod request;

use request::Request;
use std::collections::HashSet;

fn main() {
    let args = match cli::init() {
        Some(args) => args,
        None => return,
    };

    let is_dry_run = args.is_dry_run();
    let request = Request::new(args.get_token());
    let issues = if is_dry_run {
        HashSet::new()
    } else {
        request.get_issues().expect("Failed to get issues")
    };

    let pattern = args.get_pattern();
    let files = command::get_tracked_files();
    let file_to_issues =
        parse::find_all_todos(&files, &issues, pattern, args.is_verbose());

    if file_to_issues.len() > 0 && !is_dry_run {
        cli::output_issues_and_prompt_user(&request, &file_to_issues);
    }
}
