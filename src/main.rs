mod compile;
mod compilers;
mod jwt;
mod system;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "I am Compilet."
}

/// The main function to launch the server
#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(compile::WasmCache {
            dir: PathBuf::from("cache"),
        })
        .mount(
            "/",
            routes![index, system::system, compile::compile, jwt::validate],
        )
}
