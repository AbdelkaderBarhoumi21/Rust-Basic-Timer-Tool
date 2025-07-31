use std::io::{self, Write};
use std::thread;
use std::time::Duration;
fn main() {
    println!("----Basic Timer Tool----");
    println!("Enter the timer duration (format : hours minutes seconds)");
    let duration = match get_timer_input() {
        Some(Dur) => Dur,
        None => {
            println!("Invalid Input Please neter numbers only");
            return;
        }
    };
    println!(
        "Timer set for: {} hours, {} minutes, {} seconds",
        duration.0, duration.1, duration.2
    );
    start_timer(duration.0, duration.1, duration.2);
    println!("Time's up!");
}
/// Parse user input for hours, minutes, and seconds

fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        println!("Invalid len!");
        return None;
    }
    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let second = parts[2].parse::<u64>().ok()?;
    Some((hours, minutes, second))
}
//start the timer display countdown
fn start_timer(hours: u64, minutes: u64, seconds: u64) {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;
    for i in (1..=total_seconds).rev() {
        let hours = i / 3600;
        let minutes = (i % 3600) / 60;
        let seconds = i % 60;

        print!(
            "\rTime Remaining : {:02}:{:02}:{:02}",
            hours, minutes, seconds
        );
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!(); //Move to new line after countdowns ends
}
