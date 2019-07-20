use std::process::Command;
use std::str;

// Reads in a user's personal access token from GitHub.
pub fn read_access_token() -> String {
    println!("Please enter your personal access token.");
    rpassword::read_password_from_tty(Some("Token: "))
        .expect("Failed to read token")
}

// Executes the command `git remote get-url origin`
// Parses the result to return a string of the form :username/:repo
pub fn get_remote_name() -> String {
    let command = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .expect("Failed to execute `git remote get-url origin`");
    let output = str::from_utf8(&command.stdout).unwrap();
    // Output is of the form https://github.com/:username/:repo.git
    // So we must remove the protocol and domain as well as the .git suffix
    let split: Vec<&str> = output.split("github.com/").collect();
    let remote = String::from(split[1]);
    let vec: Vec<&str> = remote.split(".git").collect();

    String::from(vec[0])
}

// Executes the command `git ls-tree -r master --name-only`
// Parses the output to return a vector of file paths that represents all files tracked by git
pub fn get_tracked_files() -> Vec<String> {
    let command = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("master")
        .arg("--name-only")
        .output()
        .expect("Failed to execute `git ls-tree -r master --name-only`");
    let output = str::from_utf8(&command.stdout).unwrap();
    let files: Vec<&str> = output.split("\n").collect();

    files.into_iter()
        .map(|string| String::from(string))
        .filter(|string| !string.is_empty())
        .collect()
}