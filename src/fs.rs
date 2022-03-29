use crate::todo::Todo;
use std::fs;
use std::path::{Path, PathBuf};

fn todo_file_path(save_dir: &String) -> PathBuf {
    Path::new(save_dir).join(Path::new("camper-todo-cli"))
}

fn create_todo_file_if_doesnt_exist(save_dir: &String) {
    let path = todo_file_path(save_dir);

    if !path.exists() {
        fs::File::create(path).expect("unable to create file");
    }
}

fn todo_file_content(save_dir: String) -> String {
    let path = todo_file_path(&save_dir);

    fs::read_to_string(path).expect("unable to read file")
}

fn decode_todo_string(todo_string: String) -> Todo {
    let err = format!("invalid formatted string: {}", todo_string);
    let mut mpc = todo_string.split("^");
    let msg = mpc.nth(0).expect(&err).to_string();
    let priority = mpc.nth(1).expect(&err).to_string();
    let completed = mpc.nth(2).expect(&err).to_string();

    Todo {
        msg: msg,
        priority: priority.parse::<u16>().expect(&err),
        completed: completed.parse::<bool>().expect(&err),
    }
}

fn decode_todos(todos_string: String) -> Vec<Todo> {
    todos_string
        .split("|")
        .into_iter()
        .map(|todo| decode_todo_string(todo.to_string()))
        .collect()
}

pub fn encode_todos(todos: Vec<Todo>) -> String {
    todos
        .into_iter()
        .map(|todo| todo.msg + "^" + &todo.priority.to_string() + "^" + &todo.completed.to_string())
        .collect::<Vec<String>>()
        .join("|")
}
