use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
    pub compiling: u32,
    pub pending: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SystemInfo {
    pub capabilities: HashMap<String, String>,
    pub status: Status,
}
