use crate::detect::{DetectState, DETECTING, DETECTION_STATE, DETECT_KEY};
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::RwLock;
use std::thread::spawn;
use tauri::Emitter;
use tungstenite::accept;

lazy_static! {
    pub static ref DETECT_AREA: ((AtomicU32, AtomicU32), (AtomicU32, AtomicU32), (AtomicU32, AtomicU32), (AtomicU32, AtomicU32)) = (
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域1的起始点
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域1的终点
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域2的起始点
        (AtomicU32::new(0), AtomicU32::new(0)), // 区域2的终点
    );
    pub static ref LEFT_CLICK_POSITION: RwLock<Vec<(i32, i32)>> = RwLock::new(vec![]);
    pub static ref RIGHT_CLICK_POSITION: RwLock<Vec<(i32, i32)>> = RwLock::new(vec![]);
    pub static ref KEY_BIND: RwLock<HashMap<String, (i32, bool)>> = RwLock::new(HashMap::new());
    static ref MOUSE_POSITION_X: AtomicU32 = AtomicU32::new(0);
    static ref MOUSE_POSITION_Y: AtomicU32 = AtomicU32::new(0);
    static ref CLICKED: AtomicBool = AtomicBool::new(false);
}

#[tauri::command]
pub fn delete_click_position(index: i32) {
    let mut positions = LEFT_CLICK_POSITION.write().unwrap();
    positions[index as usize] = (0, 0);

    let mut positions = RIGHT_CLICK_POSITION.write().unwrap();
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

pub fn init(app_handle: tauri::AppHandle) {
    println!("listen on port 35806");
    let server = TcpListener::bind("127.0.0.1:35806").unwrap();
    std::thread::spawn(move || loop {
        for stream in server.incoming() {
            let handle1 = app_handle.clone();
            spawn (move || {
                let Ok(mut websocket) = accept(stream.unwrap()) else {return;};
                loop {
                    let msg = websocket.read().unwrap();

                    if msg.is_text() {
                        callback(msg.to_text().unwrap(), handle1.clone());
                    }
                }
            });
        }
    });
}

fn callback(e: &str, app_handle: tauri::AppHandle) {
    app_handle.emit("key_pressed", e).unwrap();
    if e == *DETECT_KEY.read().unwrap() { 
        let current_detecting = !DETECTING.load(std::sync::atomic::Ordering::Acquire);
        DETECTING.store(current_detecting, std::sync::atomic::Ordering::Release);
        
        app_handle.emit("detection_pause_state", current_detecting).unwrap();
        
        if !current_detecting { 
            let mut state = DETECTION_STATE.write().unwrap();
            *state = DetectState::Idle;
        }
    }
    
    let Some(&(index, left)) = KEY_BIND.read().unwrap().get(e) else {
        println!("Key pressed: {:?}", KEY_BIND.read().unwrap());
        return;
    };

    let pos = if left {
        *LEFT_CLICK_POSITION.read().unwrap().get(index as usize).unwrap_or(&(0, 0))
    } else {
        *RIGHT_CLICK_POSITION.read().unwrap().get(index as usize).unwrap_or(&(0, 0))
    };

    if pos == (0, 0) {
        return;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.move_mouse(pos.0, pos.1, Coordinate::Abs).unwrap();
    enigo.button(Button::Left, Direction::Click).unwrap();
}

#[tauri::command]
pub fn set_key_bind(id: i32, char: String, left: bool) {
    println!("Bind {}", char);
    KEY_BIND.write().unwrap().insert(char, (id, left));
}

#[tauri::command]
pub fn click_all_left() {
    let positions = LEFT_CLICK_POSITION.read().unwrap();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(enigo::Button::Left, enigo::Direction::Release).unwrap();
    for (x, y) in positions.iter() {
        if *x != 0 && *y != 0 {
            enigo.move_mouse(*x, *y, Coordinate::Abs).unwrap();
            enigo.button(enigo::Button::Left, enigo::Direction::Click).unwrap();
            std::thread::sleep(std::time::Duration::from_micros(500));
        }
    }
}

#[tauri::command]
pub fn click_all_right() {
    let positions = RIGHT_CLICK_POSITION.read().unwrap();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(enigo::Button::Left, enigo::Direction::Release).unwrap();
    for (x, y) in positions.iter() {
        if *x != 0 && *y != 0 {
            enigo.move_mouse(*x, *y, Coordinate::Abs).unwrap();
            enigo.button(enigo::Button::Left, enigo::Direction::Click).unwrap();
            std::thread::sleep(std::time::Duration::from_micros(500));
        }
    }
}