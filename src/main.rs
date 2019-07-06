use std::{
    fs::{self},
    io::{self, Write},
    path::Path,
    process::Command,
    str,
};

fn main() {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }

    let (_username, _password) = get_credentials();

    let _files = get_tracked_files();

    read_files(Path::new("./")).unwrap();
}

fn get_credentials() -> (String, String) {
    println!("Please enter your Github credentials.");
    print!("Username: ");
    io::stdout().flush().unwrap();

    let mut username = String::new();

    io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    let password  = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    (username, password)
}

fn get_tracked_files() -> Vec<String> {
    let command = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("master")
        .arg("--name-only")
        .output()
        .expect("Failed to execute command");
    let output = str::from_utf8(&command.stdout).unwrap();
    let files: Vec<&str> = output.split("\n").collect();

    files.into_iter()
        .map(|string| String::from(string))
        .filter(|string| !string.is_empty())
        .collect()
}

fn read_files(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            read_files(&path)?;
        } else {
            println!("{}", path.display());
        }
    }
    Ok(())
}
