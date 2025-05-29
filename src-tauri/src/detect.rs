use crate::click::{click_all_left, click_all_right};
use base64::Engine;
use lazy_static::lazy_static;
use opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT;
use opencv::core::{Point, Scalar, Vector};
use opencv::imgproc;
use opencv::prelude::*;
use serde::Serialize;
use serde_json::json;
use std::cmp::PartialEq;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
use tauri::{Emitter, WebviewUrl, WebviewWindowBuilder};
use xcap::image::GenericImageView;
use xcap::Monitor;

lazy_static! {
  pub static ref DETECTION_STATE: RwLock<DetectState> = RwLock::new(DetectState::Idle);
  pub static ref DETECTING: AtomicBool = AtomicBool::new(true);
  pub static ref DETECT_KEY: RwLock<String> = RwLock::new("".to_string());
  pub static ref CAN_CLICK: AtomicBool = AtomicBool::new(true);
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Serialize)]
pub enum DetectState {
    Idle,
    LeftDetected,
    RightDetected,
}

pub fn init(app_handle: tauri::AppHandle) {
    // 初始化xcap
    let monitors = Monitor::all().expect("Failed to get monitors");
    if monitors.is_empty() {
        panic!("No monitors found");
    }

    // 新建窗口
    WebviewWindowBuilder::new(
        &app_handle,
        "Capture-Preview",
        WebviewUrl::App("preview".into()),
    )
    .title("Capture Preview")
    .center()
    .build()
    .map_err(|e| e.to_string())
    .unwrap()
    .show()
    .unwrap();

    // 启动检测线程
    std::thread::spawn(move || {
        loop {
            if !CAN_CLICK.load(Ordering::Acquire) {
              continue;
            }
            if !DETECTING.load(Ordering::Acquire) {
                continue;
            }

            // 获取检测区域1坐标
            let x1_1 = crate::click::DETECT_AREA
                .0
                 .0
                .load(std::sync::atomic::Ordering::Relaxed) as i32;
            let y1_1 = crate::click::DETECT_AREA
                .0
                 .1
                .load(std::sync::atomic::Ordering::Relaxed) as i32;
            let x2_1 = crate::click::DETECT_AREA
                .1
                 .0
                .load(std::sync::atomic::Ordering::Relaxed) as i32;
            let y2_1 = crate::click::DETECT_AREA
                .1
                 .1
                .load(std::sync::atomic::Ordering::Relaxed) as i32;

            // 获取检测区域2坐标
            let x1_2 = crate::click::DETECT_AREA
                .2
                 .0
                .load(std::sync::atomic::Ordering::Relaxed) as i32;
            let y1_2 = crate::click::DETECT_AREA
                .2
                 .1
                .load(std::sync::atomic::Ordering::Relaxed) as i32;
            let x2_2 = crate::click::DETECT_AREA
                .3
                 .0
                .load(std::sync::atomic::Ordering::Relaxed) as i32;
            let y2_2 = crate::click::DETECT_AREA
                .3
                 .1
                .load(std::sync::atomic::Ordering::Relaxed) as i32;

            if x1_1 == 0
                || y1_1 == 0
                || x2_1 == 0
                || y2_1 == 0
                || x1_2 == 0
                || y1_2 == 0
                || x2_2 == 0
                || y2_2 == 0
            {
                // 如果坐标无效，则跳过处理
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            let scale_factor = if tauri_plugin_os::platform().contains("windows") {
                1.0 // Windows does not change the position  when scaling
            } else {
                Monitor::from_point(x1_1, y1_1)
                    .unwrap()
                    .scale_factor()
                    .unwrap()
            };
            let mut state = DETECTION_STATE.write().unwrap();

            // 处理区域1
            let width_1 = (x2_1 - x1_1).abs() * scale_factor as i32;
            let height_1 = (y2_1 - y1_1).abs() * scale_factor as i32;
            let x_1 = x1_1.min(x2_1) * scale_factor as i32;
            let y_1 = y1_1.min(y2_1) * scale_factor as i32;
            if width_1 > 0 && height_1 > 0 {
                let left = capture_screen_region(
                    &app_handle,
                    x_1,
                    y_1,
                    width_1,
                    height_1,
                    "update-preview-1",
                )
                .unwrap();
                // 1. 如果idle并且检测到物体，则设置为左检测状态
                // 2. 如果是左检测状态但没有检测到物体，则设置为idle状态
                if *state == DetectState::Idle && left > 0 {
                    *state = DetectState::LeftDetected;
                    click_all_left();
                } else if *state == DetectState::LeftDetected && left == 0 {
                    *state = DetectState::Idle;
                    click_all_right();
                }
            }

            // 处理区域2
            let width_2 = (x2_2 - x1_2).abs() * scale_factor as i32;
            let height_2 = (y2_2 - y1_2).abs() * scale_factor as i32;
            let x_2 = x1_2.min(x2_2) * scale_factor as i32;
            let y_2 = y1_2.min(y2_2) * scale_factor as i32;
            if width_2 > 0 && height_2 > 0 {
                let right = capture_screen_region(
                    &app_handle,
                    x_2,
                    y_2,
                    width_2,
                    height_2,
                    "update-preview-2",
                )
                .unwrap();

                if *state == DetectState::Idle && right > 0 {
                    *state = DetectState::RightDetected;
                    click_all_right();
                } else if *state == DetectState::RightDetected && right == 0 {
                    *state = DetectState::Idle;
                    click_all_left();
                }
            }

            app_handle.emit("detection_state", *state).unwrap();

            // 等待0.5毫秒
            std::thread::sleep(std::time::Duration::from_micros(500));
        }
    });
}

pub fn capture_screen_region(
    app_handle: &tauri::AppHandle,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    event_name: &str,
) -> Result<u32, String> {
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
        Scalar::all(0.0),
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

    // 边缘检测
    let mut edges = Mat::default();
    imgproc::canny(&mat, &mut edges, 50.0, 150.0, 3, false).map_err(|e| e.to_string())?;

    // 查找轮廓
    let mut contours = Vector::<Vector<Point>>::new();
    let hierarchy = Mat::default();
    imgproc::find_contours(
        &edges,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )
    .map_err(|e| e.to_string())?;

    // 绘制轮廓a
    let mut result = Mat::default();
    imgproc::cvt_color(
        &edges,
        &mut result,
        imgproc::COLOR_GRAY2BGR,
        0,
        ALGO_HINT_DEFAULT,
    )
    .unwrap();
    for i in 0..contours.len() {
        imgproc::draw_contours(
            &mut result,
            &contours,
            i as i32,
            Scalar::new(0.0, 255.0, 0.0, 255.0),
            2,
            imgproc::LINE_8,
            &hierarchy,
            0,
            Point::new(0, 0),
        )
        .map_err(|e| e.to_string())?;
    }

    // 将图像转换为base64字符串
    let mut encoded_buffer = Vector::new();
    opencv::imgcodecs::imencode(".png", &result, &mut encoded_buffer, &Vector::new())
        .map_err(|e| e.to_string())?;
    let base64_image = base64::engine::general_purpose::STANDARD.encode(&encoded_buffer);

    // 发送图像数据和物体数量到前端
    let data = json!({
        "image": base64_image,
        "count": contours.len()
    });
    app_handle
        .emit(event_name, data)
        .map_err(|e| e.to_string())?;

    Ok(contours.len() as u32)
}

#[tauri::command]
pub fn set_detection_key(key: String) {
    let mut detect_key = DETECT_KEY.write().unwrap();
    *detect_key = key;
}
#[tauri::command]
pub fn set_click_state(state: bool){
  CAN_CLICK.store(state, Ordering::Relaxed)
}