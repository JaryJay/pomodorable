// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![load_tasks])
  .invoke_handler(tauri::generate_handler![save_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn load_tasks() -> String {
  println!("Hello from Rust!");
  "This is the return value".to_string()
}

#[tauri::command]
fn save_data() {
  println!("Placeholder");
}
