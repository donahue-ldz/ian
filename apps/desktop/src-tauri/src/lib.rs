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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(Mutex::new(app::bootstrap::bootstrap_runtime()))
        .invoke_handler(tauri::generate_handler![
            desktop::commands::handle_ian_event,
            desktop::commands::get_ian_state,
            desktop::commands::save_window_position,
            desktop::commands::reset_window_position,
            desktop::commands::get_settings,
            desktop::commands::save_behavior_mode,
            desktop::commands::save_active_pet,
            desktop::commands::save_quiet_hours,
            desktop::commands::save_creature_settings,
            desktop::commands::save_reminders_enabled,
            desktop::commands::save_reminder_settings,
            desktop::commands::save_do_not_disturb,
            desktop::commands::save_privacy_onboarding_seen,
            desktop::commands::save_find_ian_shortcut_enabled,
            desktop::commands::save_capability_enabled,
            desktop::commands::save_developer_workspace,
            desktop::commands::save_developer_snooze,
            desktop::commands::ingest_build_test_summary
        ])
        .setup(|app| {
            desktop::window::configure_main_window(app)?;
            desktop::tray::install_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Ian desktop app");
}
