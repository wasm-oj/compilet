use crate::compilers::get_compilers;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
    compiling: u32,
    pending: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SystemInfo {
    capabilities: HashMap<String, String>,
    status: Status,
}

#[get("/system")]
pub fn system() -> Json<SystemInfo> {
    let compilers = get_compilers();

    let capabilities = compilers.iter().map(|c| (c.lang(), c.describe())).collect();

    Json(SystemInfo {
        capabilities,
        status: Status {
            compiling: 0,
            pending: 0,
        },
    })
}
