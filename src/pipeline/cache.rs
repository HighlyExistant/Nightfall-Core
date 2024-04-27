use std::{mem::MaybeUninit, os::raw::c_void, sync::Arc};

use ash::{vk::{self, PipelineCacheCreateInfo}, vk_bitflags_wrapped};

use crate::{device::LogicalDevice, error::VulkanError};
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html>"]
pub struct PipelineCacheCreateFlags(pub(crate) u32);
vk_bitflags_wrapped!(PipelineCacheCreateFlags, u32);
impl PipelineCacheCreateFlags {
    const EXTERNALLY_SYNCHRONIZED: u32 = 1;
}
 
#[repr(C)]
pub struct VkPipelineCacheHeaderOne {
    length: u32, // == mem::size_of::<VkPipelineCacheHeaderOne>()
    version: u32, // == vk::PipelineCacheHeaderVersion::ONE
    vendor_id: u32,
    device_id: u32,
    uuid: [u8; vk::UUID_SIZE],
}
/// less safer version of pipeline cache with no application level checks. Used for when you don't want 
/// to use the strategies used in [this article](https://zeux.io/2019/07/17/serializing-pipeline-cache/)
pub struct PipelineCacheRaw {
    device: Arc<LogicalDevice>,
    cache: vk::PipelineCache
}

impl PipelineCacheRaw {
    pub fn new(device: Arc<LogicalDevice>, initial_data: &[u8], flags: PipelineCacheCreateFlags) -> Result<Self, VulkanError> {
        let info = if initial_data.is_empty() {
            PipelineCacheCreateInfo {
                ..Default::default()
            }
        } else {
            PipelineCacheCreateInfo {
                initial_data_size: initial_data.len(),
                p_initial_data: initial_data.as_ptr() as *const _,
                flags: vk::PipelineCacheCreateFlags::from_raw(flags.0),
                ..Default::default()
            }
        };
        let cache = unsafe {
            device.device.create_pipeline_cache(&info, None )
        }.map_err(VulkanError::from)?;
        Ok(Self { device: device.clone(), cache })
    }
    pub fn empty(device: Arc<LogicalDevice>) -> Result<Self, VulkanError> {
        let cache = unsafe {
            device.device.create_pipeline_cache(&PipelineCacheCreateInfo::default(), None )
        }.map_err(VulkanError::from)?;
        Ok(Self { device: device.clone(), cache })
    }
    pub fn get(&self) -> Result<Vec<u8>, VulkanError> {
        unsafe { self.device.device.get_pipeline_cache_data(self.cache).map_err(VulkanError::from) }
    }
    pub fn merge(&self, src: &[Self]) -> Result<(), VulkanError> {
        let caches = src.iter().map(|cache|{cache.cache}).collect::<Vec<_>>();
        unsafe { self.device.device.merge_pipeline_caches(self.cache, &caches).map_err(VulkanError::from) }
    }
}
impl Drop for PipelineCacheRaw {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_pipeline_cache(self.cache, None) };
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PipelineCachePrefixHeader {
    magic: u32,     // i've chosen arbitrarily the magic number: 0x7665726F
    data_size: u32,
    data_hash: u64,
    vendor_id: u32,
    device_id: u32,
    driver_version: u32,
    driver_abi: u32,
    uuid: [u8; vk::UUID_SIZE],
}
/// safer version of pipeline cache that uses the strategies of [this article](https://zeux.io/2019/07/17/serializing-pipeline-cache/)
pub struct PipelineCache {
    inner: PipelineCacheRaw
}

impl PipelineCache {
    pub fn new(device: Arc<LogicalDevice>, initial_data: &[u8], flags: PipelineCacheCreateFlags) -> Result<Self, VulkanError> {
        let info = if initial_data.is_empty() {
            PipelineCacheCreateInfo {
                ..Default::default()
            }
        } else {
            unsafe { Self::validate_cache_data(device.clone(), initial_data) }.ok_or(VulkanError::InvalidPipelineCachePrefixHeader)?;
            PipelineCacheCreateInfo {
                initial_data_size: initial_data.len()-std::mem::size_of::<PipelineCachePrefixHeader>(),
                p_initial_data: unsafe { initial_data.as_ptr().add(std::mem::size_of::<PipelineCachePrefixHeader>()) } as *const _,
                flags: vk::PipelineCacheCreateFlags::from_raw(flags.0),
                ..Default::default()
            }
        };
        let cache = unsafe {
            device.device.create_pipeline_cache(&info, None )
        }.map_err(VulkanError::from)?;
        Ok(Self { inner: PipelineCacheRaw { device: device.clone(), cache } })
    }
    // should only be called when initial_data != 0
    unsafe fn validate_cache_data(device: Arc<LogicalDevice>, initial_data: &[u8]) -> Option<bool> {
        let prefix = *(initial_data.as_ptr() as *const PipelineCachePrefixHeader);
        // if it doesn't equal magic number it is invalid and should be ignored
        if prefix.magic != 0x7665726F {
            return None;
        }
        let properties = device.physical_device.properties();
        Some(
            prefix.device_id == properties.device_id && 
            prefix.driver_version == properties.driver_version && 
            prefix.driver_abi == std::mem::size_of::<usize>() as u32 && 
            prefix.vendor_id == properties.vendor_id &&
            prefix.uuid == properties.pipeline_cache_uuid,
        )
    }
    pub fn make_prefix_header(&self) -> PipelineCachePrefixHeader {
        let data = self.inner.get().unwrap();
        let properties = self.inner.device.physical_device.properties();
        PipelineCachePrefixHeader { 
            magic: 0x7665726F, 
            data_size: data.len() as u32, 
            data_hash: 0x12345678, 
            vendor_id: properties.vendor_id, 
            device_id: properties.device_id, 
            driver_version: properties.driver_version, 
            driver_abi: std::mem::size_of::<usize>() as u32, 
            uuid: properties.pipeline_cache_uuid, 
        }
    }
    pub fn get(&self) -> Result<Vec<u8>, VulkanError> {
        let mut size = 0;
        let cache = vk::PipelineCache::null();
        unsafe {
            (self.inner.device.fns.v1_0.get_pipeline_cache_data)(
                self.inner.device.handle(),
                cache,
                &mut size,
                std::ptr::null_mut(),
            ).result().map_err(VulkanError::from)?;
            let mut data = Vec::<u8>::with_capacity(size + std::mem::size_of::<PipelineCachePrefixHeader>());
            // copy the prefix header to the pipeline cache data
            let prefix = self.make_prefix_header();
            std::ptr::copy_nonoverlapping(&prefix, data.as_mut_ptr() as *mut PipelineCachePrefixHeader, std::mem::size_of::<PipelineCachePrefixHeader>());
            // copy the actual pipeline cache data
            (self.inner.device.fns.v1_0.get_pipeline_cache_data)(
                self.inner.device.handle(),
                cache,
                &mut size,
                data.as_mut_ptr().add(std::mem::size_of::<PipelineCachePrefixHeader>()) as *mut c_void,
            ).result().map_err(VulkanError::from)?;
            data.set_len(size + std::mem::size_of::<PipelineCachePrefixHeader>());
            Ok(data)
        }
    }
}