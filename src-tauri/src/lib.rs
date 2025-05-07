mod window;
mod click;

use window::{open_select_window, close_select_window};
use click::{delete_click_position, move_mouse};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(async {
        click::init();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_select_window, close_select_window, delete_click_position, move_mouse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
