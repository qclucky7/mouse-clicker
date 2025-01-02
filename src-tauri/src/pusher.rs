use tauri::{AppHandle, Emitter};
use serde::Serialize;
use once_cell::sync::OnceCell;

pub static GLOBAL_APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MouseClickStatus {
    opened: bool
}

pub fn open_mouse_click(opened: bool) {
    if let Some(app) = GLOBAL_APP_HANDLE.get() {
        app.emit("open_mouse_click", MouseClickStatus{
            opened,
        }).unwrap();
    }    
}