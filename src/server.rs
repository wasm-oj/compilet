use std::path::PathBuf;

use rocket::Build;
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
        .manage(compile::WasmCache {
            dir: PathBuf::from("cache"),
        })
        .mount(
            "/",
            routes![index, system::system, compile::compile, jwt::validate],
        )
}
