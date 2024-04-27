mod layout;
mod writer;
mod pool;
mod set;
use ash::vk;
pub use layout::*;
pub use writer::*;
pub use pool::*;
pub use set::*;

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferInfo.html>"]
pub struct DescriptorBufferInfo {
    pub buffer: ash::vk::Buffer,
    pub offset: u64,
    pub range: u64,
}

impl From<DescriptorBufferInfo> for ash::vk::DescriptorBufferInfo {
    fn from(value: DescriptorBufferInfo) -> Self {
        Self { buffer: value.buffer, offset: value.offset, range: value.range }
    }
}
impl From<&DescriptorBufferInfo> for &ash::vk::DescriptorBufferInfo {
    fn from(value: &DescriptorBufferInfo) -> Self {
        unsafe { std::mem::transmute::<&DescriptorBufferInfo, &vk::DescriptorBufferInfo>(value) }
    }
}