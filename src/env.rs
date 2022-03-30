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
