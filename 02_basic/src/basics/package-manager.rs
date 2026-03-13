// You can add an external crate to your project by running `cargo add crate_name`

use chrono::Utc;

fn main() {
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);
}