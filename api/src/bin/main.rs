#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]

use pulstats::configuration::get_configuration;
use pulstats::startup::{prepare, run};
use pulstats::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "pulstats=debug,tower_http=debug")
    }

    let subscriber = get_subscriber(
        "pulstats".into(),
        "info".into(),
        std::io::stdout,
        Some(configuration.tracing.get_collector_endpoint()),
    );

    init_subscriber(subscriber);

    let app = prepare(&configuration).await?;

    let address =
        format!("{}:{}", &configuration.application.host, &configuration.application.port);
    let listener = TcpListener::bind(&address)?;

    run(app, listener).await?;

    Ok(())
}
