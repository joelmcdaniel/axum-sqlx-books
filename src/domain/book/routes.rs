use axum::routing::get;
use axum::Router;

use crate::app_state::AppState;
use crate::domain::book::book::Book;

pub fn books_routes() -> Router<AppState> {
    Router::new().route("/books", get(Book::get_all))
}
