use std::collections::HashMap;

use clap::{App, Arg};
use dialoguer::{theme::ColorfulTheme, Confirmation, Editor, Select};

use super::command;
use super::issue;
use super::request;

use issue::Issue;
use request::Request;

const OPEN: usize = 0;
const EDIT: usize = 1;
const SKIP: usize = 2;
const EXIT: usize = 3;

pub struct Args {
    directory: String,
    token: String,
}

impl Args {
    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_directory(&self) -> String {
        self.directory.clone()
    }
}

pub fn init() -> Args {
    if !command::is_git_repo() {
        panic!("Must be in a git directory!");
    }

    let matches = App::new("todo-to-issue")
        .version("0.1")
        .author("Sergei Chestakov <sergei332@gmail.com>")
        .about("Converts TODO comments to GitHub issues")
        .arg(
            Arg::with_name("DIRECTORY")
                .help("Sets which directory to look through")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("TOKEN")
                .help("Sets the token for user")
                .takes_value(true),
        )
        .get_matches();

    let token = matches.value_of("token").unwrap_or("");

    let directory = matches.value_of("DIRECTORY").unwrap_or("./").to_string();

    let access_token = if token.is_empty() {
        command::read_access_token()
    } else {
        token.to_string()
    };

    Args {
        token: access_token,
        directory: directory,
    }
}

pub fn output_and_send_issues(
    request: &Request,
    map: &HashMap<String, Vec<Issue>>,
) {
    let selections = &["Open Issue", "Edit Issue", "Skip", "Exit"];

    for (file, issues) in map {
        for issue in issues {
            println!("Found issue in file {}:", &file);
            println!("{}", &issue.to_string());

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .default(0)
                .items(&selections[..])
                .interact()
                .unwrap();

            match selection {
                OPEN => println!("OPENING"),
                EDIT => edit_issue(&issue),
                SKIP => continue,
                EXIT => return,
                _ => (),
            }
        }
    }
}

pub fn prompt_to_continue() -> bool {
    Confirmation::new()
        .with_text("Do you want to continue?")
        .interact()
        .expect("Failed to read confirmation")
}

fn edit_issue(issue: &Issue) {
    let result = Editor::new().edit(&issue.to_string()).unwrap();

    if let Some(input) = result {
        match Issue::from_string(input) {
            Some(issue) => println!("Created issue:\n{}", issue.to_string()),
            None => println!(
                "Failed to create issue. Please review format and try again."
            ),
        }
    } else {
        println!("Abort!");
    }
}
