use clap::{App, Arg};
use dialoguer::Confirmation;

use super::command;

pub fn init() {
    if !command::is_git_repo() {
        panic!("Must be in a git directory!");
    }

    let _matches = App::new("todo-to-issue")
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
}

pub fn prompt_to_continue() -> bool {
    Confirmation::new()
        .with_text("Do you want to continue?")
        .interact()
        .expect("Failed to read confirmation")
}
