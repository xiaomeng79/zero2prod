use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuraton
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Postgres connection pool
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    run(listener, connection_pool)?.await
}
