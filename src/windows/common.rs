use crate::rdev::{Button, EventType};
use crate::windows::keyboard::Keyboard;
use crate::windows::keycodes::key_from_code;
use crate::windows::{DWORD, LONG, MOUSE_BACKWARD, MOUSE_FORWARD, WORD};
use lazy_static::lazy_static;
use std::convert::TryInto;
use std::os::raw::c_int;
use std::sync::Mutex;
use windows_sys::Win32::Foundation::{GetLastError, WPARAM};
use windows_sys::Win32::Foundation::{HLOCAL, LPARAM, LRESULT};
use windows_sys::Win32::UI::WindowsAndMessaging::HHOOK;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExA, KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, WHEEL_DELTA, WH_KEYBOARD_LL, WH_MOUSE_LL,
    WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP, WM_XBUTTONDOWN, WM_XBUTTONUP,
};
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

#[inline]
fn hiword(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}

pub static mut HOOK: HHOOK = 0;
lazy_static! {
    pub(crate) static ref KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new().unwrap());
}

pub unsafe fn get_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    kb.vkCode
}

pub unsafe fn get_scan_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    kb.scanCode
}

pub unsafe fn get_point(lpdata: LPARAM) -> (LONG, LONG) {
    let mouse = *(lpdata as *const MSLLHOOKSTRUCT);
    (mouse.pt.x, mouse.pt.y)
}

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644986(v=vs.85)
/// confusingly, this function returns a WORD (unsigned), but may be
/// interpreted as either signed or unsigned depending on context
pub unsafe fn get_delta(lpdata: LPARAM) -> WORD {
    let mouse = *(lpdata as *const MSLLHOOKSTRUCT);
    hiword(mouse.mouseData)
}
pub unsafe fn get_button_code(lpdata: LPARAM) -> WORD {
    let mouse = *(lpdata as *const MSLLHOOKSTRUCT);
    hiword(mouse.mouseData)
}

pub unsafe fn convert(param: WPARAM, lpdata: LPARAM) -> Option<EventType> {
    match param.try_into() {
        Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            Some(EventType::KeyPress(key))
        }
        Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            Some(EventType::KeyRelease(key))
        }
        Ok(WM_LBUTTONDOWN) => Some(EventType::ButtonPress(Button::Left)),
        Ok(WM_LBUTTONUP) => Some(EventType::ButtonRelease(Button::Left)),
        Ok(WM_MBUTTONDOWN) => Some(EventType::ButtonPress(Button::Middle)),
        Ok(WM_MBUTTONUP) => Some(EventType::ButtonRelease(Button::Middle)),
        Ok(WM_RBUTTONDOWN) => Some(EventType::ButtonPress(Button::Right)),
        Ok(WM_RBUTTONUP) => Some(EventType::ButtonRelease(Button::Right)),
        Ok(WM_XBUTTONDOWN) => {
            let code = get_button_code(lpdata) as u8;
            match code {
                num if num == MOUSE_FORWARD => Some(EventType::ButtonPress(Button::Forward)),
                num if num == MOUSE_BACKWARD => Some(EventType::ButtonPress(Button::Backward)),
                num => Some(EventType::ButtonPress(Button::Unknown(num))),
            }
        }
        Ok(WM_XBUTTONUP) => {
            let code = get_button_code(lpdata) as u8;
            match code {
                num if num == MOUSE_FORWARD => Some(EventType::ButtonRelease(Button::Forward)),
                num if num == MOUSE_BACKWARD => Some(EventType::ButtonRelease(Button::Backward)),
                num => Some(EventType::ButtonRelease(Button::Unknown(num))),
            }
        }
        Ok(WM_MOUSEMOVE) => {
            let (x, y) = get_point(lpdata);
            Some(EventType::MouseMove {
                x: x as f64,
                y: y as f64,
            })
        }
        Ok(WM_MOUSEWHEEL) => {
            let delta = get_delta(lpdata);
            Some(EventType::Wheel {
                delta_x: 0,
                delta_y: (delta.checked_div(hiword(WHEEL_DELTA)).unwrap_or_default()) as i64,
            })
        }
        Ok(WM_MOUSEHWHEEL) => {
            let delta = get_delta(lpdata);
            Some(EventType::Wheel {
                delta_x: (delta.checked_div(hiword(WHEEL_DELTA)).unwrap_or_default()) as i64,
                delta_y: 0,
            })
        }
        _ => None,
    }
}

type RawCallback = unsafe extern "system" fn(code: c_int, param: WPARAM, lpdata: LPARAM) -> LRESULT;
pub enum HookError {
    Mouse(DWORD),
    Key(DWORD),
}

pub unsafe fn set_key_hook(callback: RawCallback) -> Result<(), HookError> {
    let hmod: HLOCAL = 0;
    let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), hmod, 0);

    if hook == 0 {
        let error = GetLastError();
        return Err(HookError::Key(error));
    }
    HOOK = hook;
    Ok(())
}

pub unsafe fn set_mouse_hook(callback: RawCallback) -> Result<(), HookError> {
    let hmod: HLOCAL = 0;
    let hook = SetWindowsHookExA(WH_MOUSE_LL, Some(callback), hmod, 0);
    if hook == 0 {
        let error = GetLastError();
        return Err(HookError::Mouse(error));
    }
    HOOK = hook;
    Ok(())
}
