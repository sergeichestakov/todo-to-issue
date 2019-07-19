use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
    process::Command,
    str,
};
use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;

const API_ENDPOINT: &str = "https://api.github.com";
const TODO: &str = "TODO";

struct Request {
    client: reqwest::Client,
    url: String,
    auth_header: String,
}

impl Request {
    fn create_issue(&self, params: HashMap<&str, String>) ->
        Result<(), Box<std::error::Error>> {
        let response = self.client
            .post(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .json(&params)
            .send()?;
        println!("{:?}", response);
        let title = params.get("title").unwrap();
        handle_status_code(response.status(), title);

        Ok(())
    }
}

fn build_request() -> Request {
    let remote = get_remote_name();
    let token = get_access_token();

    Request {
        client: reqwest::Client::new(),
        url: format!("{}/repos/{}/issues", API_ENDPOINT, remote).to_string(),
        auth_header: format!("token {}", token).to_string(),
    }
}

fn build_params<'a>(title: String, description: String) -> HashMap<&'a str, String> {
    let mut params = HashMap::new();
    params.insert("title", title);
    params.insert("body", description);
    params
}

fn main() {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }

    read_files().expect("Failed to read files");
}

fn get_access_token() -> String {
    println!("Please enter your personal access token.");
    rpassword::read_password_from_tty(Some("Token: "))
        .expect("Failed to read token")
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

    let todo = line.find(TODO);
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
    let vec: Vec<&str> = line.split(TODO).collect();
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

fn handle_status_code(status: StatusCode, title: &str) {
    match status {
        StatusCode::CREATED => {
            println!("Successfully created issue with title: {}", title);
        }
        StatusCode::UNAUTHORIZED => {
            panic!(
                "Unathorized request. \
                Make sure your access token is valid and \
                you have pull access to the repository."
            );
        }
        StatusCode::GONE => {
            panic!("Issues are disabled in this repository.");
        }
        StatusCode::FORBIDDEN => {
            panic!(
                "You have reached the GitHub API rate limit. \
                Please try again later."
            );
        }
        StatusCode::NOT_FOUND => {
            panic!(
                "Repo or username not found. \
                If your repository is private check that \
                your access token has the correct permissions."
            );
        }
        s => panic!("Received unexpected status code {}", s),
    };
}

fn read_files() -> io::Result<()> {
    let request = build_request();
    let files = get_tracked_files();

    for path in files {
        let file = File::open(&path)?;
        let buffer = BufReader::new(file);

        let mut line_number = 0;
        for line_option in buffer.lines() {
            let line = line_option.unwrap();
            line_number += 1;

            if contains_todo(&line) {
                let params = build_params(
                    parse_title(&line),
                    create_description(&line_number, &path)
                );
                let result = request.create_issue(params);
                println!("{:?}", result);
            }
        }
    }

    Ok(())
}
