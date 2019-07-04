use std::io;
use std::io::Write;
use std::path::Path;

extern crate rpassword;

fn main() {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }

    println!("Please enter your Github credentials.");
    print!("Username: ");
    io::stdout().flush().unwrap();

    let mut username = String::new();

    io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    let _pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
}
