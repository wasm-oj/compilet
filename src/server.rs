use std::net::Ipv4Addr;
use std::path::PathBuf;

use rocket::Build;
use rocket::Config;
use rocket::Rocket;

use crate::compile;
use crate::jwt;
use crate::system;

#[get("/")]
fn index() -> &'static str {
    "I am Compilet."
}

/// Get the Rocket instance
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .configure(Config {
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Config::default()
        })
        .manage(compile::WasmCache {
            dir: PathBuf::from("cache"),
        })
        .mount(
            "/",
            routes![index, system::system, compile::compile, jwt::validate],
        )
}
