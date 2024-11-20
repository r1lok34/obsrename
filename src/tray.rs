// use std::{ffi::OsStr, mem::zeroed, os::windows::ffi::OsStrExt, ptr::null_mut};

// use winapi::{
//     shared::windef::HWND,
//     um::{
//         libloaderapi::GetModuleHandleW,
//         winuser::{
//             CreateWindowExW, DefWindowProcW, LoadCursorW, LoadIconW, PostQuitMessage, CS_HREDRAW,
//             CS_VREDRAW, WNDCLASSW, WS_OVERLAPPEDWINDOW,
//         },
//     },
// };

// unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> isize {
//     match msg {
//         winapi::um::winuser::WM_DESTROY => {
//             PostQuitMessage(0);
//         }
//         32868 => {
//             match lparam as u32 {
//                 winapi::um::winuser::WM_LBUTTONDOWN => {
//                     println!("Left button down on the notify icon");
//                 }
//                 winapi::um::winuser::WM_RBUTTONDOWN => {
//                     println!("Right button down on the notify icon");
//                     // You could show a context menu here if you want.
//                 }
//                 _ => (),
//             }
//         }
//         _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
//     }
//     0
// }

// #[allow(non_snake_case)]
// pub fn tray_init() -> winapi::um::shellapi::NOTIFYICONDATAW {
//     let mut wc: WNDCLASSW = unsafe { zeroed() };

//     let h_instance = unsafe { GetModuleHandleW(null_mut()) };

//     unsafe {
//         wc.style = CS_HREDRAW | CS_VREDRAW;
//         wc.lpfnWndProc = Some(window_proc);
//         wc.hInstance = h_instance;
//         wc.hIcon = LoadIconW(null_mut(), winapi::um::winuser::IDI_APPLICATION);
//         wc.hCursor = LoadCursorW(null_mut(), winapi::um::winuser::IDC_ARROW);
//     }

//     let WM_MYMESSAGE = winapi::um::winuser::WM_APP + 100;

//     let trayToolTip = "obsrename".to_string();
//     let mut trayToolTipInt: [u16; 128] = [0; 128];
//     let trayToolTipStrStep: &str = &*trayToolTip;
//     let trayToolTipStepOS = OsStr::new(trayToolTipStrStep);
//     let trayToolTipStepUTF16 = trayToolTipStepOS.encode_wide().collect::<Vec<u16>>();
//     trayToolTipInt[..trayToolTipStepUTF16.len()].copy_from_slice(&trayToolTipStepUTF16);

//     let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = unsafe { zeroed() };

//     let hwnd = unsafe {
//         CreateWindowExW(
//             0,
//             wc.lpszClassName,
//             to_wide_str("Tray App").as_ptr(),
//             WS_OVERLAPPEDWINDOW,
//             0,
//             0,
//             300,
//             200,
//             null_mut(),
//             null_mut(),
//             h_instance,
//             null_mut(),
//         )
//     };

//     if hwnd.is_null() {
//         log::error!("failed to create window");
//     }

//     unsafe {
//         nid.cbSize = size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32;
//         nid.hWnd = hwnd;
//         nid.uID = 1001;
//         nid.uCallbackMessage = WM_MYMESSAGE;
//         nid.hIcon =
//             winapi::um::winuser::LoadIconW(null_mut(), winapi::um::winuser::IDI_APPLICATION);
//         nid.szTip = trayToolTipInt;
//         nid.uFlags = winapi::um::shellapi::NIF_MESSAGE
//             | winapi::um::shellapi::NIF_ICON
//             | winapi::um::shellapi::NIF_TIP;
//     }

//     unsafe {
//         winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid);
//     }

//     nid
// }

// pub fn tray_hide(nid: *mut winapi::um::shellapi::NOTIFYICONDATAW) {
//     unsafe {
//         winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, nid);
//     }
// }

// fn to_wide_str(str: &str) -> Vec<u16> {
//     use std::os::windows::ffi::OsStrExt;
//     std::ffi::OsStr::new(str)
//         .encode_wide()
//         .chain(std::iter::once(0))
//         .collect()
// }
