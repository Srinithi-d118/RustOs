use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use spin::Mutex;
use crate::interrupts::KEY_BUFFER;

pub static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
    Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore)
);

pub fn pop_key() -> Option<DecodedKey> {
    // 1. Check PS/2 hardware keyboard scancode ringbuffer
    if let Some(scancode) = KEY_BUFFER.lock().pop() {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            return keyboard.process_keyevent(key_event);
        }
    }

    // 2. Check COM1 serial port input (allows headless terminal interaction)
    if let Some(b) = crate::serial::try_read_byte() {
        match b {
            b'\r' | b'\n' => return Some(DecodedKey::Unicode('\n')),
            0x08 | 0x7F => return Some(DecodedKey::Unicode('\x08')), // Backspace
            0x1B => return Some(DecodedKey::RawKey(KeyCode::Escape)),
            0x20..=0x7E => return Some(DecodedKey::Unicode(b as char)),
            _ => {}
        }
    }

    None
}