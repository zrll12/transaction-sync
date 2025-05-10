use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    pub static ref DETECT_AREA: ((AtomicU32, AtomicU32), (AtomicU32, AtomicU32), (AtomicU32, AtomicU32), (AtomicU32, AtomicU32)) = (
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域1的起始点
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域1的终点
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域2的起始点
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域2的终点
    );
    pub static ref LEFT_CLICK_POSITION: Mutex<Vec<(i32, i32)>> = Mutex::new(vec![]);
    pub static ref RIGHT_CLICK_POSITION: Mutex<Vec<(i32, i32)>> = Mutex::new(vec![]);
    static ref MOUSE_POSITION_X: AtomicU32 = AtomicU32::new(0);
    static ref MOUSE_POSITION_Y: AtomicU32 = AtomicU32::new(0);
    static ref CLICKED: AtomicBool = AtomicBool::new(false);
}

#[tauri::command]
pub fn delete_click_position(index: i32) {
    let mut positions = LEFT_CLICK_POSITION.lock().unwrap();
    positions[index as usize] = (0, 0);

    let mut positions = RIGHT_CLICK_POSITION.lock().unwrap();
    positions[index as usize] = (0, 0);
}

#[tauri::command]
pub fn move_mouse() {
    let x = DETECT_AREA.0 .0.load(std::sync::atomic::Ordering::Acquire) as i32;
    let y = DETECT_AREA.0 .1.load(std::sync::atomic::Ordering::Acquire) as i32;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
}

// fn click_all() {
//     let current_mouse_pos_x = MOUSE_POSITION_X.load(std::sync::atomic::Ordering::Acquire);
//     let current_mouse_pos_y = MOUSE_POSITION_Y.load(std::sync::atomic::Ordering::Acquire);
//     MOUSE_POSITION_X.store(0, std::sync::atomic::Ordering::SeqCst);
//     MOUSE_POSITION_Y.store(0, std::sync::atomic::Ordering::SeqCst);
//     let positions = CLICK_POSITION.lock().unwrap();
//     let mut enigo = Enigo::new(&Settings::default()).unwrap();
//     enigo.button(Button::Left, Direction::Release).unwrap();
//     for (x, y) in positions.iter() {
//         enigo.move_mouse(*x, *y, Coordinate::Abs).unwrap();
//         enigo.button(Button::Left, Direction::Click).unwrap();
//         sleep(Duration::from_millis(100));
//     }
//     enigo.button(Button::Left, Direction::Release).unwrap();
//     enigo
//         .move_mouse(
//             current_mouse_pos_x as i32,
//             current_mouse_pos_y as i32,
//             Coordinate::Abs,
//         )
//         .unwrap();
//     enigo.button(Button::Left, Direction::Release).unwrap();
//     sleep(Duration::from_millis(500));
//     MOUSE_POSITION_X.store(current_mouse_pos_x, std::sync::atomic::Ordering::SeqCst);
//     MOUSE_POSITION_Y.store(current_mouse_pos_y, std::sync::atomic::Ordering::SeqCst);
//     CLICKED.store(false, std::sync::atomic::Ordering::SeqCst);
// }
