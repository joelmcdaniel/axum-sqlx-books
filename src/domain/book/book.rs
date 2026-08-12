use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::app_state::AppState;
use crate::domain::book::dto::{CreateBookDto, DeleteBookResult, UpdateBookDto};
use crate::domain::book::error::BookError;


#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
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

    pub async fn get_by_id(Path(id): Path<i32>, app_state: State<AppState>) -> Result<Json<Book>, BookError> {
        let book = sqlx::query_as!(
            Book,
            r#"
                SELECT *
                FROM books
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BookError::NotFound(id),
            _ => BookError::Database(e),
        })?;

        Ok(Json(book))
    }

    pub async fn create(
        app_state: State<AppState>,
        Json(body): Json<CreateBookDto>,
    ) -> Result<(StatusCode, Json<Book>), BookError> {
        let book = sqlx::query_as!(
            Book,
            r#"
                INSERT INTO books (title, author, year)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            body.title,
            body.author,
            body.year
        )
        .fetch_one(&app_state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(book)))
    }

    pub async fn update(
        Path(id): Path<i32>,
        app_state: State<AppState>,
        Json(body): Json<UpdateBookDto>,
    ) -> Result<Json<Book>, BookError> {
        let book = sqlx::query_as!(
            Book,
            r#"
                UPDATE books
                SET title = COALESCE($1, title),
                    author = COALESCE($2, author),
                    year = COALESCE($3, year)
                WHERE id = $4
                RETURNING *
            "#,
            body.title,
            body.author,
            body.year,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BookError::NotFound(id),
            _ => BookError::Database(e),
        })?;

        Ok(Json(book))
    }

    pub async fn delete(
        Path(id): Path<i32>,
        app_state: State<AppState>,
    ) -> Result<Json<DeleteBookResult>, BookError> {
        // TODO: Delete a book by id and return the deleted id, or NotFound if missing
        let result = sqlx::query_as!(
            DeleteBookResult,
            "DELETE FROM books WHERE id = $1 RETURNING id",
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BookError::NotFound(id),
            _ => BookError::Database(e),
        })?;

        Ok(Json(result))
    }
}
