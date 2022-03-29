pub struct Todo {
    pub msg: String,
    pub priority: u16,
    pub completed: bool,
}

struct TodoManager {
    save_dir: String,
    todos: Vec<Todo>,
}

impl TodoManager {
    fn from_file(save_dir: String) -> Self {
        Self {
            save_dir: save_dir,
            todos: vec![],
        }
    }
    fn sort_todos(&mut self) {
        (&mut self.todos).sort_by(|a, b| a.priority.cmp(&b.priority))
    }
    fn add_todo(&mut self) {}
    fn complete_todo(&mut self) {}
    fn remove_todo(&mut self) {}
    fn display_todos(&self) {
        self.todos.iter().for_each(|t| {
            println!(
                "[{}] {}. {}",
                if t.completed { "x" } else { " " },
                t.priority,
                t.msg,
            )
        })
    }
}
