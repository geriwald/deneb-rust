use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::healthcheck::health_checker_handler,
    handlers::todo::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        list_todos_handler,
    },
    models::todo,
};

pub fn create_router() -> Router {
    let db = todo::todo_db();

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/todos",
            post(create_todo_handler).get(list_todos_handler),
        )
        .route(
            "/api/todos/{id}",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        .with_state(db)
}
