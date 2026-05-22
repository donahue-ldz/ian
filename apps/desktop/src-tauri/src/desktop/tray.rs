use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};

pub fn install_tray(app: &mut App) -> tauri::Result<()> {
    let find_ian = MenuItem::with_id(app, "find_ian", "找回 Ian", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&find_ian])?;
    let _tray = TrayIconBuilder::new()
        .tooltip("Ian")
        .menu(&menu)
        .on_menu_event(|app, event| {
            if event.id().as_ref() != "find_ian" {
                return;
            }

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        })
        .on_tray_icon_event(|tray, _event| {
            if let Some(window) = tray.app_handle().get_webview_window("main") {
                let _ = window.show();
            }
        })
        .build(app)?;

    Ok(())
}
