#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod nssm;
mod service;

use commands::*;

fn main() {
    // Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_service_cmd,
            remove_service_cmd,
            start_service_cmd,
            stop_service_cmd,
            restart_service_cmd,
            list_all_services_cmd,
            list_nssm_services_cmd,
            get_service_log_cmd
        ])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
