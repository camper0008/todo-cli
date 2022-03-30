use crate::input::Command;
use crate::todo::{Todo, TodoManager};

fn display_help() {
    const COMMANDS: [&str; 9] = [
        "help: display this menu",
        "view | list: view todos",
        "clear | cls: clear screen",
        "exit: exit and save to file",
        "add <priority> <msg>: add todo with priority and message",
        "rm <priority>: remove todo with priority",
        "done <priority>: set todo with priority to be done",
        "redo <priority>: set todo with priority to be not done",
        "swap | mv <priority> <priority>: swap the priorities of two todos",
    ];
    const ARGS: [&str; 1] =
        ["running todo-cli with args runs the args and immediately exits, e.g. 'todo-cli view'"];
    println!(
        "{}\n",
        ARGS.into_iter()
            .fold(String::from("args:"), |acc, s| acc + "\n  " + s)
    );
    println!(
        "{}\n",
        COMMANDS
            .into_iter()
            .fold(String::from("commands:"), |acc, s| acc + "\n  " + s)
    );
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn run(manager: &mut TodoManager, cmd: Command) {
    match cmd {
        Command::Error { msg } => println!("err: {}", msg),
        Command::Clear => clear_screen(),
        Command::View => manager.display_todos(),
        Command::Help => display_help(),
        Command::Exit => {
            manager.save_to_file();
            std::process::exit(0);
        }
        Command::Add { priority, msg } => manager.add_todo(Todo {
            priority,
            msg,
            completed: false,
        }),
        Command::Remove { priority } => {
            if manager.todo_exists(priority) {
                manager.remove_todo(priority);
                return;
            }
            println!("err: todo with priority {priority} does not exist");
        }
        Command::Done { priority } => {
            if manager.todo_exists(priority) {
                manager.set_todo_completion(priority, true);
                return;
            }
            println!("err: todo with priority {priority} does not exist");
        }
        Command::Redo { priority } => manager.set_todo_completion(priority, false),
        Command::Swap {
            priority_0,
            priority_1,
        } => {
            if manager.todo_exists(priority_0) && manager.todo_exists(priority_1) {
                manager.swap_todos(priority_0, priority_1);
            } else if !manager.todo_exists(priority_0) {
                println!("err: todo with priority {priority_0} does not exist");
            } else if !manager.todo_exists(priority_1) {
                println!("err: todo with priority {priority_1} does not exist");
            }
        }
    };
}
