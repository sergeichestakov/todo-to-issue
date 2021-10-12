use super::cli;
use super::issue;
use console::style;
use issue::Issue;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use serde_json::json;
use std::collections::HashSet;

const API_ENDPOINT: &str = "https://api.github.com";

pub struct Request {
    client: reqwest::Client,
    url: String,
    remote_url: String,
    auth_header: String,
}

impl Request {
    pub fn new(token: String, remote: String) -> Request {
        //! Creates a new request object that encapsulates the http client,
        //! url formatted with the API endpoint and user's remote repo,
        //! and auth header containing the user's token.
        Request {
            client: reqwest::Client::new(),
            url: format!("{}/repos/{}/issues", API_ENDPOINT, remote)
                .to_string(),
            remote_url: format!("https://github.com/{}", remote).to_string(),
            auth_header: format!("token {}", token).to_string(),
        }
    }

    pub fn open_issue(&self, issue: &Issue, label: &String) -> Option<usize> {
        //! Makes a POST request to create a new issue with
        //! the inputted params (title and description).
        //!
        //! Panics if the response is not 201 Created or the request fails.
        //! Returns a number which represents the issue number from GitHub.
        let mut response = self
            .client
            .post(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .json(&issue.to_json(&label))
            .send()
            .expect("Failed to create issue");

        if !Self::is_successful_response(response.status()) {
            return None;
        }

        match response.json::<issue::Response>() {
            Ok(json) => Some(json.get_number()),
            Err(_) => None,
        }
    }

    pub fn get_issues(
        &self,
        is_dry_run: bool,
        label: String,
    ) -> Option<HashSet<String>> {
        //! Makes a GET request to retrieve all issues (open and closed)
        //! with a todo label in the remote repository.
        //!
        //! Returns a hashset of the issue titles. Returns early if the response
        //! is not 200 OK or the request fails.
        if is_dry_run {
            return Some(HashSet::new());
        }

        println!(
            "Fetching all issues with {} label from {}",
            style(format!("issue::{}", label)).cyan(),
            style(&self.remote_url).italic()
        );

        let params = json!({
            "labels": format!("issue::{}", label),
            "state": "all",
        });
        let mut response = self
            .client
            .get(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .query(&params)
            .send()
            .expect("Failed to get issues");

        if !Self::is_successful_response(response.status()) {
            return None;
        }

        let mut issues = HashSet::new();
        if let Ok(json_array) = response.json::<Vec<issue::Response>>() {
            for result in json_array {
                issues.insert(result.get_title());
            }
        }

        match issues.len() {
            0 => println!(
                "No previously opened issues found in the remote repo."
            ),
            n => println!(
                "Found {} previously opened {} in the remote repo.",
                style(n).bold(),
                cli::handle_plural(&n, "issue")
            ),
        };

        Some(issues)
    }

    fn is_successful_response(status: StatusCode) -> bool {
        //! Asserts that the status code returned is either
        //! 200 OK or 201 CREATED.
        //!
        //! Otherwise, outputs a detailed description about the error.
        match status {
            StatusCode::OK | StatusCode::CREATED => return true,
            StatusCode::UNAUTHORIZED => cli::print_error(
                "Unathorized request. \
                 Make sure your access token is valid and \
                 you have pull access to the repository.",
            ),
            StatusCode::GONE => {
                cli::print_error("Issues are disabled in this repository.");
            }
            StatusCode::FORBIDDEN => cli::print_error(
                "You have reached the GitHub API rate limit. \
                 Please try again later.",
            ),
            StatusCode::NOT_FOUND => cli::print_error(
                "Remote repository not found. \
                 If your repository is private check that \
                 your access token has the correct permissions.",
            ),
            StatusCode::UNPROCESSABLE_ENTITY => {
                cli::print_error("Unable to process request.");
            }
            s => cli::print_error(
                &format!("Received unexpected status code {}", s).to_string(),
            ),
        };

        false
    }
}
