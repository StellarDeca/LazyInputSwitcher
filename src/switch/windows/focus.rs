//! 焦点窗口句柄 FFI 封装

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

/// 获取前台焦点窗口句柄
pub(super) fn get_ground_handle() -> Result<HWND, Box<dyn std::error::Error>> {
    let handle = unsafe { GetForegroundWindow() };
    if handle.is_invalid() {
        Err("Failed to get foreground window".into())
    } else {
        Ok(handle)
    }
}
