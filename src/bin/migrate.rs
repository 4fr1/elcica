use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().max_connections(1).connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("migrasi selesai");
    Ok(())
}
