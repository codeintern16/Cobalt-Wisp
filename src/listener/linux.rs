use std::sync::Arc;
use std::thread;

use evdev::{InputEventKind, Key};

use super::GlobalInput;

pub fn start<F>(on_event: F)
where
    F: Fn(GlobalInput) + Send + Sync + 'static,
{
    let on_event = Arc::new(on_event);
    let mut count = 0;

    for (_path, mut device) in evdev::enumerate() {
        let Some(keys) = device.supported_keys() else {
            continue;
        };

        let is_keyboard = keys.contains(Key::KEY_A) && keys.contains(Key::KEY_ENTER);
        let is_mouse = keys.contains(Key::BTN_LEFT);

        if !is_keyboard && !is_mouse {
            continue;
        }

        count += 1;
        let cb = Arc::clone(&on_event);
        let name = device.name().unwrap_or("unknown").to_string();

        thread::spawn(move || {
            eprintln!("[listener] 開始監聽: {name}");
            loop {
                match device.fetch_events() {
                    Ok(events) => {
                        for ev in events {
                            // 1 = 按下（略過 0 = 放開、2 = 長按連發）
                            if ev.value() != 1 {
                                continue;
                            }
                            if let InputEventKind::Key(key) = ev.kind() {
                                match key {
                                    Key::BTN_LEFT => cb(GlobalInput::MouseLeft),
                                    Key::BTN_RIGHT => cb(GlobalInput::MouseRight),
                                    // Linux keycode 0x100 以下為標準鍵盤按鍵，避開觸控板的 BTN_TOUCH 等訊號
                                    k if k.code() < 0x100 => cb(GlobalInput::KeyPress),
                                    _ => {}
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[listener] 裝置中斷 ({name}): {e}");
                        break;
                    }
                }
            }
        });
    }

    if count == 0 {
        eprintln!("[listener] 找不到可讀取的輸入裝置，請確認已加入 input 群組並重新登入");
    }
}
