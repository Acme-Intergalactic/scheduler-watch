extern crate dotenv;
extern crate monitor_schedules;

fn main() {
    dotenv::dotenv().ok();
    let (n, allowed) = monitor_schedules::monitor_schedules();
    println!("{} late schedules, {} commissioned dynos", n, allowed);
}
