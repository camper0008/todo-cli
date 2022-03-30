use crate::fs::{decode_todo_file_content, encode_todos_to_file};

pub struct Todo {
    pub msg: String,
    pub priority: usize,
    pub completed: bool,
}

pub struct TodoManager {
    save_dir: String,
    todos: Vec<Todo>,
}

impl TodoManager {
    pub fn from_file(save_dir: String) -> Self {
        let todos = decode_todo_file_content(&save_dir);
        Self {
            save_dir: save_dir,
            todos: todos,
        }
    }

    fn sort_todos(&mut self) {
        (&mut self.todos).sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    pub fn todo_exists(&self, priority: usize) -> bool {
        self.todos.iter().nth(priority).is_some()
    }

    pub fn save_to_file(&self) {
        encode_todos_to_file(&self.save_dir, &self.todos);
    }

    pub fn swap_todos(&mut self, priority_0: usize, priority_1: usize) {
        let todo_0 = self.todos.iter_mut().nth(priority_0).unwrap();
        todo_0.priority = priority_1;

        let todo_1 = self.todos.iter_mut().nth(priority_1).unwrap();
        todo_1.priority = priority_0;

        self.sort_todos();
    }

    pub fn add_todo(&mut self, todo: Todo) {
        let priority = if todo.priority > self.todos.iter().count() {
            self.todos.iter().count()
        } else {
            todo.priority
        };

        self.todos
            .iter_mut()
            .skip(priority)
            .for_each(|t| t.priority += 1);

        self.todos.insert(
            priority,
            Todo {
                priority: priority,
                ..todo
            },
        );
    }
    pub fn set_todo_completion(&mut self, priority: usize, completed: bool) {
        self.todos.iter_mut().nth(priority).unwrap().completed = completed;
    }
    pub fn remove_todo(&mut self, priority: usize) {
        self.todos.remove(priority);
        self.todos
            .iter_mut()
            .skip(priority)
            .for_each(|t| t.priority -= 1);
    }
    pub fn display_todos(&self) {
        self.todos.iter().for_each(|t| {
            println!(
                "[{}] {}: {}",
                if t.completed { "x" } else { " " },
                t.priority,
                t.msg,
            )
        })
    }
}
