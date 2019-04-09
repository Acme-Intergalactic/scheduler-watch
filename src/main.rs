extern crate dotenv;
extern crate monitor_schedules;

fn main() {
    dotenv::dotenv().ok();
    let app_name = std::env::args().nth(1).expect("HEROKU_APP_NAME must be set");
    let (n, allowed) = monitor_schedules::monitor_schedules();
    println!("{} late schedules, {} commissioned dynos for app {}", n, allowed, app_name);
}
