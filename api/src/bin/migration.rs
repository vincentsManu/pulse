#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]

use pulstats::configuration::get_configuration;
use pulstats::startup::get_connection_pool;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "pulstats=debug,tower_http=debug")
    }

    let pool = get_connection_pool(&configuration.database_writable);

    sqlx::migrate!("./migrations").run(&pool).await?;

    // grant select for the read user
    let grant_query = format!(
        "GRANT SELECT ON ALL TABLES IN SCHEMA public TO {user}",
        user = configuration.database.username
    );

    sqlx::query(&grant_query).execute(&pool).await?;

    Ok(())
}
