use std::{
    env,
    fs::{self, File},
    io::{prelude::*,self, BufReader, Write},
    path::Path,
};

extern crate rpassword;

fn main() {
    if !Path::new(".git").is_dir() {
        panic!("Must be in a git directory!");
    }
    let _ignored: Vec<String> = read_gitignore().unwrap();
    let _args: Vec<String> = env::args().collect();

    println!("Please enter your Github credentials.");
    print!("Username: ");
    io::stdout().flush().unwrap();

    let mut username = String::new();

    io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    let _pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    read_files(Path::new("./")).unwrap();
}

fn read_gitignore() -> io::Result<Vec<String>> {
    let file = File::open(".gitignore")?;
    let buffer = BufReader::new(file);

    let mut vector: Vec<String> = buffer.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    vector.push(String::from(".git"));
    Ok(vector)
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
