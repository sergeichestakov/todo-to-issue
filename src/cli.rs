use std::collections::HashMap;

use clap::{App, Arg};
use console::style;
use dialoguer::{theme::ColorfulTheme, Editor, Select};
use glob::Pattern;

use super::command;
use super::issue;
use super::request;

use issue::Issue;
use request::Request;

const SELECTIONS: &[&str] = &["Open Issue", "Edit Issue", "Skip Issue", "Exit"];
const ALL_FILES: &str = "*";

const OPEN: usize = 0;
const EDIT: usize = 1;
const SKIP: usize = 2;

pub struct Args {
    pattern: Pattern,
    token: String,
    is_dry_run: bool,
    is_verbose: bool,
}

impl Args {
    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_pattern(&self) -> &Pattern {
        &self.pattern
    }

    pub fn is_dry_run(&self) -> bool {
        self.is_dry_run
    }

    pub fn is_verbose(&self) -> bool {
        self.is_verbose
    }
}

pub fn init() -> Option<Args> {
    //! Initializes the CLI and parses command line arguments.
    //!
    //! Returns an Option containing the Args as a struct or None
    //! if the user is not in a git repo.
    let matches = App::new("todo-to-issue")
        .version("0.1.1")
        .author("Sergei Chestakov <sergei332@gmail.com>")
        .about("Converts TODO comments into GitHub issues")
        .arg(
            Arg::with_name("token")
                .help("Sets the token for user")
                .index(1),
        )
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .value_name("PATTERN")
                .help("Sets a glob pattern to narrow search for TODO comments")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dry-run")
                .short("n")
                .long("dry-run")
                .help("Outputs the number of TODOs without opening any issues"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Makes output more descriptive"),
        )
        .get_matches();

    if !command::is_git_repo() {
        print_error("Must be in a git repository.");
        return None;
    }

    let is_dry_run = matches.is_present("dry-run");
    let is_verbose = matches.is_present("verbose");

    let pattern_value = matches.value_of("pattern").unwrap_or(ALL_FILES);

    let pattern = match Pattern::new(pattern_value) {
        Ok(pattern) => pattern,
        Err(_) => Pattern::new(ALL_FILES).unwrap(),
    };

    let token = match matches.value_of("token") {
        Some(t) => t.to_string(),
        None => match is_dry_run {
            true => String::new(),
            false => command::read_access_token(),
        },
    };

    return Some(Args {
        pattern,
        token,
        is_dry_run,
        is_verbose,
    });
}

pub fn output_issues_and_prompt_user(
    request: &Request,
    map: &HashMap<String, Vec<Issue>>,
) {
    //! Outputs every todo comment found and prompts the user for action.
    //!
    //! Allows the user to
    //! - Open a GitHub issue
    //! - Edit the body or title before opening
    //! - Skip to the next one
    //! - Exit the program
    for (_file, issues) in map {
        for issue in issues {
            println!("\n{}", &issue.to_formatted_string());

            let prompt =
                format!("{}", style("What would you like to do?").italic())
                    .to_string();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&prompt)
                .default(0)
                .items(&SELECTIONS[..])
                .interact()
                .unwrap();

            let done = match selection {
                OPEN => open_issue(&request, &issue),
                EDIT => edit_issue(&request, &issue),
                SKIP => false,
                _ => true,
            };
            if done {
                return;
            }
        }
    }

    print_success("All done!");
}

pub fn print_success(msg: &str) {
    println!("{}", style(msg).green());
}

pub fn print_warning(msg: &str) {
    println!("{}", style(msg).yellow());
}

pub fn print_error(msg: &str) {
    println!("{} {}", style("ERROR:").red(), style(msg).red());
}

pub fn print_dim(msg: &str) {
    println!("{}", style(msg).dim());
}

pub fn handle_plural(number: &usize, word: &str) -> String {
    match number {
        1 => word.to_string(),
        _ => format!("{}s", word).to_string(),
    }
}

fn edit_issue(request: &Request, issue: &Issue) -> bool {
    //! Opens the user's default editor and allows them to edit an issue's
    //! title and body before opening it.
    //!
    //! Creates an issue on GitHub if the format is valid
    //! (see Issue::from_string) and the user saves and quits.
    //! Aborts the operation if the user exits without saving.
    //! Returns a bool indicating whether or not to terminate the program.
    let result = Editor::new().edit(&issue.to_string()).unwrap();

    match result {
        Some(input) => match Issue::from_string(input) {
            Some(new_issue) => {
                return open_issue(request, &new_issue);
            }
            None => print_warning("Invalid format. Not creating issue."),
        },
        None => {
            print_warning("Editor closed without saving. Not creating issue.")
        }
    }

    false
}

fn open_issue(request: &Request, issue: &Issue) -> bool {
    //! Creates the GitHub issue and outputs the result.
    //! Returns a bool indicating whether or not to terminate the program.
    match request.open_issue(issue) {
        Some(issue_number) => {
            let success_msg = format!(
                "Successfully opened issue #{}: \"{}\"",
                issue_number,
                issue.get_title()
            )
            .to_string();

            print_success(&success_msg);
            false
        }
        None => true,
    }
}
