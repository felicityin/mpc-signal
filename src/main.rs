mod lobby;
mod ws;
use lobby::Lobby;
mod messages;
mod start_connection;
use actix::Actor;
use start_connection::start_connection as start_connection_route;
mod protos;

use actix_web::{web, App, HttpServer};

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    let chat_server = Lobby::default().start(); // create and spin up a lobby

    log::info!("initializing server on port: {}", PORT);

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/health").to(|| async { "ok" }))
            .service(start_connection_route)
            .app_data(web::Data::new(chat_server.clone())) // register the lobby
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await?;

    log::info!("server stoped: {}", PORT);

    Ok(())
}
