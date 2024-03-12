// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_tasks,
            save_data,
            update_savefile_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_tasks(app_handle: tauri::AppHandle) -> Result<String, String> {
    let mut path = app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap_or(std::path::PathBuf::new());
    fs::create_dir_all(&path).unwrap();
    path.push("savefilepath.txt");
    
    let save_path_string = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Err("Could not read savefilepath.txt".into()),
    };

    let save_path = if save_path_string.is_empty() {
        default_savepath(app_handle)
    } else {
        PathBuf::from(save_path_string)
    };
    match fs::read_to_string(save_path) {
        Ok(s) => Ok(s),
        Err(_) => Err("Something went wrong".into()),
    }
}

#[tauri::command]
fn save_data(app_handle: tauri::AppHandle, data: String) -> Result<(), String> {
    let mut path = app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap_or(std::path::PathBuf::new());
    fs::create_dir_all(&path).unwrap();
    path.push("savefilepath.txt");

    let file = File::create(&path);
    if file.is_err() {
        return Err(format!("Could not read file: {}", &file.err().unwrap()).into());
    }
    let save_path_string = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Err("Could not read savefile!".into()),
    };
    let save_path = if save_path_string.is_empty() {
        default_savepath(app_handle)
    } else {
        PathBuf::from(save_path_string)
    };
    match fs::write(save_path, data) {
        Ok(()) => (),
        Err(_) => return Err("Could not write to savefile!".into()),
    };

    Ok(())
}

#[tauri::command]
fn update_savefile_path(app_handle: tauri::AppHandle, new_path: String) -> Result<(), String> {
    let mut path = app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap_or(std::path::PathBuf::new());
    path.push("savefile.txt");
    let mut file = match File::create(&path) {
        Ok(s) => s,
        Err(_) => return Err("Could not create file".into()),
    };
    match file.write_all(new_path.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Could not write to file".into()),
    };
    Ok(())
}

fn default_savepath(app_handle: tauri::AppHandle) -> PathBuf {
    let mut path = app_handle.path_resolver().app_data_dir().unwrap();
    path.push("save.md");
    path
}
