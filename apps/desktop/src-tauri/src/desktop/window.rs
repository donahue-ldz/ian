use tauri::{App, Manager, PhysicalPosition};

pub fn configure_main_window(app: &mut App) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        if let Some(monitor) = window.primary_monitor()? {
            let origin = monitor.position();
            let size = monitor.size();
            let window_size = window.outer_size()?;
            let x = origin.x + size.width.saturating_sub(window_size.width + 48) as i32;
            let y = origin.y + size.height.saturating_sub(window_size.height + 72) as i32;
            window.set_position(PhysicalPosition::new(x as i32, y as i32))?;
        }
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}
