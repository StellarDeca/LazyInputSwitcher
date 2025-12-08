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

    #[cfg(target_os = "macos")]
    macos_controller: macos::MacOSController,
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

        #[cfg(target_os = "macos")]
        match macos::MacOSController::new() {
            Ok(macos_controller) => Ok(Switcher { macos_controller }),
            Err(err) => Err(err),
        }
    }

    pub(super) fn query(&self) -> Result<InputMethodMode, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        return Ok::<InputMethodMode, Box<dyn Error>>(self.windows_controller.get_mode());

        #[cfg(target_os = "linux")]
        return self.linux_controller.query();

        #[cfg(target_os = "macos")]
        return self.macos_controller.query();
    }

    pub(super) fn switch(&self, target_mode: InputMethodMode) -> Result<bool, Box<dyn Error>> {
        let mode = self.query()?;
        if target_mode != mode {
            #[cfg(target_os = "windows")]
            return Ok::<bool, Box<dyn Error>>(self.windows_controller.switch_mode(target_mode));

            #[cfg(target_os = "linux")]
            return self.linux_controller.switch(mode);

            #[cfg(target_os = "macos")]
            return self.macos_controller.switch(mode);
        };
        Ok(true)
    }
}
