use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

use super::cli;
use super::issue;
use console::style;
use issue::Issue;

// const TODO: &str = "TODO";

pub fn find_all_labels(
    files: &Vec<String>,
    issues: &HashSet<String>,
    pattern: &glob::Pattern,
    is_verbose: bool,
    label: &String,
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
        println!(
            "Searching all files tracked by git for {} comments...",
            label
        );
    } else {
        println!(
            "Searching all files matching pattern \"{}\" for {}comments...",
            pattern_str, label,
        );
    }

    for file in files {
        if pattern.matches(&file) {
            let result =
                find_labels_in_file(&file, &issues, is_verbose, &label);
            if let Ok(vector) = result {
                let num_issues = vector.len();
                if num_issues > 0 {
                    println!(
                        "Found {} {} in {}",
                        style(num_issues).bold(),
                        cli::handle_plural(&num_issues, &label),
                        style(file).italic()
                    );
                    file_to_issues.insert(file.clone(), vector);
                    total += num_issues;
                }
            }
        }
    }

    match total {
        0 => {
            cli::print_success(&format!("No {}s found. You're all set!", label))
        }
        num_issues => println!(
            "Found {} {} total.",
            style(num_issues).bold(),
            cli::handle_plural(&num_issues, &format!("{}", label))
        ),
    }

    file_to_issues
}

fn find_labels_in_file(
    path: &str,
    prev_issues: &HashSet<String>,
    is_verbose: bool,
    label: &String,
) -> io::Result<Vec<Issue>> {
    //! Reads every line in a file for a "todo" comment, creating an Issue
    //! object for each one with the parsed title and description.
    //!
    //! Returns an IO result containing a vector of Issues if successful.
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let mut issues_in_file = Vec::new();

    if is_verbose {
        cli::print_dim(&format!("Searching {}", path).to_string());
    }
    let mut line_number = 0;
    for line_result in buffer.lines() {
        if let Err(e) = line_result {
            return Err(e);
        }
        line_number += 1;

        let line = line_result.unwrap();
        if contains_label(&line, &label) {
            let title = extract_title(&line, &label);
            let body = create_body(&line_number, path, &label);

            if is_verbose {
                println!("Line {}: \"{}\"", &line_number, title)
            }
            if !prev_issues.contains(title.as_str()) {
                let issue = Issue::new(title, body);
                issues_in_file.push(issue);
            } else if is_verbose {
                cli::print_warning(
                    "This issue was already opened in the remote repo.",
                );
            }
        }
    }

    Ok(issues_in_file)
}

fn contains_label(line: &str, label: &String) -> bool {
    //! Returns if a line contains a todo comment.
    //! Looks for both C and Bash style comments.
    let comment = match line.find("//") {
        Some(value) => Some(value),
        None => line.find("#"),
    };

    let a_label = line.find(&*label);
    if comment.is_some() && a_label.is_some() {
        let comment_index = comment.unwrap();
        let label_index = a_label.unwrap();
        return label_index > comment_index;
    }

    false
}

fn extract_title(line: &str, label: &String) -> String {
    //! Parses a line containing a todo comment and returns the
    //! remainder of the String after "todo" to be used as the title of a
    //! new GitHub issue.
    //FIXME: some issue here
    let vec: Vec<&str> = line.split(&*label).collect();
    let after_label = vec[1];
    let title = if after_label.starts_with(":") {
        &after_label[1..]
    } else {
        after_label
    }
    .trim();

    title.to_string()
}

fn create_body(line_number: &u32, file_path: &str, label: &String) -> String {
    //! Creates a generic description for a new GitHub issue
    //! based on a "todo" comment.
    format!(
        "Found a {} comment on line {} of file {}",
        label, line_number, file_path
    )
    .to_string()
}
