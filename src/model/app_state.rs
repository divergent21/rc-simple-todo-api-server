use std::sync::{Arc, Mutex};
use super::todo::Todo;

type TodoList = Arc<Mutex<Vec<Todo>>>;

pub struct AppState {
    pub todo_list: TodoList,
}

impl AppState {
    pub fn init () -> Self {
        Self {
            todo_list: Arc::new(Mutex::new(Vec::new()))
        }
    }
}