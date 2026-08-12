use axum::routing::get;
use axum::Router;

use crate::app_state::AppState;
use crate::domain::book::book::Book;

pub fn books_routes() -> Router<AppState> {
    Router::new()
        .route("/books", get(Book::get_all).post(Book::create))
        .route(
            "/books/{id}",
            get(Book::get_by_id)
                .patch(Book::update)
                .delete(Book::delete),
        )
}
