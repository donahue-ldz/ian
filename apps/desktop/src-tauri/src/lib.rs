pub mod adapters;
pub mod app;
pub mod core;
pub mod desktop;
pub mod domain;
pub mod protocol;
pub mod resources;
pub mod security;
pub mod storage;

use std::sync::Mutex;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(app::bootstrap::bootstrap_runtime()))
        .invoke_handler(tauri::generate_handler![
            desktop::commands::handle_ian_event,
            desktop::commands::get_ian_state,
            desktop::commands::save_window_position
        ])
        .setup(|app| {
            desktop::window::configure_main_window(app)?;
            desktop::tray::install_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Ian desktop app");
}
