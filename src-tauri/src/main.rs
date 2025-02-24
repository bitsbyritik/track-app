// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, Event, EventType};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

const FILE_PATH: &str = "event.csv";

#[derive(Serialize)]
struct MouseEvent {
    event_type: String,
    x: f64,
    y: f64,
    timestamp: u64,
}

#[derive(Serialize)]
struct KeyboardEvent {
    event_type: String,
    key: String,
    code: String,
    timestamp: u64,
}

fn initialize_file() {
    if std::fs::metadata(FILE_PATH).is_err() {
        let mut file = File::create(FILE_PATH).expect("Failed to create a file");
        file.write_all(b"{}").expect("Failed to initialize_file");
    }
}

fn read_file() -> Value {
    let mut file = File::open(FILE_PATH).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    serde_json::from_str(&content).unwrap_or_else(|_| json!({}))
}

fn write_to_csv(data: &Value) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open file");

    let json_string = serde_json::to_string_pretty(data).expect("Failed in json_string");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write");
}

fn track_event(event: Event) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let log_event = match event.event_type {
        EventType::MouseMove { x, y } => {
            let mouse_event = MouseEvent {
                event_type: "MouseMove".to_string(),
                x,
                y,
                timestamp,
            };
            serde_json::to_value(mouse_event).expect("Failed to Serialize")
        }

        EventType::ButtonPress(btn) | EventType::ButtonRelease(btn) => {
            let mouse_event = MouseEvent {
                event_type: format!("MouseButton {:?}", btn),
                x: 0.0,
                y: 0.0,
                timestamp,
            };
            serde_json::to_value(mouse_event).expect("Failed to Serialize")
        }

        EventType::KeyPress(key) | EventType::KeyRelease(key) => {
            let keyboard_event = KeyboardEvent {
                event_type: format!("{:?}", event.event_type),
                key: format!("{:?}", key),
                code: format!("{:?}", key),
                timestamp,
            };
            serde_json::to_value(keyboard_event).expect("Failed to Serialize")
        }

        EventType::Wheel { delta_x, delta_y } => json!({
            "event_type": "Wheel",
            "delta_x": delta_x,
            "delta_y": delta_y,
            "timestamp": timestamp
        }),
    };

    let mut data = read_file();

    if let Some(array) = data["new"].as_array_mut() {
        array.push(log_event);
    } else {
        data["new"] = json!([log_event]);
    }

    write_to_csv(&data);
}

fn main() {
    //   track_event_lib::run()

    initialize_file();

    if let Err(error) = listen(track_event) {
        eprint!("Error: {:?}", error);
    }
}
