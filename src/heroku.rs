use reqwest::header::ACCEPT;
use std::collections::HashMap;
use std::env;

struct HerokuConfig {
    app_name: String,
    api_key: String,
}

impl HerokuConfig {
    fn new() -> HerokuConfig {
        let app_name = env::var("HEROKU_APP_NAME").expect("HEROKU_APP_NAME must be set");
        let api_key = env::var("HEROKU_API_KEY").expect("HEROKU_API_KEY must be set");
        HerokuConfig{app_name, api_key}
    }
}

#[allow(dead_code)]
fn print_response(response: &mut reqwest::Response) {
    println!("Status: {}", response.status());
    println!("Headers:\n{:?}", response.headers());
    println!("{}", response.text().expect("Response has no text."));
}

/// Get all the target Heroku app environment variables.
fn get_config() -> HashMap<String, String> {
    let envs = HerokuConfig::new();
    let url = format!(
        "https://api.heroku.com/apps/{}/config-vars",
        envs.app_name
    );

    let client = reqwest::Client::new();
    let mut res = client
        .get(&url)
        .header(ACCEPT, "application/vnd.heroku+json; version=3")
        .bearer_auth(&envs.api_key)
        .send()
        .expect("Is your internet down?");
    res.json().expect("Missing json.")
}

/// Create a temporary dyno and run `command` string on it.  
/// `time_to_live` is in seconds.  
/// Example:  
/// ```run_command(&"./manage.py extendschedules", 600)```
pub fn run_command(command: &str, time_to_live: u32) {
    let envs = HerokuConfig::new();

    let url = format!("https://api.heroku.com/apps/{}/dynos", envs.app_name);

    let mut map = HashMap::new();
    map.insert("command", command);
    let ttl = time_to_live.to_string();
    map.insert("time_to_live", &ttl);

    let client = reqwest::Client::new();
    let _res = client
        .post(&url)
        .header(ACCEPT, "application/vnd.heroku+json; version=3")
        .bearer_auth(&envs.api_key)
        .json(&map)
        .send()
        .expect("Is your internet down?");
}

/// Get DATABASE_URL for current app from Heroku via get_config.
pub fn db_url() -> Option<String> {
    match get_config().get("DATABASE_URL") {
        Some(value) => Some(value.to_string()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() {
        dotenv::dotenv().ok();
    }

    #[test]
    fn test_get_config() {
        setup();
        println!("{:#?}", get_config())
    }

    #[test]
    fn test_db_url() {
        setup();
        println!("{}", db_url().unwrap())
    }

    #[test]
    fn test_run_command() {
        setup();
        run_command(&"bash", 1);
    }
}
