use std::sync::Mutex;

use serde::Serialize;
use tauri::State;

use crate::{
    app::IanRuntime,
    protocol::{
        BehaviorMode, BuildTestStatus, DeveloperSnooze, DeveloperWorkspace, IanAction, IanEvent,
        IanState, PlayfulEnergy, Position, QuietHours,
    },
};

#[derive(Debug, Clone, Serialize)]
pub struct MemoryCandidateView {
    pub id: i64,
    pub tags: Vec<String>,
    pub status: String,
    pub created_at_ms: i64,
    pub confirmed_at_ms: Option<i64>,
}

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
pub fn reset_window_position(
    window: tauri::WebviewWindow,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<IanAction>, String> {
    let position =
        super::window::reset_webview_window_position(&window).map_err(|error| error.to_string())?;

    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_position(Position {
            x: f64::from(position.x),
            y: f64::from(position.y),
        })
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
pub fn save_active_pet(
    active_pet_id: String,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_active_pet(active_pet_id)
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
pub fn save_reminder_settings(
    enabled: bool,
    interval_minutes: u16,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_reminder_settings(enabled, interval_minutes)
}

#[tauri::command]
pub fn save_do_not_disturb(
    enabled: bool,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_do_not_disturb(enabled)
}

#[tauri::command]
pub fn save_privacy_onboarding_seen(
    seen: bool,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_privacy_onboarding_seen(seen)
}

#[tauri::command]
pub fn save_find_ian_shortcut_enabled(
    enabled: bool,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .save_find_ian_shortcut_enabled(enabled)
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

#[tauri::command]
pub fn list_memory_candidates(
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<MemoryCandidateView>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .memory_candidates()
        .map(|records| records.into_iter().map(MemoryCandidateView::from).collect())
}

#[tauri::command]
pub fn confirm_memory_candidate(
    id: i64,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<MemoryCandidateView>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .confirm_memory_candidate(id)
        .map(|records| records.into_iter().map(MemoryCandidateView::from).collect())
}

#[tauri::command]
pub fn delete_memory_candidate(
    id: i64,
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<MemoryCandidateView>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .delete_memory_candidate(id)
        .map(|records| records.into_iter().map(MemoryCandidateView::from).collect())
}

#[tauri::command]
pub fn clear_memory_candidates(
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<MemoryCandidateView>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .clear_memory_candidates()
        .map(|records| records.into_iter().map(MemoryCandidateView::from).collect())
}

#[tauri::command]
pub fn export_memory_summary(
    runtime: State<'_, Mutex<IanRuntime>>,
) -> Result<Vec<MemoryCandidateView>, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .export_memory_summary()
        .map(|records| records.into_iter().map(MemoryCandidateView::from).collect())
}

#[tauri::command]
pub fn clear_interaction_journal(runtime: State<'_, Mutex<IanRuntime>>) -> Result<(), String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .clear_interaction_journal()
}

#[tauri::command]
pub fn reset_local_settings(runtime: State<'_, Mutex<IanRuntime>>) -> Result<IanState, String> {
    runtime
        .lock()
        .map_err(|_| "Ian runtime lock poisoned".to_string())?
        .reset_local_settings()
}

impl From<crate::storage::repositories::memory_repo::MemoryCandidate> for MemoryCandidateView {
    fn from(candidate: crate::storage::repositories::memory_repo::MemoryCandidate) -> Self {
        Self {
            id: candidate.id,
            tags: candidate.tags.split(',').map(str::to_string).collect(),
            status: candidate.status,
            created_at_ms: candidate.created_at_ms,
            confirmed_at_ms: candidate.confirmed_at_ms,
        }
    }
}
