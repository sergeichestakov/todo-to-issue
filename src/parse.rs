use std::str;

const TODO: &str = "TODO";

pub fn contains_todo(line: &str) -> bool {
    let comment = match line.find("//") {
        Some(value) => Some(value),
        None => line.find("#"),
    };

    let todo = line.find(TODO);
    if comment.is_some() && todo.is_some() {
        let comment_index = comment.unwrap();
        let todo_index = todo.unwrap();
        return todo_index > comment_index;
    }

    false
}

pub fn extract_title(line: &str) -> String {
    let vec: Vec<&str> = line.split(TODO).collect();
    let after_todo = vec[1];
    let title = if after_todo.starts_with(":") {
        &after_todo[1..]
    } else {
        after_todo
    }.trim();

    String::from(title)
}

pub fn create_description(line_number: &u32, file_path: &str) -> String {
    format!("Found a TODO comment on line {} of file {}",
            line_number, file_path).to_string()
}