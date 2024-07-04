use std::{ops::Deref, sync::Arc};

use ash::vk;

use crate::{commands::CommandPoolAllocation, device::LogicalDevice, image::{ImageLayout, ImageSubresourceRange, PipelineStageFlags, RawImage}, memory::{AccessFlags, DependencyFlags}, AsNfptr};

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier.html>"]
pub struct MemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}
impl From<&MemoryBarrier> for vk::MemoryBarrier {
    fn from(value: &MemoryBarrier) -> Self {
        Self { src_access_mask: vk::AccessFlags::from_raw(value.src_access_mask.0), dst_access_mask: vk::AccessFlags::from_raw(value.dst_access_mask.0), ..Default::default() }
    }
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier.html>"]
pub struct ImageMemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: vk::Image,
    pub subresource_range: ImageSubresourceRange,
}
impl From<&ImageMemoryBarrier> for vk::ImageMemoryBarrier {
    fn from(value: &ImageMemoryBarrier) -> Self {
        Self { 
            src_access_mask: vk::AccessFlags::from_raw(value.src_access_mask.0), 
            dst_access_mask: vk::AccessFlags::from_raw(value.dst_access_mask.0), 
            old_layout: vk::ImageLayout::from_raw(value.old_layout.0), 
            new_layout: vk::ImageLayout::from_raw(value.new_layout.0), 
            src_queue_family_index: value.src_queue_family_index, 
            dst_queue_family_index: value.dst_queue_family_index, 
            image: value.image, 
            subresource_range: vk::ImageSubresourceRange::from(value.subresource_range),
            ..Default::default()
        }
    }
}
    #[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier.html>"]
pub struct BufferMemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: vk::Buffer,
    pub offset: u64,
    pub size: u64,
}
impl BufferMemoryBarrier {
    pub fn wait(&self, device: Arc<LogicalDevice>, command_buffer: &CommandPoolAllocation, src: PipelineStageFlags, dst: PipelineStageFlags) {
        command_buffer.pipeline_barrier(src.into(), dst.into(), DependencyFlags::empty(), &[], &[*self], &[]);
    }
}
impl From<&BufferMemoryBarrier> for vk::BufferMemoryBarrier {
    fn from(value: &BufferMemoryBarrier) -> Self {
        Self {
            src_access_mask: vk::AccessFlags::from_raw(value.src_access_mask.0),
            dst_access_mask: vk::AccessFlags::from_raw(value.dst_access_mask.0),
            src_queue_family_index: value.src_queue_family_index,
            dst_queue_family_index: value.dst_queue_family_index,
            buffer: value.buffer,
            offset: value.offset,
            size: value.size,
            ..Default::default()
        }
    }
}
pub struct Barriers {
    pub src: PipelineStageFlags, 
    pub dst: PipelineStageFlags,
    pub memory: Vec<MemoryBarrier>,
    pub image: Vec<ImageMemoryBarrier>,
    pub buffer: Vec<BufferMemoryBarrier>,
}

impl Barriers {
    pub fn new(src: PipelineStageFlags, dst: PipelineStageFlags, memory: Vec<MemoryBarrier>, image: Vec<ImageMemoryBarrier>, buffer: Vec<BufferMemoryBarrier>) -> Self {
        Self { src, dst, memory, image, buffer }
    }
    pub fn merge(&mut self, other: &mut Self) {
        self.memory.append(&mut other.memory);
        self.image.append(&mut other.image);
        self.buffer.append(&mut other.buffer);
        self.src |= other.src;
        self.dst |= other.dst;
    }
    pub fn wait(&self, device: Arc<LogicalDevice>, command_buffer: &CommandPoolAllocation) {
        command_buffer.pipeline_barrier(self.src.into(), self.dst.into(), DependencyFlags::empty(), &self.memory, &self.buffer, &self.image);
    }
}