#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;
extern crate reqwest;

use core::cmp::min;
pub mod heroku;
pub mod postgres;

/// Check number of late schedules and respond by turning on temporary dynos with ./manage.py extendschedules command.  
/// Requires two environment variables:  
/// HEROKU_APP_NAME=blipboards  
/// HEROKU_API_KEY=some_secret_key  
pub fn monitor_schedules() -> (i64, i64) {
    // 1. Get current db url.  (Relies on environment variables for Heroku creds and app name.)
    let db_url = heroku::db_url().expect("Heroku isn't returning a db url.");

    // 2. Using that url, find out how many schedules are late.
    let conn = postgres::connection(&db_url);
    let n = postgres::late_schedule_count(&conn);

    // 3. If any schedules are late, fix that.
    // 3a.  Assume 30 seconds per schedule per dyno and try to get through them within 10 minutes.  So 20 per dyno.
    let needed = match n {
        0 => 0,
        n => (n / 20) + 1
    };

    // 3b.  However, the max number of dynos allowed is 50.  So don't have more than 40.
    let allowed = min(needed, 40);

    // 3c.  Start the required dynos, but make sure that they die within 10 minutes.
    for _i in 0..allowed {
        heroku::run_command("./manage.py extendschedules", 600)
    }
    (n, allowed)
}
