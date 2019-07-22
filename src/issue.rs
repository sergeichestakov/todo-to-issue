use std::collections::HashMap;

use serde::Deserialize;

pub const LABEL: &str = "TODO";
const TITLE_PREFIX: &str = "Title: ";
const BODY_PREFIX: &str = "Body: ";

pub struct Issue {
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    id: i32,
    title: String,
    body: String,
    number: i32,
    state: String,
}

impl Issue {
    pub fn new(title: String, body: String) -> Issue {
        Issue { title, body }
    }

    pub fn from_string(string: String) -> Option<Issue> {
        // 1. Split the string based on \n
        let split: Vec<&str> = string.split("\n").collect();
        // 2. Assert there are AT LEAST two parts
        if split.len() < 2 {
            return None;
        }

        let title_line = split[0];
        let body_line = split[1];

        // 3. Assert first line starts with "Title:"
        // 4. Assert second line starts with "Body:"
        if !title_line.starts_with(TITLE_PREFIX)
            || !body_line.starts_with(BODY_PREFIX)
        {
            return None;
        }

        // 5. Strip prefixes and trim lines
        let title = title_line[TITLE_PREFIX.len()..].trim().to_string();
        let mut body = body_line[BODY_PREFIX.len()..].trim().to_string();

        // 6. Ensure title is not empty.
        if title.is_empty() {
            return None;
        }

        // 7. Construct body from the rest of the file
        for index in 2..split.len() {
            body += "\n";
            body += split[index];
        }

        Some(Issue { title, body })
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {}\n{} {}",
            TITLE_PREFIX, &self.title, BODY_PREFIX, &self.body
        )
        .to_string()
    }

    pub fn to_map(&self) -> HashMap<&str, &str> {
        let mut params = HashMap::new();
        params.insert("title", self.title.as_str());
        params.insert("body", self.body.as_str());
        params.insert("labels", LABEL);
        return params;
    }
}

impl Response {
    pub fn get_title(&self) -> String {
        self.title.to_string()
    }
}
