use crate::input::{parse_input, Command};
use std::env;
use std::path::Path;

fn home_variable() -> String {
    env::var("HOME").expect("unable to get home variable")
}

pub fn save_path() -> String {
    Path::new(&home_variable())
        .join(Path::new(".config"))
        .to_string_lossy()
        .to_string()
}

pub fn command_from_args() -> Option<Command> {
    if env::args().count() > 1 {
        let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
        Some(parse_input(input))
    } else {
        None
    }
}
