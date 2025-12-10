//! MacOS 输入法 C api 声明

use core_foundation::array::CFArrayRef;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFStringRef;

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    /// API 常量
    pub(super) static kTISPropertyInputSourceID: CFStringRef;

    /// 获取当前激活输入法的引用
    /// 不用时需要手动释放 CFRelease
    pub(super) fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;

    /// 根据提供的属性创建一个满足条件的所有输入源列表
    /// 不用时需要手动释放 CFRelease
    pub(super) fn TISCreateInputSourceList(properties: CFDictionaryRef, include_all_installed: bool) -> CFArrayRef;

    /// 切换到指定的输入源
    /// 返回 0 成功；非 0 失败
    pub(super) fn TISSelectInputSource(source: TISInputSourceRef) -> i32;

    /// 获取指定输入源的某个属性值
    pub(super) fn TISGetInputSourceProperty(source: TISInputSourceRef, property_key: CFStringRef) -> CFTypeRef;
}

/// 不可变输入对象
pub(super) type TISInputSourceRef = *const libc::c_void;

/// 所有 CoreFoundation 对象都以 CFTypeRef 为父类
pub(super) type CFTypeRef = *const libc::c_void;
