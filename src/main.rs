mod env;
mod fs;
mod todo;

use crate::{env::default_save_path, fs::encode_todos, todo::Todo};

fn main() {
    let dir_save_path: String = default_save_path();
    println!(
        "{}",
        encode_todos(vec![
            Todo {
                msg: "task 0".to_string(),
                priority: 0,
                completed: true,
            },
            Todo {
                msg: "task 1".to_string(),
                priority: 1,
                completed: true,
            },
            Todo {
                msg: "task 2".to_string(),
                priority: 2,
                completed: true,
            }
        ])
    );
}
