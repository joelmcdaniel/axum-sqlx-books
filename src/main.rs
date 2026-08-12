use axum::Router;

mod app_state;
mod domain;
mod utils;


use crate::app_state::AppState;
use crate::domain::book::routes::books_routes;
use crate::utils::tracing::init_tracing;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

const PORT_8000: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_tracing();

    let app_state = AppState::init().await.expect("Failed to initialize database");


    let app = Router::new().merge(books_routes()).with_state(app_state);

    let listener = tokio::net::TcpListener::bind(PORT_8000)
        .await
        .expect("Failed to bind address");

    tracing::debug!(
        "listening on {}",
        listener.local_addr().expect("Failed to get local address")
    );

    axum::serve(listener, app).await.expect("Failed to start server");

}
