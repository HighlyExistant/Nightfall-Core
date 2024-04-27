use std::{cell::Cell, os::raw::c_void, sync::Arc};

use ash::{vk, vk_bitflags_wrapped};

mod map;
mod pointer;
pub use map::*;
pub use pointer::*;

use crate::{buffers::MemoryPropertyFlags, device::LogicalDevice, error::VulkanError};
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html>"]
pub struct MemoryAllocateFlags(pub(crate) u32);
vk_bitflags_wrapped!(MemoryAllocateFlags, u32);
impl MemoryAllocateFlags {
    #[doc = "Force allocation on specific devices"]
    pub const DEVICE_MASK: Self = Self(0b1);
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html>"]
pub struct AccessFlags(pub(crate) u32);
vk_bitflags_wrapped!(AccessFlags, u32);
impl AccessFlags {
    #[doc = "specifies read access to indirect command data read as part of an indirect build, trace, drawing or dispatching command. Such access occurs in the PipelineStage::DRAW_INDIRECT_BIT pipeline stage"]
    pub const INDIRECT_COMMAND_READ: Self = Self(0b1);
    #[doc = "specifies read access to an index buffer as part of an indexed drawing command, bound by vkCmdBindIndexBuffer2KHR and vkCmdBindIndexBuffer. Such access occurs in the PipelineStage::VERTEX_INPUT_BIT pipeline stage"]
    pub const INDEX_READ: Self = Self(0b10);
    #[doc = "specifies read access to a vertex buffer as part of a drawing command, bound by vkCmdBindVertexBuffers. Such access occurs in the PipelineStage::VERTEX_INPUT_BIT pipeline stage"]
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0b100);
    #[doc = "specifies read access to a uniform buffer in any shader pipeline stage"]
    pub const UNIFORM_READ: Self = Self(0b1000);
    #[doc = "specifies read access to an input attachment within a render pass during subpass shading or fragment shading. Such access occurs in the VK_PIPELINE_STAGE_2_SUBPASS_SHADER_BIT_HUAWEI or VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT pipeline stage"]
    pub const INPUT_ATTACHMENT_READ: Self = Self(0b1_0000);
    #[doc = "specifies read access to a uniform texel buffer, sampled image, storage buffer, physical storage buffer, shader binding table, storage texel buffer, or storage image in any shader pipeline stage"]
    pub const SHADER_READ: Self = Self(0b10_0000);
    #[doc = "specifies write access to a storage buffer, physical storage buffer, storage texel buffer, or storage image in any shader pipeline stage"]
    pub const SHADER_WRITE: Self = Self(0b100_0000);
    pub const COLOR_ATTACHMENT_READ: Self = Self(0b1000_0000);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(0b1_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(0b10_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(0b100_0000_0000);
    pub const TRANSFER_READ: Self = Self(0b1000_0000_0000);
    pub const TRANSFER_WRITE: Self = Self(0b1_0000_0000_0000);
    pub const HOST_READ: Self = Self(0b10_0000_0000_0000);
    pub const HOST_WRITE: Self = Self(0b100_0000_0000_0000);
    pub const MEMORY_READ: Self = Self(0b1000_0000_0000_0000);
    pub const MEMORY_WRITE: Self = Self(0b1_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html>"]
pub struct DependencyFlags(pub(crate) u32);
vk_bitflags_wrapped!(DependencyFlags, u32);
impl DependencyFlags {
    #[doc = "Dependency is per pixel region "]
    pub const BY_REGION: Self = Self(0b1);
}
pub struct DeviceMemory {
    pub(crate) handle: vk::DeviceMemory,
    pub(crate) device: Arc<LogicalDevice>,
    pub(crate) size: usize,
    pub(crate) memory_type_index: u32,
    pub(crate) mapping_state: Cell<Option<MappingState>>,
}

impl DeviceMemory {
    pub fn allocate(device: Arc<LogicalDevice>, size: usize, memory_type_index: u32, extension: *const vk::MemoryAllocateFlagsInfo) -> Result<Self, VulkanError> {
        let allocate_info = vk::MemoryAllocateInfo {
            allocation_size: size as u64,
            memory_type_index: memory_type_index,
            p_next: extension as *const _ as _,
            ..Default::default()
        };
        
        let handle = unsafe { device.device.allocate_memory(&allocate_info, None).map_err(VulkanError::from)? };
        Ok(
            Self { handle, device, size, memory_type_index, mapping_state: Cell::new(None) }
        )
    }
    pub fn get_memory_type_index(device: Arc<LogicalDevice>, properties: MemoryPropertyFlags, requirements: vk::MemoryRequirements) -> u32 {
        let memory_properties = device.physical_device.get_physical_device_memory_properties();
        
        let i = (0..memory_properties.memory_type_count).find_map(|i| {
            if requirements.memory_type_bits & (1 << i) == (1 << i) &&
				memory_properties.memory_types[i as usize].property_flags & vk::MemoryPropertyFlags::from_raw(properties.0) == vk::MemoryPropertyFlags::from_raw(properties.0) {
				Some(i)
			} else {
                None
            }
        }).unwrap();
        i
    }
    pub fn bind_memory(&self, buffer: vk::Buffer, offset: usize) -> Result<(), VulkanError> {
        unsafe { 
            (self.device.fns.v1_0.bind_buffer_memory)(
                    self.device.handle(),
                    buffer,
                    self.handle,
                    offset as u64
            ).result().map_err(VulkanError::from)
        }
    }
    pub fn bind_image(&self, image: vk::Image, offset: usize) {
        unsafe { self.device.device.bind_image_memory(image, self.handle, offset as u64).unwrap() };
    }
    #[inline]
    pub fn memory(&self) -> vk::DeviceMemory {
        self.handle
    }
}

impl Drop for DeviceMemory {
    fn drop(&mut self) {
        unsafe { self.device.device.free_memory(self.memory(), None) };
    }
}