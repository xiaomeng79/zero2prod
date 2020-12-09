use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuraton
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Redirect all `log`'s events to our subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    // Postgres connection pool
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    run(listener, connection_pool)?.await
}
