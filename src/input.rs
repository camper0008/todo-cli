use std::io::stdin;

pub enum Command {
    View,
    Help,
    Exit,
    Clear,
    Add {
        priority: usize,
        msg: String,
    },
    Error {
        msg: String,
    },
    Remove {
        priority: usize,
    },
    Done {
        priority: usize,
    },
    Redo {
        priority: usize,
    },
    Swap {
        priority_0: usize,
        priority_1: usize,
    },
}

fn priority_from_split(split: &Vec<&str>, idx: usize) -> Result<usize, String> {
    if split.get(idx).is_none() {
        return Err(format!("no priority given at position {}", idx));
    }
    let priority_result = split.get(idx).unwrap().parse::<usize>();
    if priority_result.is_err() {
        return Err(format!("invalid priority given at position {}", idx));
    }
    return Ok(priority_result.unwrap());
}

fn add_handle(split: Vec<&str>) -> Command {
    let priority_result = priority_from_split(&split, 1);
    if priority_result.is_err() {
        return Command::Error {
            msg: priority_result.err().unwrap(),
        };
    }
    let priority = priority_result.unwrap();

    if split.get(2).is_none() {
        return Command::Error {
            msg: "no todo message given".to_string(),
        };
    }
    let msg = split
        .iter()
        .skip(2)
        .map(|s| (*s).trim().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    Command::Add { priority, msg }
}

fn rm_handle(split: Vec<&str>) -> Command {
    let priority_result = priority_from_split(&split, 1);
    if priority_result.is_err() {
        return Command::Error {
            msg: priority_result.err().unwrap(),
        };
    }
    let priority = priority_result.unwrap();
    Command::Remove { priority }
}

fn done_handle(split: Vec<&str>) -> Command {
    let priority_result = priority_from_split(&split, 1);
    if priority_result.is_err() {
        return Command::Error {
            msg: priority_result.err().unwrap(),
        };
    }
    let priority = priority_result.unwrap();
    Command::Done { priority }
}

fn redo_handle(split: Vec<&str>) -> Command {
    let priority_result = priority_from_split(&split, 1);
    if priority_result.is_err() {
        return Command::Error {
            msg: priority_result.err().unwrap(),
        };
    }
    let priority = priority_result.unwrap();
    Command::Redo { priority }
}

fn swap_handle(split: Vec<&str>) -> Command {
    let priority_0_result = priority_from_split(&split, 1);
    if priority_0_result.is_err() {
        return Command::Error {
            msg: priority_0_result.err().unwrap(),
        };
    }
    let priority_0 = priority_0_result.unwrap();

    let priority_1_result = priority_from_split(&split, 2);
    if priority_1_result.is_err() {
        return Command::Error {
            msg: priority_1_result.err().unwrap(),
        };
    }
    let priority_1 = priority_1_result.unwrap();

    Command::Swap {
        priority_0,
        priority_1,
    }
}

pub fn stdin_input() -> Command {
    let mut cmd = String::new();
    stdin().read_line(&mut cmd).unwrap();
    parse_input(cmd)
}

pub fn parse_input(cmd: String) -> Command {
    let split: Vec<&str> = cmd.split(" ").map(|s| s.trim()).collect();

    match split.get(0).unwrap().to_lowercase().as_str() {
        "view" | "list" => Command::View,
        "help" => Command::Help,
        "exit" => Command::Exit,
        "clear" | "cls" => Command::Clear,
        "add" => add_handle(split),
        "rm" => rm_handle(split),
        "done" => done_handle(split),
        "redo" => redo_handle(split),
        "swap" | "mv" => swap_handle(split),
        other_cmd => Command::Error {
            msg: format!("unrecognized command '{other_cmd}', try 'help'"),
        },
    }
}
