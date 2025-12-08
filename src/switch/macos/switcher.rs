//! MACOS 输入法 api绑定
//!
use super::tis::*;
use core_foundation::array::CFArray;
use core_foundation::base::TCFType;
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation_sys::base::CFRelease;
use std::error::Error;
use std::ptr;

/// 获取当前活动输入法
pub(super) fn get_mode() -> Result<String, Box<dyn Error>> {
    let current = unsafe { TISCopyCurrentKeyboardInputSource() };
    if current.is_null() {
        return Err("TISCopyCurrentKeyboardInputSource returned null".into());
    }

    let prop = unsafe { TISGetInputSourceProperty(current, kTISPropertyInputSourceID) };
    if prop.is_null() {
        unsafe { CFRelease(current as CFTypeRef) }; // 手动释放输入法引用
        return Err("TISGetInputSourceProperty returned null".into());
    }

    let cf_id = unsafe { CFString::wrap_under_get_rule(prop as CFStringRef) };
    let id = cf_id.to_string();
    unsafe { CFRelease(current as CFTypeRef) };
    Ok(id)
}

/// 切换到指定的输入法
pub(super) fn switch_mode(target_id: String) -> Result<bool, Box<dyn Error>> {
    // 创建 CFDictionary 搜索过滤器
    let key = unsafe { CFString::wrap_under_get_rule(kTISPropertyInputSourceID) };
    let value = CFString::new(target_id.as_str());
    let filter = CFDictionary::from_CFType_pairs(&[(key.as_CFType(), value.clone().as_CFType())]);

    unsafe {
        let list = TISCreateInputSourceList(filter.as_concrete_TypeRef(), false);
        if list.is_null() {
            return Err("TISCreateInputSourceList returned null".into());
        }

        // array 拥有所有权,自动释放内存
        let array: CFArray<TISInputSourceRef> = CFArray::wrap_under_create_rule(list);
        if array.is_empty() {
            return Err(format!("Input method '{}' not found", target_id).into());
        }

        let item = array.get(0).unwrap();
        let result = TISSelectInputSource(*item);
        Ok(result == 0)
    }
}

/// 获取所有可用输入法列表
pub(super) fn get_method_list() -> Result<Vec<String>, Box<dyn Error>> {
    let list = unsafe {
        // 获取用户可用的所有输入法列表
        TISCreateInputSourceList(ptr::null_mut() as CFDictionaryRef, false)
    };
    if list.is_null() {
        return Err("MacOS input method is null".into());
    };
    let array: CFArray<TISInputSourceRef> = unsafe { CFArray::wrap_under_create_rule(list) };
    let mut result = Vec::new();
    unsafe {
        for i in 0..array.len() {
            let item = array.get(i).unwrap();
            let prop = TISGetInputSourceProperty(*item, kTISPropertyInputSourceID);
            if !prop.is_null() {
                let id = CFString::wrap_under_get_rule(prop as CFStringRef);
                result.push(id.to_string());
            };
        }
    }
    Ok(result)
}
