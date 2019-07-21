use std::collections::HashMap;

use serde::Deserialize;

pub struct Issue<'a> {
    title: &'a str,
    body: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    id: i32,
    title: String,
    body: String,
    number: i32,
    state: String,
}

impl<'a> Issue<'a> {
    pub fn new(title: &'a str, body: &'a str) -> Issue<'a> {
        Issue {
            title: title,
            body: body,
        }
    }

    pub fn get_title(&self) -> &str {
        self.title
    }

    pub fn to_map(&self) -> HashMap<&str, &str> {
        let mut params = HashMap::new();
        params.insert("title", self.title);
        params.insert("body", self.body);
        return params;
    }
}

impl Response {
    pub fn get_title(&self) -> String {
        self.title.to_string()
    }
}
