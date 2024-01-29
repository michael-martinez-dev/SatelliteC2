mod satellite;
mod satellite;

use std::thread::sleep;
use std::time::Duration;
use satellite::Satellite;

fn main() {
    let mut sat1: Satellite = Satellite::new("Sat01".to_string());
    let mut delta_time = 0.0;
    println!("Starting simulation...");

    loop {
        sat1.update(delta_time);
        sleep(Duration::from_secs(1));
        delta_time += 1.0;
    }
}
