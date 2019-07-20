use std::collections::HashMap;

use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;

use super::command;

const API_ENDPOINT: &str = "https://api.github.com";

pub struct Request {
    client: reqwest::Client,
    url: String,
    auth_header: String,
}

impl Request {
    pub fn new() -> Request {
        let remote = command::get_remote_name();
        let token = command::read_access_token();

        Request {
            client: reqwest::Client::new(),
            url: format!("{}/repos/{}/issues", API_ENDPOINT, remote).to_string(),
            auth_header: format!("token {}", token).to_string(),
        }
    }

    pub fn create_issue(&self, params: HashMap<&str, String>) ->
        Result<(), Box<std::error::Error>> {
        let response = self.client
            .post(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .json(&params)
            .send()?;
        println!("{:?}", response);
        let title = params.get("title").unwrap();
        Self::handle_status_code(response.status(), title);

        Ok(())
    }

    pub fn build_params<'a>(title: String, description: String) -> HashMap<&'a str, String> {
        let mut params = HashMap::new();
        params.insert("title", title);
        params.insert("body", description);
        return params;
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
}