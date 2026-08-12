use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn init() -> sqlx::Result<Self> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let pool = PgPool::connect(&database_url).await?;

        // run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}
