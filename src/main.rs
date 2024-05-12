use std::net::TcpListener;

use env_logger::Env;
use sqlx::mysql::MySqlPoolOptions;

use lokoda_backend::configuration::*;
use lokoda_backend::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = MySqlPoolOptions::new()
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to database");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
