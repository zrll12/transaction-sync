use opencv::core::Vector;
use opencv::prelude::*;
use tauri::{Emitter, WebviewUrl, WebviewWindowBuilder};
use xcap::image::GenericImageView;
use xcap::Monitor;

pub fn init(app_handle: tauri::AppHandle) {
    // 初始化xcap
    let monitors = Monitor::all().expect("Failed to get monitors");
    if monitors.is_empty() {
        panic!("No monitors found");
    }

    // 新建窗口
    WebviewWindowBuilder::new(&app_handle, "Capture-Preview", WebviewUrl::App("preview".into()))
        .center()
        .build()
        .map_err(|e| e.to_string())
        .unwrap()
        .show()
        .unwrap();

    // 启动检测线程
    std::thread::spawn(move || {
        loop {
            // 获取检测区域坐标
            let x1 = crate::click::DETECT_AREA.0.0.load(std::sync::atomic::Ordering::Relaxed) as i32;
            let y1 = crate::click::DETECT_AREA.0.1.load(std::sync::atomic::Ordering::Relaxed) as i32;
            let x2 = crate::click::DETECT_AREA.1.0.load(std::sync::atomic::Ordering::Relaxed) as i32;
            let y2 = crate::click::DETECT_AREA.1.1.load(std::sync::atomic::Ordering::Relaxed) as i32;

            let scale_factor = if tauri_plugin_os::platform().contains("windows") {
                1.0 // Windows does not change the position  when scaling
            } else {
                Monitor::from_point(x1, y1).unwrap().scale_factor().unwrap()
            };

            // 计算检测区域的宽度和高度
            let width = (x2 - x1).abs() * scale_factor as i32;
            let height = (y2 - y1).abs() * scale_factor as i32;

            // 获取左上角坐标
            let x = x1.min(x2) * scale_factor as i32;
            let y = y1.min(y2) * scale_factor as i32;

            // 执行区域检测
            if width > 0 && height > 0 {
                let _ = capture_screen_region(&app_handle, x, y, width, height);
            }

            // 等待100毫秒
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

pub fn capture_screen_region(app_handle: &tauri::AppHandle, x: i32, y: i32, width: i32, height: i32) -> Result<(), String> {
    let monitors = Monitor::all().map_err(|e| e.to_string())?;
    let primary = monitors.first().ok_or("No monitor found")?;

    // 捕获全屏图像
    let full_image = primary.capture_image().map_err(|e| e.to_string())?;

    // 确保坐标在有效范围内
    let x = x.max(0) as u32;
    let y = y.max(0) as u32;
    let width = width.max(0) as u32;
    let height = height.max(0) as u32;

    // 裁剪指定区域
    let cropped = full_image.view(x, y, width, height);

    // 转换为OpenCV格式
    let mut mat = Mat::new_rows_cols_with_default(
        height as i32,
        width as i32,
        opencv::core::CV_8UC4,
        opencv::core::Scalar::all(0.0),
    )
    .map_err(|e| e.to_string())?;

    // Convert image pixels to a contiguous buffer (BGR order for OpenCV)
    let mut buffer = Vec::with_capacity((width * height * 4) as usize);
    for (_, _, pixel) in cropped.pixels() {
        buffer.extend_from_slice(&[pixel[2], pixel[1], pixel[0], pixel[3]]);
    }

    unsafe {
        mat.set_data(buffer.as_ptr());
    }

    // 将图像转换为base64字符串
    let mut encoded_buffer = Vector::new();
    opencv::imgcodecs::imencode(".png", &mat, &mut encoded_buffer, &Vector::new())
        .map_err(|e| e.to_string())?;
    let base64_image =  base64::encode(&encoded_buffer);

    // 发送图像数据到前端
    app_handle
        .emit("update-preview", base64_image)
        .map_err(|e| e.to_string())?;

    Ok(())
}
