mod config;
mod db;
mod models;
mod routes;
mod riot;

use crate::config::Config;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use anyhow::Result;
use listenfd::ListenFd;
use sqlx::PgPool;
use tera::Tera;
use riven::{RiotApi, RiotApiConfig};

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let mut listenfd = ListenFd::from_env();

    // Delete latest log file
    match std::fs::read("output.log") {
        Ok(_) => std::fs::remove_file("output.log").unwrap(),
        Err(_) => println!("no log file"),
    };

    match setup_logger() {
        Ok(()) => {}
        Err(err) => println!("Error: {}, while configuring logger", err),
    };

    let pool = PgPool::connect(&config.db_address).await?;

    let config_clone = config.clone();
    let mut server = HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let config = config_clone.clone();
        let mut awc = awc::Client::default();
        let riot_client = RiotApi::with_key(&config.riot_api_key);

        App::new()
            .data(config.clone())
            .data(pool.clone())
            .data(tera)
            .app_data(awc)
            .app_data(riot_client)
            .wrap(Cors::permissive().max_age(3600))
            .configure(routes::endpoints)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&config.address)?,
    };

    server.run().await?;

    Ok(())
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
