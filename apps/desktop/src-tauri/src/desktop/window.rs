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
            let resolved_position =
                resolve_initial_window_position(saved_position, *origin, *size, window_size);
            window.set_position(resolved_position)?;
            if let Some(runtime) = app.try_state::<Mutex<IanRuntime>>() {
                if let Ok(mut runtime) = runtime.lock() {
                    let _ = runtime.save_position(Position {
                        x: f64::from(resolved_position.x),
                        y: f64::from(resolved_position.y),
                    });
                }
            }
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
    let position = saved_position
        .and_then(restorable_position)
        .unwrap_or_else(|| {
            default_bottom_right_position(monitor_origin, monitor_size, window_size)
        });

    clamp_window_position(position, monitor_origin, monitor_size, window_size)
}

fn default_bottom_right_position(
    monitor_origin: PhysicalPosition<i32>,
    monitor_size: PhysicalSize<u32>,
    window_size: PhysicalSize<u32>,
) -> PhysicalPosition<i32> {
    let x = monitor_origin.x + monitor_size.width.saturating_sub(window_size.width + 48) as i32;
    let y = monitor_origin.y + monitor_size.height.saturating_sub(window_size.height + 72) as i32;
    PhysicalPosition::new(x, y)
}

pub fn reset_webview_window_position(
    window: &tauri::WebviewWindow,
) -> tauri::Result<PhysicalPosition<i32>> {
    let Some(monitor) = window.current_monitor()? else {
        return Ok(PhysicalPosition::new(0, 0));
    };

    let position =
        default_bottom_right_position(*monitor.position(), *monitor.size(), window.outer_size()?);
    window.set_position(position)?;
    Ok(position)
}

fn clamp_window_position(
    position: PhysicalPosition<i32>,
    monitor_origin: PhysicalPosition<i32>,
    monitor_size: PhysicalSize<u32>,
    window_size: PhysicalSize<u32>,
) -> PhysicalPosition<i32> {
    let min_x = monitor_origin.x;
    let min_y = monitor_origin.y;
    let max_x = monitor_origin.x + monitor_size.width.saturating_sub(window_size.width) as i32;
    let max_y = monitor_origin.y + monitor_size.height.saturating_sub(window_size.height) as i32;

    PhysicalPosition::new(
        position.x.clamp(min_x, max_x),
        position.y.clamp(min_y, max_y),
    )
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

    #[test]
    fn saved_position_is_clamped_to_visible_monitor_bounds() {
        let negative = resolve_initial_window_position(
            Some(Position {
                x: -120.0,
                y: -80.0,
            }),
            PhysicalPosition::new(0, 0),
            PhysicalSize::new(1728, 1117),
            PhysicalSize::new(260, 260),
        );
        let too_far = resolve_initial_window_position(
            Some(Position {
                x: 4000.0,
                y: 3000.0,
            }),
            PhysicalPosition::new(0, 0),
            PhysicalSize::new(1728, 1117),
            PhysicalSize::new(260, 260),
        );

        assert_eq!(negative, PhysicalPosition::new(0, 0));
        assert_eq!(too_far, PhysicalPosition::new(1468, 857));
    }
}
