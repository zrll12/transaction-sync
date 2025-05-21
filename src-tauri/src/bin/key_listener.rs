use rdev::{listen, EventType, Key};
use tungstenite::{client, Message};

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:35806";
    let (mut ws_stream, _) = client::connect(url).unwrap();
    
    listen(move |e| {
        if let EventType::KeyPress(key) = e.event_type {
            if let Some(key_str) = key_type_to_string(key) {
                println!("{}", key_str);
                ws_stream.send(Message::text(key_str)).unwrap();
            }
        }
    }).unwrap();
}

fn key_type_to_string(key: Key) -> Option<String> {
    match key {
        Key::Space => {Some(" ".to_string())}
        Key::Num1 => {Some("1".to_string())}
        Key::Num2 => {Some("2".to_string())}
        Key::Num3 => {Some("3".to_string())}
        Key::Num4 => {Some("4".to_string())}
        Key::Num5 => {Some("5".to_string())}
        Key::Num6 => {Some("6".to_string())}
        Key::Num7 => {Some("7".to_string())}
        Key::Num8 => {Some("8".to_string())}
        Key::Num9 => {Some("9".to_string())}
        Key::Num0 => {Some("0".to_string())}
        Key::Minus => {Some("-".to_string())}
        Key::Equal => {Some("=".to_string())}
        Key::KeyQ => {Some("q".to_string())}
        Key::KeyW => {Some("w".to_string())}
        Key::KeyE => {Some("e".to_string())}
        Key::KeyR => {Some("r".to_string())}
        Key::KeyT => {Some("t".to_string())}
        Key::KeyY => {Some("y".to_string())}
        Key::KeyU => {Some("u".to_string())}
        Key::KeyI => {Some("i".to_string())}
        Key::KeyO => {Some("o".to_string())}
        Key::KeyP => {Some("p".to_string())}
        Key::LeftBracket => {Some("[".to_string())}
        Key::RightBracket => {Some("]".to_string())}
        Key::KeyA => {Some("a".to_string())}
        Key::KeyS => {Some("s".to_string())}
        Key::KeyD => {Some("d".to_string())}
        Key::KeyF => {Some("f".to_string())}
        Key::KeyG => {Some("g".to_string())}
        Key::KeyH => {Some("h".to_string())}
        Key::KeyJ => {Some("j".to_string())}
        Key::KeyK => {Some("k".to_string())}
        Key::KeyL => {Some("l".to_string())}
        Key::SemiColon => {Some(";".to_string())}
        Key::Quote => {Some("'".to_string())}
        Key::BackSlash => {Some("\\".to_string())}
        Key::KeyZ => {Some("z".to_string())}
        Key::KeyX => {Some("x".to_string())}
        Key::KeyC => {Some("c".to_string())}
        Key::KeyV => {Some("v".to_string())}
        Key::KeyB => {Some("b".to_string())}
        Key::KeyN => {Some("n".to_string())}
        Key::KeyM => {Some("m".to_string())}
        Key::Comma => {Some(",".to_string())}
        Key::Dot => {Some(".".to_string())}
        Key::Slash => {Some("/".to_string())}
        Key::Pause => {Some("Pause".to_string())}
        _ => None
    }
}