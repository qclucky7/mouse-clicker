// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod mouse;
mod pusher;
mod tray;
use mouse::{listen_event, INTERVAL_MILLISECONDS};

#[tauri::command]
fn set_mouse_interval(interval_milliseconds: u64) -> bool {
    unsafe { INTERVAL_MILLISECONDS = interval_milliseconds };
    return true;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            tray::generate_tray(_app);
            tauri::async_runtime::spawn(async move {
                listen_event();
            });
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_mouse_interval])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::Ready => {
                pusher::GLOBAL_APP_HANDLE
                    .set(_app_handle.clone())
                    .expect("Failed to set global app handle");
            }
            _ => {}
        })
}
