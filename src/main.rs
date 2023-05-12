/**
 * A web server that accepts code and returns the compiled wasm.
 */
mod compile;
mod compilers;
mod jwt;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

/// A simple route for the root of the website
#[get("/")]
fn index() -> &'static str {
    "I am Compilet."
}

/// The main function to launch the Rocket server
#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(compile::WasmCache {
            dir: PathBuf::from("cache"),
        })
        .mount("/", routes![index, compile::compile, jwt::validate])
}
