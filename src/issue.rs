use std::collections::HashMap;

use serde::Deserialize;

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
        Issue {
            title: title,
            body: body,
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn to_map(&self) -> HashMap<&str, &str> {
        let mut params = HashMap::new();
        params.insert("title", self.title.as_str());
        params.insert("body", self.body.as_str());
        return params;
    }
}

impl Response {
    pub fn get_title(&self) -> String {
        self.title.to_string()
    }
}
