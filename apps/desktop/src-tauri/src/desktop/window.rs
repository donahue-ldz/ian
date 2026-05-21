use std::sync::Mutex;

use tauri::{App, Manager, PhysicalPosition, PhysicalSize};

use crate::{app::IanRuntime, protocol::Position};

pub fn configure_main_window(app: &mut App) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        let saved_position = app
            .try_state::<Mutex<IanRuntime>>()
            .and_then(|runtime| runtime.lock().ok().map(|runtime| runtime.state().position));

        if let Some(monitor) = window.primary_monitor()? {
            let origin = monitor.position();
            let size = monitor.size();
            let window_size = window.outer_size()?;
            window.set_position(resolve_initial_window_position(
                saved_position,
                *origin,
                *size,
                window_size,
            ))?;
        }
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}

fn resolve_initial_window_position(
    saved_position: Option<Position>,
    monitor_origin: PhysicalPosition<i32>,
    monitor_size: PhysicalSize<u32>,
    window_size: PhysicalSize<u32>,
) -> PhysicalPosition<i32> {
    if let Some(position) = saved_position.and_then(restorable_position) {
        return position;
    }

    let x = monitor_origin.x + monitor_size.width.saturating_sub(window_size.width + 48) as i32;
    let y = monitor_origin.y + monitor_size.height.saturating_sub(window_size.height + 72) as i32;
    PhysicalPosition::new(x, y)
}

fn restorable_position(position: Position) -> Option<PhysicalPosition<i32>> {
    if !position.x.is_finite() || !position.y.is_finite() {
        return None;
    }

    if position.x == 0.0 && position.y == 0.0 {
        return None;
    }

    Some(PhysicalPosition::new(
        position.x.round() as i32,
        position.y.round() as i32,
    ))
}

#[cfg(test)]
mod tests {
    use tauri::{PhysicalPosition, PhysicalSize};

    use super::resolve_initial_window_position;
    use crate::protocol::Position;

    #[test]
    fn saved_position_wins_over_bottom_right_default() {
        let position = resolve_initial_window_position(
            Some(Position { x: 144.0, y: 233.0 }),
            PhysicalPosition::new(0, 0),
            PhysicalSize::new(1728, 1117),
            PhysicalSize::new(260, 260),
        );

        assert_eq!(position, PhysicalPosition::new(144, 233));
    }

    #[test]
    fn missing_saved_position_uses_bottom_right_default() {
        let position = resolve_initial_window_position(
            None,
            PhysicalPosition::new(0, 0),
            PhysicalSize::new(1728, 1117),
            PhysicalSize::new(260, 260),
        );

        assert_eq!(position, PhysicalPosition::new(1420, 785));
    }
}
