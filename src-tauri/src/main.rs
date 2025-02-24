// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, Mouse};
use rdev::{listen, Event, EventType};
use std::time::{SystemTime, UNIX_EPOCH};

struct MouseEvent {
    evnet_type: String,
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
                evnet_type: "MouseMove".to_string(),
                x,
                y,
                timestamp,
            };
            format!(
                "{},{},{},{}\n",
                mouse_event.evnet_type, mouse_event.x, mouse_event.y, mouse_event.timestamp
            );
        }

        EventType::ButtonPress(btn) | EventType::ButtonRelease(btn) => {
            let (x, y) = enigo.mouse_location();

            let mouse_event = MouseEvent {
                evnet_type: format!("MouseButton {:?}", btn),
                x: 

            }
        }
    };
}

fn main() {
    track_event_lib::run()
}
