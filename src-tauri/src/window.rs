use crate::click::{CLICK_POSITION, DETECT_AREA};
use tauri::{
    Emitter, WebviewUrl, WebviewWindowBuilder, Window
};

#[tauri::command]
pub async fn open_select_window(handle: tauri::AppHandle, index: i32) -> Result<(), String> {
    let window_label = format!("{}", index);

    let window = WebviewWindowBuilder::new(&handle, window_label, WebviewUrl::App("dialog".into()))
        .inner_size(100.0, 130.0)
        .center()
        .always_on_top(true)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .resizable(false)
        .build()
        .map_err(|e| e.to_string())?;

    window.show().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_preview_window(handle: tauri::AppHandle) -> Result<(), String> {
    let window = WebviewWindowBuilder::new(&handle, "preview", WebviewUrl::App("preview".into()))
        .inner_size(400.0, 300.0)
        .center()
        .always_on_top(true)
        .decorations(true)
        .resizable(true)
        .build()
        .map_err(|e| e.to_string())?;

    window.show().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn close_select_window(window: Window, handle: tauri::AppHandle) -> Result<(), String> {
    let pos = window.inner_position().unwrap();
    let scale_factor = if tauri_plugin_os::platform().contains("windows") { 
        1.0 // Windows does not change the position  when scaling
    } else {
        window.scale_factor().unwrap()
    };
    let physical_pos = (
        pos.x as f64 / scale_factor + 48.0,
        pos.y as f64 / scale_factor + 40.0,
    );
    let index = window.label().parse::<i32>().unwrap();
    if index == 0 {
        DETECT_AREA
            .0
             .0
            .store(physical_pos.0 as u32, std::sync::atomic::Ordering::Relaxed);
        DETECT_AREA
            .0
             .1
            .store(physical_pos.1 as u32, std::sync::atomic::Ordering::Relaxed);
        handle
            .emit(
                "set_detect_area1",
                (physical_pos.0 as i32, physical_pos.1 as i32),
            )
            .map_err(|e| e.to_string())?;
    } else if index == 1 {
        DETECT_AREA
            .1
             .0
            .store(physical_pos.0 as u32, std::sync::atomic::Ordering::Relaxed);
        DETECT_AREA
            .1
             .1
            .store(physical_pos.1 as u32, std::sync::atomic::Ordering::Relaxed);
        handle
            .emit(
                "set_detect_area2",
                (physical_pos.0 as i32, physical_pos.1 as i32),
            )
            .map_err(|e| e.to_string())?;
    } else {
        let member_index = index - 2;
        let mut positions = CLICK_POSITION.lock().unwrap();
        if member_index >= positions.len() as i32 {
            positions.resize(member_index as usize + 1, (0, 0))
        }
        positions[member_index as usize] = (physical_pos.0 as i32, physical_pos.1 as i32);
        handle
            .emit("set_click_position", positions.clone())
            .map_err(|e| e.to_string())?;
    }

    window.close().map_err(|e| e.to_string())
}
