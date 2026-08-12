use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::app_state::AppState;
use crate::domain::book::error::BookError;


#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub year: i16,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl Book {
    pub async fn get_all(app_state: State<AppState>) -> Result<Json<Vec<Book>>, BookError> {
        let books = sqlx::query_as!(
            Book,
            r#"
                SELECT *
                FROM books
                ORDER BY id DESC
            "#
        )
        .fetch_all(&app_state.pool)
        .await?;

        Ok(Json(books))
    }
}
