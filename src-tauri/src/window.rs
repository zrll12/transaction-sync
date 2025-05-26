use crate::click::{LEFT_CLICK_POSITION, RIGHT_CLICK_POSITION, DETECT_AREA};
use tauri::{
    Emitter, WebviewUrl, WebviewWindowBuilder, Window
};

#[tauri::command]
pub async fn open_select_window(handle: tauri::AppHandle, label_type: String, index: i32) -> Result<(), String> {
    let window_label = format!("{}_{}", label_type, index);

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
        pos.x as f64 / scale_factor + 45.0,
        pos.y as f64 / scale_factor + 35.0,
    );
    let parts: Vec<&str> = window.label().split('_').collect();
    let label_type = parts[0];
    let index = parts[1].parse::<i32>().unwrap();
    
    match label_type {
        "left" => {
            // 左侧点击位置
            let mut positions = LEFT_CLICK_POSITION.write().unwrap();
            if index >= positions.len() as i32 {
                positions.resize(index as usize + 1, (0, 0))
            }
            positions[index as usize] = (physical_pos.0 as i32, physical_pos.1 as i32);
            handle
                .emit("set_left_click_position", positions.clone())
                .map_err(|e| e.to_string())?
        },
        "right" => {
            // 右侧点击位置
            let mut positions = RIGHT_CLICK_POSITION.write().unwrap();
            if index >= positions.len() as i32 {
                positions.resize(index as usize + 1, (0, 0))
            }
            positions[index as usize] = (physical_pos.0 as i32, physical_pos.1 as i32);
            handle
                .emit("set_right_click_position", positions.clone())
                .map_err(|e| e.to_string())?
        },
        "caption" => {
            match index {
                0 => {
                    // 区域1左上角
                    DETECT_AREA.0.0.store(physical_pos.0 as u32, std::sync::atomic::Ordering::Relaxed);
                    DETECT_AREA.0.1.store(physical_pos.1 as u32, std::sync::atomic::Ordering::Relaxed);
                    handle
                        .emit(
                            "set_detect_area1",
                            (physical_pos.0 as i32, physical_pos.1 as i32),
                        )
                        .map_err(|e| e.to_string())?
                },
                1 => {
                    // 区域1右下角
                    DETECT_AREA.1.0.store(physical_pos.0 as u32, std::sync::atomic::Ordering::Relaxed);
                    DETECT_AREA.1.1.store(physical_pos.1 as u32, std::sync::atomic::Ordering::Relaxed);
                    handle
                        .emit(
                            "set_detect_area2",
                            (physical_pos.0 as i32, physical_pos.1 as i32),
                        )
                        .map_err(|e| e.to_string())?
                },
                2 => {
                    // 区域2左上角
                    DETECT_AREA.2.0.store(physical_pos.0 as u32, std::sync::atomic::Ordering::Relaxed);
                    DETECT_AREA.2.1.store(physical_pos.1 as u32, std::sync::atomic::Ordering::Relaxed);
                    handle
                        .emit(
                            "set_detect_area3",
                            (physical_pos.0 as i32, physical_pos.1 as i32),
                        )
                        .map_err(|e| e.to_string())?
                },
                3 => {
                    // 区域2右下角
                    DETECT_AREA.3.0.store(physical_pos.0 as u32, std::sync::atomic::Ordering::Relaxed);
                    DETECT_AREA.3.1.store(physical_pos.1 as u32, std::sync::atomic::Ordering::Relaxed);
                    handle
                        .emit(
                            "set_detect_area4",
                            (physical_pos.0 as i32, physical_pos.1 as i32),
                        )
                        .map_err(|e| e.to_string())?
                },
                _ => {
                }
            };
        },
        &_ => {}
    }

    window.close().map_err(|e| e.to_string())
}
