// this hides the console for Windows release builds
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_window_state;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(|app| {
        let window = app.get_window("main").unwrap();
        window.eval("window.location.replace('https://unlimcloud.cloud/');").unwrap();
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
