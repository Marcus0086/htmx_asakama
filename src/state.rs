use std::sync::Mutex;

pub struct AppState {
    pub todos: Mutex<Vec<String>>,
}
