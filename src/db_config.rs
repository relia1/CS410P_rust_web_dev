use crate::*;
use std::error::Error;
use tracing::trace;
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

pub async fn db_setup() -> Result<DatabaseConnection, Box<dyn Error>> {
    use std::env::var;
    use std::fs;

    let pg_user = var("PG_USER")?;
    let password_file = var("PG_PASSWORDFILE")?;
    let password = fs::read_to_string(password_file)?;
    let pg_host = var("PG_HOST")?;
    let pg_dbname = var("PG_DBNAME")?;

    let connection = db_connect(&pg_user, &password, &pg_host, &pg_dbname).await?;
    tracing::info!("Connected to: {:?}", connection);
    tracing::info!("Running migrations if any are needed");
    Migrator::up(&connection, None).await?;

    Ok(connection)
}

async fn db_connect(
    pg_user: &str,
    password: &str,
    pg_host: &str,
    pg_dbname: &str,
) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let url = format!(
        "postgres://{}:{}@{}:5432/{}",
        pg_user,
        password.trim(),
        pg_host,
        pg_dbname,
    );

    trace!("Attempting Connection to: {}", &url);

    match Database::connect(&url).await {
    //match PgPoolOptions::new().connect(&url).await {
        Ok(connection) => Ok(connection),
        Err(e) => Err(e),
    }
}
