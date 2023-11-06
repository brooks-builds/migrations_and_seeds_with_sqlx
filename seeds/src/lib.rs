use eyre::Result;
use sqlx::{Pool, Postgres};

pub async fn run(pool: Pool<Postgres>) -> Result<()> {
    let mut transaction = pool.begin().await?;
    let mut is_success = true;

    if let Err(error) = sqlx::query_file!("./seeds/seed_games.sql")
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Error seeding games: {error}");
        is_success = false;
    }

    if let Err(error) = sqlx::query_file!("./seeds/seed_platforms.sql")
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Error seeding platforms: {error}");
        is_success = false;
    }

    if let Err(error) = sqlx::query_file!("./seeds/seed_game_platform.sql")
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Error seeding game platform: {error}");
        is_success = false;
    }

    if is_success {
        transaction.commit().await?;
    } else {
        transaction.rollback().await?;
    }

    Ok(())
}
