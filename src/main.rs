use std::{
    fs::File,
    io::{self, prelude::*, BufReader, Write},
    path::Path,
    process::Command,
    str,
};

fn main() {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }

    let (_username, _password) = get_credentials();

    let files = get_tracked_files();

    read_files(files).unwrap();
}

fn get_credentials() -> (String, String) {
    println!("Please enter your Github credentials.");
    print!("Username: ");
    io::stdout().flush().unwrap();

    let mut username = String::new();

    io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    (username, password)
}

fn get_tracked_files() -> Vec<String> {
    let command = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("master")
        .arg("--name-only")
        .output()
        .expect("Failed to execute command");
    let output = str::from_utf8(&command.stdout).unwrap();
    let files: Vec<&str> = output.split("\n").collect();

    files.into_iter()
        .map(|string| String::from(string))
        .filter(|string| !string.is_empty())
        .collect()
}

fn contains_todo(line: &str) -> bool {
    let comment = match line.find("//") {
        Some(value) => Some(value),
        None => line.find("#"),
    };

    let todo = line.find("TODO");
    if comment.is_some() && todo.is_some() {
        let comment_index = comment.unwrap();
        let todo_index = todo.unwrap();
        if todo_index > comment_index {
            return true;
        }
    }

    false
}

fn get_remote_name() -> String {
    let command = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .expect("Failed to execute command");
    let output = str::from_utf8(&command.stdout).unwrap();
    // Remove protocol and domain from url
    let split: Vec<&str> = output.split("github.com/").collect();
    let remote = String::from(split[1]);

    // Remote .git suffix
    let vec: Vec<&str> = remote.split(".git").collect();
    String::from(vec[0])
}

fn parse_title(line: &str) -> String {
    let vec: Vec<&str> = line.split("TODO").collect();
    let after_todo = vec[1];
    let title = if after_todo.starts_with(":") {
        &after_todo[1..]
    } else {
        after_todo
    }.trim();

    String::from(title)
}

fn create_description(line_number: &u32, file_path: &str) -> String {
    format!("Found a TODO comment on line {} of file {}",
            line_number, file_path).to_string()
}

fn open_issue(_client: &reqwest::Client, _remote: &str,
              _title: &str, _description: &str) ->
    Result<(), Box<std::error::Error>> {

    Ok(())
}

fn read_files(files: Vec<String>) -> io::Result<()> {
    let client = reqwest::Client::new();
    let remote_repo = get_remote_name();
    for path in files {
        let file = File::open(&path)?;
        let buffer = BufReader::new(file);

        let mut line_number = 0;
        for line_option in buffer.lines() {
            let line = line_option.unwrap();
            line_number += 1;

            if contains_todo(&line) {
                let title = parse_title(&line);
                let description = create_description(&line_number, &path);
                let _result = open_issue(&client, &remote_repo,
                                           &title, &description);
            }
        }
    }

    Ok(())
}
