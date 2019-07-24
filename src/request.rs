use std::collections::{HashMap, HashSet};

use console::style;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;

use issue::Issue;

use super::command;
use super::issue;
use super::parse;

const API_ENDPOINT: &str = "https://api.github.com";

pub struct Request {
    client: reqwest::Client,
    url: String,
    remote_url: String,
    auth_header: String,
}

impl Request {
    pub fn new(token: String) -> Request {
        //! Creates a new request object containing the http client,
        //! url formatted with the API endpoint and user's remote repo,
        //! and auth header containing the user's token.
        let remote = command::get_remote_name();

        Request {
            client: reqwest::Client::new(),
            url: format!("{}/repos/{}/issues", API_ENDPOINT, remote)
                .to_string(),
            remote_url: format!("https://github.com/{}", remote).to_string(),
            auth_header: format!("token {}", token).to_string(),
        }
    }

    pub fn open_issue(
        &self,
        issue: &Issue,
    ) -> Result<(), Box<std::error::Error>> {
        //! Makes a POST request to create a new issue with
        //! the inputted params (title and description).
        //!
        //! Panics if the response is not 201 Created or the request fails.
        let response = self
            .client
            .post(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .json(&issue.to_json())
            .send()?;

        Self::assert_successful_response(response.status());

        Ok(())
    }

    pub fn get_issues(
        &self,
    ) -> Result<HashSet<String>, Box<std::error::Error>> {
        //! Makes a GET request to retrieve all issues (open and closed)
        //! with a todo label in the remote repository.
        //!
        //! Returns a hashset of the issue titles. Panics if the response
        //! is not 200 OK or the request fails.
        let mut params = HashMap::new();
        params.insert("labels", issue::LABEL);
        params.insert("state", "all");

        println!(
            "Fetching all issues with {} label from {}",
            style(issue::LABEL).cyan(),
            style(&self.remote_url).italic()
        );
        let mut response = self
            .client
            .get(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .query(&params)
            .send()?;

        Self::assert_successful_response(response.status());

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
                parse::handle_plural(&n, "issue")
            ),
        };

        Ok(issues)
    }

    fn assert_successful_response(status: StatusCode) {
        //! Asserts that the status code returned is either
        //! 200 OK or 201 CREATED.
        //!
        //! Otherwise, panics with a detailed description.
        match status {
            StatusCode::OK | StatusCode::CREATED => (),
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
