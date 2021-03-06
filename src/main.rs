use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, email_client::EmailClient};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Redirect all `log`'s events to our subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    // Panic if we can't read configuraton
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Postgres connection pool
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );
    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;
    run(listener, connection_pool, email_client)?.await
    // Ok(())
}
