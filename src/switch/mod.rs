/*
这个模块主要实现：
    其他语言 <==> 英文 输入法的切换
 */
   
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

use std::error::Error;
use crate::core::InputMethodMode;

pub(super) struct Switcher {
    #[cfg(target_os = "windows")]
    windows_controller: windows::WinInputMethodController,

    #[cfg(target_os = "linux")]
    linux_controller: linux::LinuxController,
}
impl Switcher {
    pub(super) fn new() -> Result<Switcher, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        match windows::WinInputMethodController::new() {
            Ok(windows_controller) => Ok(Switcher { windows_controller }),
            Err(err) => Err(err.into()),
        }

        #[cfg(target_os = "linux")]
        match linux::LinuxController::new() {
            Ok(linux_controller) => Ok(Switcher { linux_controller }),
            Err(err) => Err(err),
        }
    }

    pub(super) fn query(&self) -> Result<InputMethodMode, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        self.windows_controller.get_mode()

        #[cfg(target_os = "linux")]
        self.linux_controller.query()
    }

    pub(super) fn switch(&self, target_mode: InputMethodMode) -> bool {
        let mode = self.query();
        #[cfg(target_os = "windows")]
        if target_mode != mode {
            return self.windows_controller.switch_mode(target_mode)

            #[cfg(target_os = "linux")]
            return self.linux_controller.switch_mode(mode);
        };
        true
    }
}
