use tauri::{menu::Menu, tray::TrayIconBuilder, App, Manager};

pub fn install_tray(app: &mut App) -> tauri::Result<()> {
    let menu = Menu::new(app)?;
    let _tray = TrayIconBuilder::new()
        .tooltip("Ian")
        .menu(&menu)
        .on_tray_icon_event(|tray, _event| {
            if let Some(window) = tray.app_handle().get_webview_window("main") {
                let _ = window.show();
            }
        })
        .build(app)?;

    Ok(())
}
