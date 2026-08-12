use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BookApiError {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum BookError {
    #[error("Book with id {0} not found")]
    NotFound(i32),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for BookError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            BookError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BookError::Database(ref err) => {
                tracing::error!("Database error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (
            status_code,
            Json(BookApiError {
                error: error_message,
            }),
        )
            .into_response()
    }
}
