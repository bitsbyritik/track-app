// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, Event, EventType};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{Manager, State};

const FILE_PATH: &str = "event.csv";

struct AppState {
    task_name: Mutex<Option<String>>,
    tracking_status: Arc<AtomicBool>,
}

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

fn track_event(event: Event, state: &State<AppState>) {
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
    let task_name = state
        .task_name
        .lock()
        .unwrap()
        .clone()
        .unwrap_or_else(|| "default".to_string());

    if let Some(array) = data[task_name.clone()].as_array_mut() {
        array.push(log_event);
    } else {
        data[task_name] = json!([log_event]);
    }

    write_to_csv(&data);
}

#[tauri::command]
fn set_task_name(task: String, state: State<AppState>) {
    let mut task_name = state.task_name.lock().unwrap();
    *task_name = Some(task);
    println!("Task name set to: {}", task_name.clone().unwrap());
}

#[tauri::command]
fn start_tracking(state: State<AppState>) {
    state.tracking_status.store(true, Ordering::Relaxed);
    println!("Tracking Started");
}

#[tauri::command]
fn stop_tracking(state: State<AppState>) {
    state.tracking_status.store(false, Ordering::Relaxed);
    println!("Tracking Stopped");
}

fn main() {
    initialize_file();

    let state = AppState {
        task_name: Mutex::new(None),
        tracking_status: Arc::new(AtomicBool::new(false)),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            set_task_name,
            start_tracking,
            stop_tracking
        ])
        .setup(|app| {
            let handle = app.handle();
            let handle_clone = handle.clone();
            let tracking_flag = app.state::<AppState>().tracking_status.clone();

            std::thread::spawn(move || {
                if let Err(error) = listen(move |event| {
                    if tracking_flag.load(Ordering::Relaxed) {
                        let state = handle_clone.state::<AppState>();
                        track_event(event, &state);
                    }
                }) {
                    eprint!("Error: {:?}", error);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running application");
}
