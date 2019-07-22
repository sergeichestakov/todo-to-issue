use clap::{App, Arg};

use super::command;

pub fn init() {
    if !command::is_git_repo() {
        panic!("Must be in a git directory!");
    }

    let matches = App::new("todo-to-issue")
        .version("0.1")
        .author("Sergei Chestakov <sergei332@gmail.com>")
        .about("Converts TODO comments to GitHub issues")
        .arg(
            Arg::with_name("DIRECTORY")
                .help("Sets which directory to look through")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("TOKEN")
                .help("Sets the token for user")
                .takes_value(true),
        )
        .get_matches();

        if let Some(token) = matches.value_of("token") {
            println!("Received token {}", token);
        }
}
