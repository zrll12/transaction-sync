mod click;
mod window;
mod detect;

use click::{delete_click_position, move_mouse, set_key_bind};
use detect::{set_detection_key, set_click_state};
use tauri::async_runtime::spawn;
use window::{close_select_window, open_select_window};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_select_window,
            close_select_window,
            delete_click_position,
            move_mouse,
            set_key_bind,
            set_detection_key,
            set_click_state
        ])
        .setup(|app| {
            click::init(app.handle().clone());
            detect::init(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
