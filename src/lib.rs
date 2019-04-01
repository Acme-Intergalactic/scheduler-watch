#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;
extern crate reqwest;

use core::cmp::min;
use std::env;
pub mod heroku;
pub mod postgres;

/// Check number of late schedules and respond by turning on temporary dynos with ./manage.py extendschedules command.  
/// Requires two environment variables:  
/// HEROKU_APP_NAME=blipboards  
/// HEROKU_API_KEY=some_secret_key  
pub fn monitor_schedules() -> (u64, u32) {
    // 1. Get current db url.  (Relies on environment variables for Heroku creds and app name.)
    let db_url = heroku::db_url().expect("Heroku isn't returning a db url.");

    // 2. Using that url, find out how many schedules are late.
    let conn = postgres::connection(&db_url);
    let late_count = postgres::late_schedule_count(&conn);

    // 3. If any schedules are late, fix that.
    // 3a.  Assume 30 seconds per schedule per dyno and try to get through them within 10 minutes.  So 20 per dyno.
    let needed = match late_count {
        0 => 0,
        n => (n / 20) + 1,
    } as u32;


    // 3b.  However, the max number of dynos allowed is 50.  So don't have more than 25.
    let default_max_dynos = 25;
    let max_dynos = match env::var("MAX_DYNOS") {
        Ok(s) => min(s.parse::<u32>().unwrap(), default_max_dynos),
        Err(_) => default_max_dynos
    };
    let allowed_dynos = min(needed, max_dynos);

    // 3c.  Start the required dynos, but make sure that they die within 10 minutes.
    for _i in 0..allowed_dynos {
        heroku::run_command("./manage.py extendschedules", 600)
    }
    (late_count, allowed_dynos)
}
