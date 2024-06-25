use std::{mem::MaybeUninit, sync::Arc};

use ash::{vk, vk_bitflags_wrapped};
mod submit;
use smallvec::SmallVec;
pub use submit::*;

use crate::{commands::{CommandBufferLevel, CommandPool, CommandPoolAllocation}, device::LogicalDevice, error::VulkanError, sync::Fence};
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html>"]
pub struct DeviceQueueCreateFlags(pub(crate) u32);
vk_bitflags_wrapped!(DeviceQueueCreateFlags, u32);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html>"]
pub struct QueueFlags(pub(crate) u32);
vk_bitflags_wrapped!(QueueFlags, u32);
impl QueueFlags {
    #[doc = "Queue supports graphics operations"]
    pub const GRAPHICS: Self = Self(0b1);
    #[doc = "Queue supports compute operations"]
    pub const COMPUTE: Self = Self(0b10);
    #[doc = "Queue supports transfer operations"]
    pub const TRANSFER: Self = Self(0b100);
    #[doc = "Queue supports sparse resource memory management operations"]
    pub const SPARSE_BINDING: Self = Self(0b1000);
}
#[derive(Clone, Copy, Debug)]
pub struct QueueBuilder {
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub idx: u32,
    pub p_queue_priorities: *const f32,
}
impl Default for QueueBuilder {
    fn default() -> Self {
        Self { flags: DeviceQueueCreateFlags::default(), queue_family_index: 0, queue_count: 0, idx: 0, p_queue_priorities: std::ptr::null() }
    }
}

impl QueueBuilder {
    pub fn new() -> Self {
        Self { flags: DeviceQueueCreateFlags::empty(), queue_family_index: 0, queue_count: 0, idx: 0, p_queue_priorities: &0.0 }
    }
    pub fn build(self) {
        
    }
}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html>"]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: [u32; 3],
}
impl From<&ash::vk::QueueFamilyProperties> for QueueFamilyProperties {
    fn from(value: &ash::vk::QueueFamilyProperties) -> Self {
        Self { 
            queue_flags: QueueFlags(value.queue_flags.as_raw()), 
            queue_count: value.queue_count, 
            timestamp_valid_bits: value.timestamp_valid_bits, 
            min_image_transfer_granularity: [value.min_image_transfer_granularity.width, value.min_image_transfer_granularity.height, value.min_image_transfer_granularity.depth] 
        }
    }
}
pub struct Queue {
    handle: ash::vk::Queue,
    device: Arc<LogicalDevice>,
    queue_family_index: u32,
    queue_flags: QueueFlags,
    // index within the queue family
    idx: u32,
}

impl Queue {
    pub(crate) unsafe fn new(
        device: Arc<LogicalDevice>,
        flags: DeviceQueueCreateFlags,
        queue_flags: QueueFlags,
        queue_family_index: u32,
        idx: u32,
    ) -> Arc<Self> {
        let handle = {
            let mut output = MaybeUninit::uninit();
            (device.fns.v1_0.get_device_queue)(
                device.device.handle(),
                queue_family_index,
                idx,
                output.as_mut_ptr(),
            );
            output.assume_init()
        };
        Arc::new(
            Self {
                device,
                handle,
                idx,
                queue_flags, 
                queue_family_index,
            }
        )
    }
    pub fn single_time_commands(&self, pool: Arc<CommandPool>) -> Result<CommandPoolAllocation, VulkanError> {
        let cmd = unsafe { pool.allocate_command_buffers(CommandBufferLevel::PRIMARY, 1)?.next().unwrap() };
        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };
        unsafe { self.device.device.begin_command_buffer(cmd.get_command_buffer(), &begin_info).unwrap() };
        Ok(cmd)
    }
    pub fn end_single_time_commands(&self, pool: Arc<CommandPool>, command_buffer: &CommandPoolAllocation) {
        unsafe { self.device.device.end_command_buffer(command_buffer.get_command_buffer()).unwrap() };
        let info = vk::SubmitInfo {
            command_buffer_count: 1,
            p_command_buffers: &command_buffer.get_command_buffer(),
            ..Default::default()
        };
        unsafe { 
            self.device.device.queue_submit(self.handle, &[info], vk::Fence::null()).unwrap();
            self.device.device.queue_wait_idle(self.handle).unwrap();
            self.device.device.free_command_buffers(pool.command_pool, &[command_buffer.get_command_buffer()]);
        }
    }
    pub fn submit_raw(&self, submits: &[vk::SubmitInfo], fence: &Fence) -> Result<(), VulkanError> {
        unsafe { 
            self.device.device.queue_submit(
                self.handle,
                &submits,
                fence.get()
            ).map_err(VulkanError::from)
        }
    }
    pub fn submit(&self, submit: &[&Submission]) -> Result<SubmissionCache, VulkanError> {
        let cache = Submission::cached(submit);
        self.submit_cached(&cache)?;
        Ok(cache)
    }
    pub fn submit_with_fence(&self, submit: &[&Submission]) -> Result<Arc<Fence>, VulkanError> {
        let cache = Submission::cached(submit);
        let fence = Arc::new(Fence::new(self.device.clone(), false));
        unsafe { 
            self.device.device.queue_submit(
                self.handle,
                &cache.submits,
                fence.get()
            ).map_err(VulkanError::from)?;
        };
        Ok(fence)
    }
    pub fn submit_cached(&self, cache: &SubmissionCache) -> Result<(), VulkanError> {
        unsafe {
            self.device.device.queue_submit(
                self.handle,
                &cache.submits,
                vk::Fence::null()
            ).map_err(VulkanError::from)?;
        };
        Ok(())
    }
    pub fn submit_cached_external_fence(&self, cache: &SubmissionCache, fence: &Fence) -> Result<(), VulkanError> {
        unsafe {
            self.device.device.queue_submit(
                self.handle,
                &cache.submits,
                fence.get()
            ).map_err(VulkanError::from)
        }
    }
    pub fn submit_cached_with_fence(&self, cache: &SubmissionCache) -> Result<Arc<Fence>, VulkanError> {
        let fence = Arc::new(Fence::new(self.device.clone(), false));
        unsafe {
            self.device.device.queue_submit(
                self.handle,
                &cache.submits,
                fence.get()
            ).map_err(VulkanError::from)?;
        };
        Ok(fence)
    }
    pub fn queue_flags(&self) -> QueueFlags {
        self.queue_flags
    }
    #[inline]
    pub fn family_index(&self) -> u32 {
        self.queue_family_index
    }
    #[inline]
    pub fn device(&self) -> Arc<LogicalDevice> {
        self.device.clone()
    }
    #[inline]
    pub fn handle(&self) -> vk::Queue {
        self.handle
    }
    #[inline]
    pub fn idx(&self) -> u32 {
        self.idx
    }
}