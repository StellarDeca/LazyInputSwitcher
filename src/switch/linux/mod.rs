//! 适配 Linux 系统主流输入法框架 Fcitx 与 Ibus
//!

mod lib;
mod fcitx5;

use lib::*;
use fcitx5::*;
use std::error::Error;
use crate::core::InputMethodMode;

pub(super) struct LinuxController {
    method: Box<dyn MethodController>,
}
impl LinuxController {
    pub(super) fn new() -> Result<LinuxController, Box<dyn Error>> {
        let method =  SupportMethod::check_input_method();
        if let Some(method) = method {
            let instance = match method {
                SupportMethod::Fcitx5 => { Fcitx5Method::new()? },
            };
            Ok(LinuxController { method: Box::new(instance) })
        } else {
            Err("Linux Method Config is not available to control!".into())
        }
    }

    pub(super) fn query(&self) -> Result<InputMethodMode, Box<dyn Error>> {
        self.method.get_mode()
    }

    pub(super) fn switch(&self, target_mode: InputMethodMode) -> Result<bool, Box<dyn Error>> {
        self.method.switch_mode(target_mode)
    }
}

trait MethodController {
    fn switch_mode(&self, target_mode: InputMethodMode) -> Result<bool, Box<dyn Error>>;

    fn get_mode(&self) -> Result<InputMethodMode, Box<dyn Error>>;
}
