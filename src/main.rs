use std::{thread, time};

fn main() {
    let seconds = 60; 
    println!("Waiting: {} seconds", seconds);

    let duration = time::Duration::from_secs(seconds);

    thread::sleep(duration);
}
