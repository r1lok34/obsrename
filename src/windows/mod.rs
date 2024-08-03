use std::mem;
use winapi::shared::minwindef::UINT;
use winapi::um::winuser::{
    INPUT_u, SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
};

pub mod keyboard_listener;

pub fn type_symbol(unicode_char: u16) {
    unsafe {
        // Create a KEYBDINPUT struct for the key press event
        let key_press_input = INPUT {
            type_: INPUT_KEYBOARD,
            u: {
                let mut empty: INPUT_u = mem::zeroed();

                let ki = empty.ki_mut();

                ki.wVk = 0;
                ki.wScan = unicode_char;
                ki.dwFlags = KEYEVENTF_UNICODE;
                ki.time = 0;
                ki.dwExtraInfo = 0;

                empty
            },
        };

        // Create a KEYBDINPUT struct for the key release event
        let key_release_input = INPUT {
            type_: INPUT_KEYBOARD,
            u: {
                let mut empty: INPUT_u = mem::zeroed();

                let ki = empty.ki_mut();

                ki.wVk = 0;
                ki.wScan = unicode_char;
                ki.dwFlags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
                ki.time = 0;
                ki.dwExtraInfo = 0;

                empty
            },
        };

        // Array of inputs
        let inputs = [key_press_input, key_release_input];

        // Send the key press and release events
        SendInput(
            inputs.len() as UINT,
            inputs.as_ptr() as *mut INPUT,
            mem::size_of::<INPUT>() as i32,
        );
    }
}
