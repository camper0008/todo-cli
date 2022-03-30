mod cmd;
mod env;
mod fs;
mod input;
mod todo;

use crate::cmd::run;
use crate::env::{command_from_args, save_path};
use crate::fs::create_todo_file_if_doesnt_exist;
use crate::input::{stdin_input, Command};
use crate::todo::TodoManager;

fn main() -> ! {
    let dir_save_path = save_path();
    create_todo_file_if_doesnt_exist(&dir_save_path);
    let mut manager = TodoManager::from_file(dir_save_path);
    let arg_command = command_from_args();
    if arg_command.is_some() {
        run(&mut manager, arg_command.unwrap());
        run(&mut manager, Command::Exit);
    };
    loop {
        println!("cmd: ");
        run(&mut manager, stdin_input());
    }
}
