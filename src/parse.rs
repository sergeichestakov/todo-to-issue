use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

use super::cli;
use super::issue;
use console::style;
use issue::Issue;

const TODO: &str = "TODO";

pub fn find_all_todos(
    files: &Vec<String>,
    issues: &HashSet<String>,
    pattern: &glob::Pattern,
    is_verbose: bool,
) -> HashMap<String, Vec<Issue>> {
    //! Reads every file that matches the specified glob pattern
    //! and searches for "todo" comments line by line.
    //!
    //! Returns a HashMap that maps file path to a vector of Issue objects that
    //! represents the "todos" found in the file.
    let mut file_to_issues = HashMap::new();
    let mut total = 0;

    let pattern_str = pattern.as_str();
    if pattern_str == "*" {
        println!("Searching all files tracked by git for TODO comments...");
    } else {
        println!(
            "Searching all files with pattern \"{}\" for TODO comments...",
            pattern_str
        );
    }

    for file in files {
        if pattern.matches(&file) {
            let result = find_todos_in_file(&file, &issues, is_verbose);
            if let Ok(vector) = result {
                let num_issues = vector.len();
                if num_issues > 0 {
                    println!(
                        "Found {} {} in {}",
                        style(num_issues).bold(),
                        cli::handle_plural(&num_issues, "TODO"),
                        style(file).italic()
                    );
                    if is_verbose {
                        println!();
                    }
                    file_to_issues.insert(file.clone(), vector);
                    total += num_issues;
                }
            }
        }
    }

    match total {
        0 => cli::print_success("No TODOs found. You're all set!"),
        num_issues => println!(
            "Found {} {} total.",
            style(num_issues).bold(),
            cli::handle_plural(&num_issues, "TODO")
        ),
    }

    file_to_issues
}

fn find_todos_in_file(
    path: &str,
    prev_issues: &HashSet<String>,
    is_verbose: bool,
) -> io::Result<(Vec<Issue>)> {
    //! Reads every line in a file for a "todo" comment, creating an Issue
    //! object for each one with the parsed title and description.
    //!
    //! Returns an IO result containing a vector of Issues if successful.
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let mut issues_in_file = Vec::new();

    let mut line_number = 0;
    for line_result in buffer.lines() {
        if let Err(e) = line_result {
            return Err(e);
        }
        line_number += 1;

        let line = line_result.unwrap();
        if contains_todo(&line) {
            let title = extract_title(&line);
            let body = create_body(&line_number, path);

            if is_verbose {
                println!("Line {}: \"{}\"", &line_number, title)
            }
            if !prev_issues.contains(title.as_str()) {
                let issue = Issue::new(title, body);
                issues_in_file.push(issue);
            }
        }
    }

    Ok(issues_in_file)
}

fn contains_todo(line: &str) -> bool {
    //! Returns if a line contains a todo comment.
    //! Looks for both C and Bash style comments.
    let comment = match line.find("//") {
        Some(value) => Some(value),
        None => line.find("#"),
    };

    let todo = line.find(TODO);
    if comment.is_some() && todo.is_some() {
        let comment_index = comment.unwrap();
        let todo_index = todo.unwrap();
        return todo_index > comment_index;
    }

    false
}

fn extract_title(line: &str) -> String {
    //! Parses a line containing a todo comment and returns the
    //! remainder of the String after "todo" to be used as the title of a
    //! new GitHub issue.
    let vec: Vec<&str> = line.split(TODO).collect();
    let after_todo = vec[1];
    let title = if after_todo.starts_with(":") {
        &after_todo[1..]
    } else {
        after_todo
    }
    .trim();

    title.to_string()
}

fn create_body(line_number: &u32, file_path: &str) -> String {
    //! Creates a generic description for a new GitHub issue
    //! based on a "todo" comment.
    format!(
        "Found a TODO comment on line {} of file {}",
        line_number, file_path
    )
    .to_string()
}
