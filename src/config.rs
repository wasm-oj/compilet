use std::env;

/// Fetches the server port number from the environment variable "PORT".
/// If the variable is not set or its value cannot be parsed into u16, a default value of 32000 is returned.
pub fn server_port() -> u16 {
    env::var("PORT")
        .unwrap_or("32000".to_owned())
        .parse::<u16>()
        .unwrap_or(32000)
}

/// Fetches the application secret from the environment variable "APP_SECRET".
/// If the variable is not set, a default value of "APP_SECRET" is returned.
pub fn app_secret() -> String {
    env::var("APP_SECRET").unwrap_or("APP_SECRET".to_owned())
}

/// Fetches the cache directory from the environment variable "CACHE_DIR".
/// If the variable is not set, a default value of "cache" is returned.
pub fn cache_dir() -> String {
    env::var("CACHE_DIR").unwrap_or("cache".to_owned())
}

/// Fetches the cache disabled flag from the environment variable "NO_CACHE".
/// If the variable is not set, a default value of false is returned.
pub fn no_cache() -> bool {
    match env::var("NO_CACHE") {
        Ok(val) => val == "1" || val == "true",
        Err(_) => false,
    }
}
