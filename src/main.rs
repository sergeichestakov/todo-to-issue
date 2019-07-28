mod cli;
mod command;
mod issue;
mod parse;
mod request;

use request::Request;

fn main() {
    let args = match cli::init() {
        Some(args) => args,
        None => return,
    };
    let remote = match command::get_remote_name() {
        Some(remote) => remote,
        None => return,
    };

    let request = Request::new(args.get_token(), remote);
    let issues = match request.get_issues(args.is_dry_run()) {
        Some(issues) => issues,
        None => return,
    };

    let files = command::get_tracked_files();
    let file_to_issues = parse::find_all_todos(
        &files,
        &issues,
        args.get_pattern(),
        args.is_verbose(),
    );

    if file_to_issues.len() > 0 && !args.is_dry_run() {
        cli::output_issues_and_prompt_user(&request, &file_to_issues);
    }
}
