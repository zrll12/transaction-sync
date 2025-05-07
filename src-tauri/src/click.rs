use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DETECT_AREA: ((AtomicU32, AtomicU32), (AtomicU32, AtomicU32)) = (
        (AtomicU32::new(0), AtomicU32::new(0)),
        (AtomicU32::new(0), AtomicU32::new(0)),
    );
    pub static ref CLICK_POSITION: Mutex<Vec<(i32, i32)>> = Mutex::new(vec![]);
    static ref MOUSE_POSITION_X: AtomicU32 = AtomicU32::new(0);
    static ref MOUSE_POSITION_Y: AtomicU32 = AtomicU32::new(0);
    static ref INITED: AtomicBool = AtomicBool::new(false);
}