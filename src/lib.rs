#![allow(unused)]
mod definitions;
use ash::vk::{self, TaggedStructure};
pub use definitions::*;
pub mod instance;
pub mod device;
pub mod queue;
pub mod swapchain;
pub mod memory;
pub mod buffers;
pub mod commands;
pub mod sync;
pub mod pipeline;
pub mod image;
pub mod descriptors;
pub mod barriers;
pub mod entry;
pub mod error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u32,   
    pub minor: u32,   
    pub patch: u32,   
}
impl From<u32> for Version {
    #[inline]
    fn from(val: u32) -> Self {
        Version {
            major: ash::vk::api_version_major(val),
            minor: ash::vk::api_version_minor(val),
            patch: ash::vk::api_version_patch(val),
        }
    }
}
impl TryFrom<Version> for u32 {
    type Error = ();

    #[inline]
    fn try_from(val: Version) -> Result<Self, Self::Error> {
        if val.major <= 0x3ff && val.minor <= 0x3ff && val.patch <= 0xfff {
            Ok(ash::vk::make_api_version(
                0, val.major, val.minor, val.patch,
            ))
        } else {
            Err(())
        }
    }
}
impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
    #[inline]
    pub fn supports_version_1_0(&self) -> bool { self.major == 1 && self.minor == 0 }
    #[inline]
    pub fn supports_version_1_1(&self) -> bool { self.major == 1 && self.minor >= 1 }
    #[inline]
    pub fn supports_version_1_2(&self) -> bool { self.major == 1 && self.minor >= 2 }
    #[inline]
    pub fn supports_version_1_3(&self) -> bool { self.major == 1 && self.minor >= 3 }
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone)]
struct VulkanHeader {
    pub s_type: vk::StructureType,
    pub p_next: *const std::os::raw::c_void,
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub struct PNext {
    head: *const std::os::raw::c_void,
    next: *mut *const std::os::raw::c_void,
}
impl PNext {
    pub const fn new() -> Self {
        Self {
            head: std::ptr::null(),
            next: std::ptr::null_mut(),
        }
    }
    pub fn push_front(&mut self, structure: &impl TaggedStructure) {
        let p_next = &mut unsafe { *(structure as *const _ as *const VulkanHeader) }.p_next;
        *p_next = self.head as _;
        self.head = structure as *const _ as _;
    }
    pub fn push_back(&mut self, structure: &impl TaggedStructure) {
        let p_next = &mut unsafe { *(structure as *const _ as *const VulkanHeader) }.p_next;
        if self.head.is_null() {
            self.head = structure as *const _ as _;
            self.next = p_next;
        } else {
            unsafe { *self.next = structure as *const _ as _ };
            self.next = p_next;
        }
    }
    pub fn use_p_next(&self) -> *mut std::os::raw::c_void {
        if self.next.is_null() {
            
        } else {
            unsafe { *self.next = std::ptr::null_mut() };
        }
        self.head as _
    }
}
impl Default for PNext {
    fn default() -> Self {
        Self { head: std::ptr::null(), next: std::ptr::null_mut() }
    }
}

#[cfg(test)]
mod tests {
}
