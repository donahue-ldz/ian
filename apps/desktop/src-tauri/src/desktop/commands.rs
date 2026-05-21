use std::sync::Mutex;

use tauri::State;

use crate::{
    app::IanRuntime,
    protocol::{BehaviorMode, IanAction, IanEvent, IanState, Position},
};

#[tauri::command]
pub fn handle_ian_event(
    event: IanEvent,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<IanAction>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .handle_event(event)
}

#[tauri::command]
pub fn get_ian_state(runtime: State<'_, Mutex<IanRuntime>>) -> Result<IanState, String> {
    Ok(runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .state())
}

#[tauri::command]
pub fn save_window_position(
    position: Position,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<IanAction>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_position(position)
}

#[tauri::command]
pub fn get_settings(runtime: State<'_, Mutex<IanRuntime>>) -> Result<IanState, String> {
    get_ian_state(runtime)
}

#[tauri::command]
pub fn save_behavior_mode(
    mode: BehaviorMode,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_behavior_mode(mode)
}

#[tauri::command]
pub fn save_reminders_enabled(
    enabled: bool,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_reminders_enabled(enabled)
}

#[tauri::command]
pub fn save_capability_enabled(
    capability: String,
    enabled: bool,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_capability_enabled(capability, enabled)
}
