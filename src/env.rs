use std::env;
use std::path::Path;

pub fn home_variable() -> String {
    env::var("HOME").expect("unable to get home variable")
}

pub fn default_save_path() -> String {
    Path::new(&home_variable())
        .join(Path::new(".config"))
        .to_string_lossy()
        .to_string()
}

pub fn save_path() -> String {
    for arg in env::args() {
        if arg.starts_with("-p=") {
            return arg
                .splitn(2, "=")
                .nth(1)
                .expect("invalid -p option")
                .to_string();
        }
    }
    default_save_path()
}
