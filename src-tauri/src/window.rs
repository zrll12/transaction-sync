use tauri::{CursorIcon, Emitter, Manager, WebviewUrl, WebviewWindowBuilder, Window, WindowBuilder, WindowEvent};
use tauri::async_runtime::handle;
use tauri::utils::config::WindowConfig;
use tauri::window::Color;
use crate::click::{CLICK_POSITION, DETECT_AREA};

#[tauri::command]
pub async fn open_select_window(window: Window, handle: tauri::AppHandle, index: i32) -> Result<(), String> {
    let window_label = format!("{}", index);
    
    let window = WebviewWindowBuilder::new(
        &handle,
        window_label,
        WebviewUrl::App("dialog".into()),
    )
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
pub async fn close_select_window(window: Window, handle: tauri::AppHandle) -> Result<(), String> {
    let pos = window.outer_position().unwrap();
    let index = window.label().parse::<i32>().unwrap();
    if index == 0 { 
        DETECT_AREA.0.0.store(pos.x as u32, std::sync::atomic::Ordering::Relaxed);
        DETECT_AREA.0.1.store(pos.y as u32, std::sync::atomic::Ordering::Relaxed);
        handle.emit("set_detect_area1", (pos.x, pos.y)).map_err(|e| e.to_string())?;
    } else if index == 1 {
        DETECT_AREA.1.0.store(pos.x as u32, std::sync::atomic::Ordering::Relaxed);
        DETECT_AREA.1.1.store(pos.y as u32, std::sync::atomic::Ordering::Relaxed);
        handle.emit("set_detect_area2", (pos.x, pos.y)).map_err(|e| e.to_string())?;
    } else { 
        let member_index = index - 2;
        let mut positions = CLICK_POSITION.lock().unwrap();
        if member_index >= positions.len() as i32 {
            positions.resize(member_index as usize + 1, (0, 0))
        }
        positions[member_index as usize] = (pos.x, pos.y);
        handle.emit("set_click_position", positions.clone()).map_err(|e| e.to_string())?;
    }
    
    
    window.close().map_err(|e| e.to_string())
}