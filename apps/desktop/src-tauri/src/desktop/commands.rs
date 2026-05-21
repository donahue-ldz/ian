use std::sync::Mutex;

use tauri::State;

use crate::{
    app::IanRuntime,
    protocol::{
        BehaviorMode, BuildTestStatus, DeveloperSnooze, DeveloperWorkspace, IanAction, IanEvent,
        IanState, PlayfulEnergy, Position, QuietHours,
    },
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
pub fn save_quiet_hours(
    quiet_hours: QuietHours,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_quiet_hours(quiet_hours)
}

#[tauri::command]
pub fn save_creature_settings(
    movement_intensity: String,
    bubble_frequency: String,
    rest_behavior: String,
    playful_energy: PlayfulEnergy,
    playful_snoozed_until_ms: Option<i64>,
    surface_scale: f64,
    diagnostics_enabled: bool,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_creature_settings(
            movement_intensity,
            bubble_frequency,
            rest_behavior,
            playful_energy,
            playful_snoozed_until_ms,
            surface_scale,
            diagnostics_enabled,
        )
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

#[tauri::command]
pub fn save_developer_workspace(
    workspace: DeveloperWorkspace,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_developer_workspace(workspace)
}

#[tauri::command]
pub fn save_developer_snooze(
    snooze: DeveloperSnooze,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_developer_snooze(snooze)
}

#[tauri::command]
pub fn ingest_build_test_summary(
    workspace_id: Option<String>,
    tool: String,
    status: BuildTestStatus,
    duration_ms: u64,
    tests_total: u32,
    tests_failed: u32,
    error_kind: Option<String>,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<IanAction>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .ingest_build_test_summary(
            workspace_id,
            tool,
            status,
            duration_ms,
            tests_total,
            tests_failed,
            error_kind,
        )
}
