use std::net::TcpListener;

use once_cell::sync::Lazy;
use pulstats::configuration::{get_configuration, DatabaseSettings};
use pulstats::startup::{prepare, run};
use pulstats::telemetry::{get_subscriber, init_subscriber};
use sqlx::{Connection, Executor, PgConnection, PgPool};

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            get_subscriber(subscriber_name, default_filter_level, std::io::stdout, None);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink, None);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub database_name: String,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn post_user_session(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/pulse/user_session", &self.address))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn list_user_sessions(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/manu/user_session", &self.address))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn health_check(&self) -> reqwest::Response {
        self.api_client
            // Use the returned application address
            .get(&format!("{}/healthz", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_stats(&self) -> reqwest::Response {
        self.api_client
            // Use the returned application address
            .get(&format!("{}/pulstats/stats", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        std::env::set_var("APP_ENVIRONMENT", "test");
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = format!(
            "{}-{}-test",
            chrono::Local::now().format("%F-%H-%M-%S"),
            thread_rng().sample_iter(&Alphanumeric).take(3).map(char::from).collect::<String>()
        );
        c.database_writable.database_name = c.database.database_name.clone();
        // Use a random OS port
        c.application.port = 0;

        c
    };

    let address =
        format!("{}:{}", &configuration.application.host, &configuration.application.port);
    let listener = TcpListener::bind(&address).unwrap();
    let application_port = listener.local_addr().unwrap().port();

    // Create and migrate the database
    let pool = configure_database(&configuration.database_writable).await;

    let app = prepare(&configuration).await.unwrap();

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("{}:{}", configuration.application.base_url, application_port),
        port: application_port,
        // db_pool: get_connection_pool_test(&configuration.database).await,
        db_pool: pool,
        database_name: configuration.database.database_name.clone(),
        api_client: client,
    };

    // Launch the application as a background task

    let _ = tokio::spawn(run(app, listener));

    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool =
        PgPool::connect_with(config.with_db()).await.expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    return connection_pool;
}
