use std::net::Ipv4Addr;
use std::path::PathBuf;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Build;
use rocket::Config;
use rocket::Rocket;

use crate::cli::build;
use crate::compile;
use crate::jwt;
use crate::system;

#[get("/")]
fn index() -> &'static str {
    "I am Compilet."
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerInfo {
    pub version: String,
    pub commit: String,
    pub build_time: String,
    pub os: String,
}

#[get("/info")]
fn info() -> Json<ServerInfo> {
    Json(ServerInfo {
        version: build::PKG_VERSION.to_string(),
        commit: build::COMMIT_HASH.to_string(),
        build_time: build::BUILD_TIME.to_string(),
        os: build::BUILD_OS.to_string(),
    })
}

/// Get the Rocket instance
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .configure(Config {
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "32000".to_string())
                .parse()
                .unwrap(),
            ..Config::default()
        })
        .manage(compile::WasmCache {
            dir: PathBuf::from("cache"),
        })
        .mount(
            "/",
            routes![index, info, system::system, compile::compile, jwt::validate],
        )
}
