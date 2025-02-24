// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, Event, EventType};
use std::time::{SystemTime, UNIX_EPOCH};

struct MouseEvent {
    event_type: String,
    x: f64,
    y: f64,
    timestamp: u64,
}

struct KeyboardEvent {
    event_type: String,
    key: String,
    code: String,
    timestamp: u64,
}

fn track_event(event: Event) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let log_entry = match event.event_type {
        EventType::MouseMove { x, y } => {
            let mouse_event = MouseEvent {
                event_type: "MouseMove".to_string(),
                x,
                y,
                timestamp,
            };
            format!(
                "{},{},{},{}",
                mouse_event.event_type, mouse_event.x, mouse_event.y, mouse_event.timestamp
            )
        }

        EventType::ButtonPress(btn) | EventType::ButtonRelease(btn) => {
            let mouse_event = MouseEvent {
                event_type: format!("MouseButton {:?}", btn),
                x: 0.0,
                y: 0.0,
                timestamp,
            };
            format!(
                "{},{},{},{}",
                mouse_event.event_type, mouse_event.x, mouse_event.y, mouse_event.timestamp
            )
        }

        EventType::KeyPress(key) | EventType::KeyRelease(key) => {
            let keyboard_event = KeyboardEvent {
                event_type: format!("{:?}", event.event_type),
                key: format!("{:?}", key),
                code: format!("{:?}", key),
                timestamp,
            };
            format!(
                "{},{},{},{}",
                keyboard_event.event_type,
                keyboard_event.key,
                keyboard_event.code,
                keyboard_event.timestamp
            )
        }

        EventType::Wheel { delta_x, delta_y } => {
            format!("Wheel, {}, {}, {}", delta_x, delta_y, timestamp)
        }
    };

    println!("{}", log_entry);
}

fn main() {
    //   track_event_lib::run()
    if let Err(error) = listen(track_event) {
        eprint!("Error: {:?}", error);
    }
}
