use super::compress;
use super::cors;
use super::routes;
use super::version;
use crate::compile;
use crate::config::*;
use rocket::serde::{Deserialize, Serialize};
use rocket::Build;
use rocket::Config;
use rocket::Rocket;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerInfo {
    pub version: String,
    pub commit: String,
    pub data: String,
    pub os: String,
}

/// Get the Rocket instance
pub fn rocket() -> Rocket<Build> {
    let server = rocket::build()
        .configure(Config {
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            port: server_port(),
            ..Config::default()
        })
        .manage(compile::WasmCache {
            dir: PathBuf::from(cache_dir()),
        })
        .mount(
            "/",
            routes![
                routes::index,
                routes::info,
                routes::system,
                routes::compile,
                routes::validate
            ],
        );

    let server = server.attach(version::fairing());

    let server = if !no_cors() {
        server.attach(cors::CORS)
    } else {
        server
    };

    if cfg!(debug_assertions) {
        server
    } else {
        server.attach(compress::fairing())
    }
}
