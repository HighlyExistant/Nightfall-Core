#![allow(unused)]
use std::{mem::MaybeUninit, sync::Arc, time::Duration};

use ash::{prelude::VkResult, vk};

use crate::error::VulkanError;

use super::device::LogicalDevice;
#[derive(Clone)]
pub struct Fence {
    fence: vk::Fence,
    device: Arc<LogicalDevice>
}

impl Fence {
    pub fn new(device: Arc<LogicalDevice>, signaled: bool) -> Self {
        let create_info = vk::FenceCreateInfo {
            flags: if signaled { vk::FenceCreateFlags::SIGNALED } else { vk::FenceCreateFlags::empty() },
            ..Default::default()
        };
        let fence = unsafe { device.device.create_fence(&create_info, None).unwrap() };
        Self { fence, device }
    }
    #[inline]
    pub fn reset(&self) { unsafe { self.device.device.reset_fences(&[self.fence]).unwrap() } }
    #[inline]
    pub fn wait(&self, duration: std::time::Duration) -> Result<(), VulkanError> { unsafe { self.device.device.wait_for_fences(&[self.fence], true, duration.as_nanos() as u64).map_err(VulkanError::from) } }
    #[inline]
    pub fn wait_max(&self) -> Result<(), VulkanError> {
        self.wait(Duration::from_nanos(u64::MAX))
    }
    pub fn status(&self, idx: usize) -> VkResult<bool> { unsafe { self.device.device.get_fence_status(self.fence) } }
    pub fn get(&self) -> vk::Fence { self.fence }
}

pub struct Semaphore {
    pub(crate) semaphore: vk::Semaphore,
    pub(crate) device: Arc<LogicalDevice>
}

impl Semaphore {
    pub fn new(device: Arc<LogicalDevice>) -> Self {
        let create_info = vk::SemaphoreCreateInfo {
            ..Default::default()
        };
        
        let semaphore = unsafe { device.device.create_semaphore(&create_info, None).unwrap() };
        Self { semaphore, device }
    }
    pub fn signal(&self, value: u64) {
        let signal_info = vk::SemaphoreSignalInfo {
            semaphore: self.semaphore,
            value,
            ..Default::default()
        };
        unsafe { self.device.device.signal_semaphore(&signal_info).unwrap() }
    }
    pub fn counter(&self, idx: usize) -> VkResult<u64> {
        unsafe { self.device.device.get_semaphore_counter_value(self.semaphore) }
    }
    pub fn get(&self) -> vk::Semaphore { self.semaphore }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_fence(self.fence, None) };
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        println!("Destroying Semaphore");
        unsafe { self.device.device.destroy_semaphore(self.semaphore, None) };
    }
}