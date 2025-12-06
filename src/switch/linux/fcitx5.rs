//! Fcitx5 输入法框架支持

use std::error::Error;
use configparser::ini::Ini;
use crate::core::InputMethodMode;
use super::lib::{StaticLinuxMethodShell as LShell};

const DEFAULT_METHOD: &str = "keyboard-us";
const ENGLISH_METHOD: [&str; 1] = ["keyboard-us"];
const NATIVE_METHOD: [&str; 2] = ["rime", "pinyin"];

pub(super) struct Fcitx5Method {
    pub(super) english: String,
    pub(super) native: String,
}
impl Fcitx5Method {
    pub(super) fn new() -> Result<Fcitx5Method, Box<dyn Error>> {
        // 初始化数据结构体并判断 Fcitx5 环境
        let methods = Fcitx5Method::_get_method_list()?;
        let (native, english) = Fcitx5Method::_check_supported_method(methods);
        if native == DEFAULT_METHOD && english == DEFAULT_METHOD {
            return Err("Linux Fcitx5 Input Method Config is not available to control!".into())
        };
        Ok(Fcitx5Method { native, english })
    }

    pub(super) fn get_mode(&self) -> Result<InputMethodMode, Box<dyn Error>> {
        let mode = LShell::run_script("fcitx5/query", None)?;
        if mode == self.native {
            Ok(InputMethodMode::Native)
        } else if mode == self.english{
            Ok(InputMethodMode::English)
        } else {
            Err(format!("Unknown mode {mode}").into())
        }
    }

    pub(super) fn switch_mode(&self, target_mode: InputMethodMode) -> Result<bool, Box<dyn Error>> {
        match target_mode {
            InputMethodMode::Native => {
                LShell::run_script("fcitx5/switch", Some(&[self.english.as_str()]))?;
            },
            InputMethodMode::English => {
                LShell::run_script("fcitx5/switch", Some(&[self.native.as_str()]))?;
            }
        };
        Ok(target_mode == self.get_mode()?)
    }

    fn _get_method_list() -> Result<Vec<String>, Box<dyn Error>> {
        // 解析 配置文件返回获取安装的输入法列表
        let profile = LShell::run_script("fcitx5/config", None)?;
        let mut cfg_parser = Ini::new_cs();
        let cfg = cfg_parser.read(profile)?;
        let mut methods = Vec::new();
        for (section, properties) in cfg.iter() {
            if section.starts_with("Groups/") && section.contains("/Items/") {
                // 检查段落中是否有 Name 字段
                if let Some(name_value) = properties.get("Name") {
                    let name = name_value.clone().unwrap_or(DEFAULT_METHOD.to_string());
                    methods.push(name);
                }
            }
        };
        Ok(methods)
    }

    fn _check_supported_method(methods: Vec<String>) -> (String, String) {
        // 按照 从左到右 匹配，匹配成功则返回
        let native = NATIVE_METHOD.iter().find(|&id| methods.contains(&id.to_string()));
        let english = ENGLISH_METHOD.iter().find(|&id| methods.contains(&id.to_string()));
        (native.unwrap_or(&DEFAULT_METHOD).to_string(), english.unwrap_or(&DEFAULT_METHOD).to_string())
    }
}
