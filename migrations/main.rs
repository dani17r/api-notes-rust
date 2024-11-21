use confik::{Configuration, EnvSource};
use dotenvy::dotenv;
use std::fs;
use std::path::Path;
use tokio_postgres::NoTls;

mod run;
mod database;

use crate::database::{config::ServerConfig, errors::MyError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = ServerConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();
    let client = pool.get().await.map_err(MyError::PoolError)?;

    let migration_files = run::main();

    for file_path in migration_files {
        let current_path = &format!("migrations/{}.sql", &file_path);
        if Path::new(current_path).exists() {
            let sql_commands = fs::read_to_string(current_path)?;
            for command in sql_commands.split(';') {
                let trimmed_command = command.trim();
                if !trimmed_command.is_empty() {
                    client.execute(trimmed_command, &[]).await?;
                }
            }
            println!("Ejecutado: {}", current_path);
        } else {
            eprintln!("El archivo no existe: {}", current_path);
        }
    }

    println!("Migraciones ejecutadas con éxito.");

    std::process::exit(0);
}
