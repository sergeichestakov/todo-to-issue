use std::process::Command;
use std::str;

use dialoguer::PasswordInput;

use super::cli;

pub fn read_access_token() -> String {
    //! Reads in a user's personal access token from GitHub.
    println!("Please paste your personal access token from GitHub below.");
    PasswordInput::new()
        .with_prompt("Token")
        .interact()
        .expect("Failed to read token")
}

pub fn is_git_repo() -> bool {
    //! Returns whether the current repo is a git repository.
    let command = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute `git status`");
    let output = str::from_utf8(&command.stdout).unwrap().trim();

    !output.is_empty()
}

pub fn get_remote_name() -> Option<String> {
    //! Executes the command `git remote get-url origin`.
    //! Parses the result to return a string of the form :username/:repo
    //! if successful. Otherwise, returns None if there is no remote.
    let command = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .expect("Failed to execute `git remote get-url origin`");
    let output = str::from_utf8(&command.stdout).unwrap();
    // Output is of the form https://github.com/:username/:repo.git
    // So we must remove the protocol/domain and .git suffix.
    let split: Vec<&str> = output.split("github.com/").collect();
    if split.len() < 2 {
        cli::print_error("No remote found.");
        return None;
    }
    let remote = split[1].to_string();
    let vec: Vec<&str> = remote.split(".git").collect();

    Some(vec[0].to_string())
}

pub fn get_tracked_files() -> Vec<String> {
    //! Executes the command `git ls-tree -r {branch} --name-only`.
    //! Parses the output to return a vector of file paths that represents
    //! all files tracked by git.
    let branch = get_branch_name();
    let command = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg(branch)
        .arg("--name-only")
        .output()
        .expect("Failed to execute `git ls-tree -r master --name-only`");
    let output = str::from_utf8(&command.stdout).unwrap();
    let files: Vec<&str> = output.split("\n").collect();

    files
        .into_iter()
        .map(|string| string.to_string())
        .filter(|string| !string.is_empty())
        .collect()
}

fn get_branch_name() -> String {
    //! Executes the command `git rev-parse --abbrev-ref HEAD`.
    //! Returns the output which represents the current branch the user is on.
    let command = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to execute `git rev-parse --abbrev-ref HEAD`");
    let output = str::from_utf8(&command.stdout).unwrap();

    output.trim().to_string()
}
