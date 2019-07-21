use std::collections::{HashMap, HashSet};

use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;

use issue::Issue;

use super::command;
use super::issue;

const API_ENDPOINT: &str = "https://api.github.com";
const LABEL: &str = "TODO";

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
            url: format!("{}/repos/{}/issues", API_ENDPOINT, remote)
                .to_string(),
            auth_header: format!("token {}", token).to_string(),
        }
    }

    pub fn open_issue(
        &self,
        issue: &Issue,
    ) -> Result<(), Box<std::error::Error>> {
        let params = issue.to_map();
        let response = self
            .client
            .post(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .json(&params)
            .send()?;
        println!("{:?}", response);

        Self::assert_successful_response(response.status());

        println!(
            "Successfully created issue with title: {}",
            issue.get_title()
        );

        Ok(())
    }

    pub fn get_issues(
        &self,
    ) -> Result<HashSet<String>, Box<std::error::Error>> {
        // Get all open and closed issues with this label
        let mut params = HashMap::new();
        params.insert("labels", LABEL);
        params.insert("state", "all");

        let mut response = self
            .client
            .get(&self.url)
            .header(AUTHORIZATION, self.auth_header.clone())
            .query(&params)
            .send()?;
        println!("{:?}", response);

        Self::assert_successful_response(response.status());

        let mut issues = HashSet::new();
        if let Ok(json_array) = response.json::<Vec<issue::Response>>() {
            for result in json_array {
                issues.insert(result.get_title());
            }
        }

        Ok(issues)
    }

    fn assert_successful_response(status: StatusCode) {
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
