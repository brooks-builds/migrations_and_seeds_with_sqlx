use seeds::run;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("missing .env file");

    let database_uri =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_uri)
        .await
        .expect("Error connecting to database");

    run(pool).await.unwrap();
}
