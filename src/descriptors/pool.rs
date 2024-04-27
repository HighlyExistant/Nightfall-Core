use std::sync::Arc;

use ash::{vk, vk_bitflags_wrapped};

use crate::{device::LogicalDevice, error::VulkanError};

use super::{DescriptorLayout, DescriptorSetAllocation, DescriptorType};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html>"]
pub struct DescriptorPoolCreateFlags(pub(crate) u32);
vk_bitflags_wrapped!(DescriptorPoolCreateFlags, u32);
impl DescriptorPoolCreateFlags {
    pub const FREE_DESCRIPTOR_SET: Self = Self(0b1);
    pub const UPDATE_AFTER_BIND: Self = Self(0b10);
    pub const HOST_ONLY_EXT: Self = Self(0b100);
}
pub struct DescriptorPool {
    device: Arc<LogicalDevice>,
    pub pool: vk::DescriptorPool
}
impl DescriptorPool {
    pub fn builder() -> DescriptorPoolBuilder {
        DescriptorPoolBuilder::new()
    }
    pub fn allocate(self: Arc<Self>, layouts: &'_ [Arc<DescriptorLayout>]) -> Result<impl ExactSizeIterator<Item = DescriptorSetAllocation>+'_, VulkanError> {
        let rawlayouts = layouts.iter().map(|val|{ val.layout() }).collect::<Vec<_>>();
        let info = vk::DescriptorSetAllocateInfo {
            p_set_layouts: rawlayouts.as_ptr(),
            descriptor_pool: self.pool,
            descriptor_set_count: rawlayouts.len() as u32,
            ..Default::default()
        };
        Ok(unsafe { 
            let this = self.clone();
            self.device.device.allocate_descriptor_sets(&info)
            .map_err(|err|{VulkanError::from(err)})?
            .into_iter()
            .zip(layouts.iter())
            .map(move |(set, layout)|{ DescriptorSetAllocation::new(set, this.clone(), layout.clone()) })
        })
    }
    pub fn deallocate(&self, sets: &[DescriptorSetAllocation]) -> Result<(), VulkanError> {
        let sets = sets.into_iter().map(|set|{ set.set() }).collect::<Vec<_>>();
        unsafe { self.device.device.free_descriptor_sets(self.pool, &sets).map_err(|err|{VulkanError::from(err)}) }
    }
    pub fn deallocate_all(&self) -> Result<(), VulkanError> {
        unsafe { self.device.device.reset_descriptor_pool(self.pool, vk::DescriptorPoolResetFlags::empty()).map_err(|err|{VulkanError::from(err)}) }
    }
}
pub struct DescriptorPoolBuilder {
    pools: Vec<vk::DescriptorPoolSize>,
    flag: vk::DescriptorPoolCreateFlags,
    max_sets: u32
}
impl DescriptorPoolBuilder {
    pub fn new() -> Self {
        Self { pools: vec![], flag: vk::DescriptorPoolCreateFlags::empty(), max_sets: 0 }
    }
    pub fn add_pool_size(mut self, ty: DescriptorType, descriptor_count: u32) -> Self {
        let pool = vk::DescriptorPoolSize {
            descriptor_count: descriptor_count,
            ty: vk::DescriptorType::from_raw(ty.0),
        };
        self.pools.push(pool);
        self
    }
    pub fn set_flag(mut self, flag: DescriptorPoolCreateFlags) -> Self {
        self.flag = vk::DescriptorPoolCreateFlags::from_raw(flag.0);
        self
    }
    pub fn set_max_sets(mut self, max: u32) -> Self {
        self.max_sets = max;
        self
    }
    pub fn add_max_sets(mut self, sets: u32) -> Self {
        self.max_sets += sets;
        self
    }
    pub fn build(self, device: Arc<LogicalDevice>) -> Arc<DescriptorPool> {
        let create_info = vk::DescriptorPoolCreateInfo {
           pool_size_count: self.pools.len() as u32,
           p_pool_sizes: self.pools.as_ptr(),
           flags: self.flag,
           max_sets: self.max_sets,
            ..Default::default()
        };
        let pool = unsafe { device.device.create_descriptor_pool(&create_info, None).unwrap() };
        Arc::new(DescriptorPool { pool, device: device.clone() })
    }
}
impl Drop for DescriptorPool {
    fn drop(&mut self) {
        unsafe {
            if cfg!(debug_assertions) {
                println!("Descriptor Pool Dropped");
            }
            self.device.device.destroy_descriptor_pool(self.pool, None);
        }
    }
}