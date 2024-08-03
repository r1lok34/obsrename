use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
};

use crate::models::{Action, KeyStatus};
use crate::{KEYLOCK, SENDER};

unsafe extern "system" fn keyboard_hook(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code >= 0 {
        let kb = &*(l_param as *const KBDLLHOOKSTRUCT);

        // Example to block all key presses, you can add conditions here
        if let Ok(sender) = SENDER.lock() {
            if w_param == WM_KEYDOWN as usize {
                let _ = sender.send(Action::Key((KeyStatus::Press, kb.vkCode)));
            }
            if w_param == WM_KEYUP as usize {
                let _ = sender.send(Action::Key((KeyStatus::Release, kb.vkCode)));
            }
        }
        let klck = KEYLOCK.lock().unwrap().clone();
        if kb.vkCode == 36 {
            if klck == true {
            } else {
                return 1;
            }
        } else {
            if klck == true {
                return 1; // Block the key press
            }
        }
    }
    CallNextHookEx(null_mut(), code, w_param, l_param)
}

pub fn start_keyboard_hook() {
    std::thread::spawn(|| unsafe {
        let h_instance = GetModuleHandleW(null_mut());
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), h_instance, 0);
        if hook.is_null() {
            eprintln!("Failed to set hook");
            return;
        }

        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        UnhookWindowsHookEx(hook);
    });
}
