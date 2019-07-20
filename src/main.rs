use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

use request::Request;

mod request;
mod command;
mod parse;

fn main() -> io::Result<()> {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }

    let request = Request::new();
    let files = command::get_tracked_files();

    for path in files {
        let file = File::open(&path)?;
        let buffer = BufReader::new(file);

        let mut line_number = 0;
        for line_option in buffer.lines() {
            let line = line_option.unwrap();
            line_number += 1;

            if parse::contains_todo(&line) {
                let params = Request::build_params(
                    parse::extract_title(&line),
                    parse::create_description(&line_number, &path)
                );
                let result = request.create_issue(params);
                println!("{:?}", result);
            }
        }
    }

    Ok(())
}
