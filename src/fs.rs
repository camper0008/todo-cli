use crate::todo::Todo;
use std::fs;
use std::path::{Path, PathBuf};

pub const TODO_FILE_NAME: &str = "camper-todo-cli";

fn todo_file_path(save_dir: &String) -> PathBuf {
    Path::new(save_dir).join(Path::new(TODO_FILE_NAME))
}

pub fn create_todo_file_if_doesnt_exist(save_dir: &String) {
    let path = todo_file_path(save_dir);

    if !path.exists() {
        fs::File::create(path).expect("unable to create file");
    }
}

fn todo_file_content(save_dir: &String) -> String {
    let path = todo_file_path(save_dir);

    fs::read_to_string(path)
        .expect("unable to read file")
        .trim()
        .to_string()
}

fn decode_todo_string(todo_string: String) -> Todo {
    let err = format!("invalid formatted todo: {}", todo_string);
    let mpc: Vec<&str> = todo_string.split("^").collect();
    let msg = mpc.get(0).expect(&err).to_string();
    let priority = mpc.get(1).expect(&err).to_string();
    let completed = mpc.get(2).expect(&err).to_string();

    Todo {
        msg: msg,
        priority: priority.parse::<usize>().expect(&err),
        completed: completed.parse::<bool>().expect(&err),
    }
}

fn decode_todos(todos_string: String) -> Vec<Todo> {
    todos_string
        .split("|")
        .into_iter()
        .filter(|s| s.chars().count() > 0)
        .map(|todo| decode_todo_string(todo.to_string()))
        .collect()
}

pub fn decode_todo_file_content(save_dir: &String) -> Vec<Todo> {
    let file_content = todo_file_content(save_dir);
    let todos = decode_todos(file_content);
    todos
}

fn encode_todos(todos: &Vec<Todo>) -> String {
    todos
        .iter()
        .map(|todo| {
            todo.msg.to_owned()
                + "^"
                + &todo.priority.to_string()
                + "^"
                + &todo.completed.to_string()
        })
        .collect::<Vec<String>>()
        .join("|")
}

pub fn encode_todos_to_file(save_dir: &String, todos: &Vec<Todo>) {
    let path = todo_file_path(save_dir);
    let file_content = encode_todos(todos);
    fs::write(path, file_content).expect("unable to save to file");
}
