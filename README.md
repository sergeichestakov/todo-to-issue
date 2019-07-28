# todo-to-issue

CLI tool that converts forgotten TODO comments into actionable GitHub issues.

![Screenshot](https://user-images.githubusercontent.com/24947334/62002420-74b4fc80-b0b8-11e9-8fc8-fa469926b106.png?s=100)
## Installation

#### MacOS
```bash
brew tap sergeichestakov/taproom
brew install todo-to-issue
```

#### Ubuntu/Debian
```bash
wget https://github.com/sergeichestakov/todo-to-issue/raw/master/dist/todo-to-issue_amd64.deb
sudo dpkg -i todo-to-issue_amd64.deb
```

#### Cargo
```bash
cargo install todo-to-issue
``` 
Make sure `${HOME}/.cargo/bin` is in your PATH to be able to run installed binaries.

## How to use

1. Make sure you generate a [Personal Access Token](https://github.com/settings/tokens/new) on GitHub  with `public_repo` or `repo` scope.

2. `cd` into the directory you want to inspect. Make sure it's a git repository and the remote has issues enabled.

3. Run `todo-to-issue $TOKEN` where `$TOKEN` is your personal access token. Alternatively, you can run the command without the token argument which will prompt you to paste it into a hidden password input. If you're just doing a dry run (`-n` flag), you don't need a token at all.

## How it works

Running this command will read every file tracked by git for TODO comments, generating a title and body for each one. Then, for every TODO found, it will prompt you with the following options:
```
1. Open Issue
2. Edit Issue
3. Skip Issue
4. Exit
```

- `Open Issue` will create a new GitHub issue with a `TODO` label, based on the generated title and body.

> By default, the title here is simply the rest of the comment after `TODO:` and the description contains the line and file the comment appears in.
- `Edit Issue` will open your default editor and allow you to change the title and body before opening the issue. The only restriction here is that the edited issue must be of the following format:
```
Title: Your one line title here.
Body: Your description here.

This can span multiple lines and include markdown just like normal GitHub issues.

Everything after the second line is considered part of the description so this can be arbitrarily long.
```
> In other words, the first line must begin with `Title:` and the second must begin with `Body:`. This is just to make parsing easier and will not be included as part of your Issue. If the file contains an invalid format when you save and quit, the issue will not be created. You can also quit without saving to move on to the next comment without creating an issue.

- `Skip` will move on to the next comment found.

- `Exit` will terminate the program.

If you're not doing a dry run, running this command will also query all of the previous GH issues (open and closed) with the `TODO` label. If any of them have the same title, they will be ignored. This is to prevent creating multiple GH issues for the same comment.

> Note: This also means the output of a dry run is the actual amount of TODO comments found, as opposed to the default behavior which outputs the number of TODO comments that do not match the title of an existing issue with a `TODO` label in the remote repository.

### Options

| Option  | Description |
| ------------- | ------------- |
| `-n, --dry-run`  | Outputs the number of TODOs without opening any issues.  |
| `-h, --help` | Prints help information. |
| `-p, --pattern "<PATTERN>"` | Sets a glob pattern to narrow search for TODO comments to specific files. |
| `-V, --version` | Prints version information. |
| `-v, --verbose`  | Makes output more descriptive.  |

## Local Development
Make sure you have Rust [installed](https://www.rust-lang.org/tools/install).

On Linux you also have to install OpenSSL via `sudo apt-get install pkg-config libssl-dev`. See the [docs](https://docs.rs/openssl/0.10.24/openssl/) for more.
```bash
git clone https://github.com/sergeichestakov/todo-to-issue.git
cd todo-to-issue
cargo build

# Running the program and passing params
cargo run -- $TOKEN -n -v

# To test on other repos create a symlink
sudo ln -s $PWD/target/debug/todo-to-issue /usr/local/bin
cd ../another_repo
todo-to-issue $TOKEN
```
