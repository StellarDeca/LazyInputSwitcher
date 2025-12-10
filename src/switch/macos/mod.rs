//! MacOS 输入法绑定实现

mod tis;
mod switcher;

use crate::core::InputMethodMode;
use std::error::Error;
use switcher::*;

const DEFAULT_LANGUAGE_ID: &str = "com.apple.keylayout.ABC";
const ENGLISH_LANGUAGE_ID: [&str; 1] = ["com.apple.keylayout.ABC"];
const NATIVE_LANGUAGE_ID: [&str; 1] = ["com.apple.inputmethod.SCIM.ITABC"];

pub(super) struct MacOSController {
    english: String,
    native: String,
}
impl MacOSController {
    pub(super) fn new() -> Result<MacOSController, Box<dyn Error>> {
        let languages = get_method_list()?;
        let (native, english) = MacOSController::_check_supported_method(languages);
        if native == DEFAULT_LANGUAGE_ID && english == DEFAULT_LANGUAGE_ID {
            return Err("MacOS input method is not available to control!".into())
        };
        Ok(MacOSController { native, english })
    }

    pub(super) fn query(&self) -> Result<InputMethodMode, Box<dyn Error>> {
        let mode = get_mode()?;
        if mode == self.native {
            Ok(InputMethodMode::Native)
        } else if mode == self.english {
            Ok(InputMethodMode::English)
        } else {
            Err("Unknown input method".into())
        }
    }

    pub(super) fn switch(&self, target_mode: InputMethodMode) -> Result<bool, Box<dyn Error>> {
        match target_mode {
            InputMethodMode::Native => switch_mode(self.native.as_str()),
            InputMethodMode::English => switch_mode(self.english.as_str()),
        }
    }

    fn _check_supported_method(methods: Vec<String>) -> (String, String) {
        // 按照 从左到右 匹配，匹配成功则返回
        let native = NATIVE_LANGUAGE_ID.iter().find(|&id| methods.contains(&id.to_string()));
        let english = ENGLISH_LANGUAGE_ID.iter().find(|&id| methods.contains(&id.to_string()));
        (native.unwrap_or(&DEFAULT_LANGUAGE_ID).to_string(), english.unwrap_or(&DEFAULT_LANGUAGE_ID).to_string())
    }
}

