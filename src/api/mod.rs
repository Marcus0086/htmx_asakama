use std::sync::{Arc, Mutex};

use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Form;
use axum::{routing::post, Router};
use serde::Deserialize;

use crate::state::AppState;
use crate::templates::HtmlTemplate;
pub struct BackendAPI {
    pub api_router: Router,
}

impl BackendAPI {
    pub fn new() -> Self {
        let app_state = Arc::new(AppState {
            todos: Mutex::new(vec![]),
        });
        let api_router = Router::new()
            .route("/todos", post(add_todo))
            .with_state(app_state);

        Self { api_router }
    }
}

#[derive(Template)]
#[template(path = "components/todo-list.html")]
struct TodoList {
    todos: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct TodoRequest {
    todo: String,
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo);

    let template = TodoList {
        todos: lock.clone(),
    };

    HtmlTemplate(template)
}
