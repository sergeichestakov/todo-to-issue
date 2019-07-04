use std::io;
use std::env;
use std::io::Write;
use std::path::Path;
use std::fs::{self, DirEntry};

extern crate rpassword;

fn main() {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }
    let _args: Vec<String> = env::args().collect();

    println!("Please enter your Github credentials.");
    print!("Username: ");
    io::stdout().flush().unwrap();

    let mut username = String::new();

    io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    let _pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    read_files(Path::new("./"));
}

fn read_files(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                read_files(&path)?;
            } else {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}
