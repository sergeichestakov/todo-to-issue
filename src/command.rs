use std::process::Command;
use std::str;

use dialoguer::PasswordInput;

const FATAL_GIT_STATUS_MESSAGE: &str =
    "fatal: not a git repository (or any of the parent directories): .git";

pub fn read_access_token() -> String {
    //! Reads in a user's personal access token from GitHub.
    println!("Please enter your personal access token.");
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

    output != FATAL_GIT_STATUS_MESSAGE
}

pub fn get_remote_name() -> String {
    //! Executes the command `git remote get-url origin`.
    //! Parses the result to return a string of the form :username/:repo.
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
    let remote = split[1].to_string();
    let vec: Vec<&str> = remote.split(".git").collect();

    vec[0].to_string()
}

pub fn get_tracked_files() -> Vec<String> {
    //! Executes the command `git ls-tree -r master --name-only`.
    //! Parses the output to return a vector of file paths that represents
    //! all files tracked by git.
    let command = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("master")
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
