mod cli;
mod command;
mod issue;
mod parse;
mod request;

use request::Request;

fn main() {
    let args = cli::init();

    let request = Request::new(args.get_token());
    let files = command::get_tracked_files();
    let issues = request.get_issues().expect("Failed to get issues");

    let pattern = args.get_pattern();
    let file_to_issues = parse::populate_map(&files, &issues, pattern);
    let total = parse::count_issues(&file_to_issues);

    if total > 0 && cli::prompt_to_continue() {
        cli::output_and_send_issues(&request, &file_to_issues);
    }
}
