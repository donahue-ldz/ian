use std::sync::Mutex;

use tauri::State;

use crate::{
    app::IanRuntime,
    protocol::{IanAction, IanEvent, IanState, Position},
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
