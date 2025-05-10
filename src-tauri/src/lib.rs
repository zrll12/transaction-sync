mod click;
mod window;
mod detect;

use click::{delete_click_position, move_mouse};
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
            move_mouse
        ])
        .setup(|app| {
            detect::init(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
