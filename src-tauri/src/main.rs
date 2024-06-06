// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rust_rocksdb::DB;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use log::info;
use tauri::Manager;

lazy_static! {
    static ref DB_INSTANCE: Arc<Mutex<Option<DB>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
fn open_db(path: String) -> Result<(), String> {
    let db = DB::open_default(&path).map_err(|e| e.to_string())?;
    let mut db_instance = DB_INSTANCE.lock().unwrap();
    *db_instance = Some(db);
    Ok(())
}

#[tauri::command]
fn get_keys(start: usize, limit: usize, query: Option<String>) -> Result<Vec<String>, String> {
    let db_instance = DB_INSTANCE.lock().unwrap();
    let db = db_instance.as_ref().ok_or("Database not opened")?;
    let iter = db.iterator(rust_rocksdb::IteratorMode::Start);

    let mut keys: Vec<String> = iter
        .filter_map(|result| match result {
            Ok((key, value)) => {
                let key_str = String::from_utf8(key.to_vec()).unwrap();
                let value_str = String::from_utf8(value.to_vec()).unwrap();
                if let Some(ref q) = query {
                    if key_str.contains(q) || value_str.contains(q) {
                        Some(key_str)
                    } else {
                        None
                    }
                } else {
                    Some(key_str)
                }
            }
            Err(_) => None,
        })
        .skip(start)
        .take(limit)
        .collect();

    info!("{:?}", keys);
    Ok(keys)
}

#[tauri::command]
fn get_value(key: String) -> Result<String, String> {
    let db_instance = DB_INSTANCE.lock().unwrap();
    let db = db_instance.as_ref().ok_or("Database not opened")?;
    let value = db.get(key.as_bytes()).map_err(|e| e.to_string())?;
    match value {
        Some(v) => Ok(String::from_utf8(v.to_vec()).unwrap()),
        None => Ok(String::new()),
    }
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this block in debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools(); // Close and reopen to ensure devtools are visible
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_db, get_value, get_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}