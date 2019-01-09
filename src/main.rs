extern crate monitor_schedules;
extern crate dotenv;

fn main() {
    dotenv::dotenv().ok();
    monitor_schedules::monitor_schedules();
}