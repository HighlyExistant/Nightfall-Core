use std::sync::Arc;

use ash::{vk, prelude::VkResult};

use crate::device::LogicalDevice;

pub struct Sampler {
    pub(crate) sampler: vk::Sampler,
    pub(crate) device: Arc<LogicalDevice>,
}
pub struct SamplerBuilder {
    create_info: vk::SamplerCreateInfo
}

impl SamplerBuilder {
    pub fn new() -> Self {
        Self { create_info: vk::SamplerCreateInfo::default() }
    }
    pub fn set_all_address_modes(mut self, address_mode: vk::SamplerAddressMode) -> Self {
        self.create_info.address_mode_u = address_mode;
        self.create_info.address_mode_v = address_mode;
        self.create_info.address_mode_w = address_mode;
        self
    }
    pub fn set_address_mode_u(mut self, address_mode: vk::SamplerAddressMode) -> Self {
        self.create_info.address_mode_u = address_mode;
        self
    }
    pub fn set_address_mode_v(mut self, address_mode: vk::SamplerAddressMode) -> Self {
        self.create_info.address_mode_v = address_mode;
        self
    }
    pub fn set_address_mode_w(mut self, address_mode: vk::SamplerAddressMode) -> Self {
        self.create_info.address_mode_w = address_mode;
        self
    }
    pub fn set_all_filters(mut self, filter: vk::Filter) -> Self {
        self.create_info.mag_filter = filter;
        self.create_info.min_filter = filter;
        self
    }
    pub fn set_mag_filter(mut self, filter: vk::Filter) -> Self {
        self.create_info.mag_filter = filter;
        self
    }
    pub fn set_min_filters(mut self, filter: vk::Filter) -> Self {
        self.create_info.min_filter = filter;
        self
    }
    pub fn border_colors(mut self, color: vk::BorderColor) -> Self {
        self.create_info.border_color = color;
        self
    }
    pub fn enable_anistropy(mut self, enable: bool) -> Self {
        self.create_info.anisotropy_enable = enable as u32;
        self
    }
    pub fn enable_comparison(mut self, enable: bool) -> Self {
        self.create_info.compare_enable = enable as u32;
        self
    }
    pub fn enable_unnormalized_coordinates(mut self, enable: bool) -> Self {
        self.create_info.unnormalized_coordinates = enable as u32;
        self
    }
    pub fn compare_op(mut self, op: vk::CompareOp) -> Self {
        self.create_info.compare_op = op;
        self
    }
    pub fn set_max_lod(mut self, lod: f32) -> Self {
        self.create_info.max_lod = lod;
        self
    }
    pub fn set_min_lod(mut self, lod: f32) -> Self {
        self.create_info.min_lod = lod;
        self
    }
    pub fn set_mip_lod_bias(mut self, lod: f32) -> Self {
        self.create_info.mip_lod_bias = lod;
        self
    }
    pub fn set_mipmap_mode(mut self, mode: vk::SamplerMipmapMode) -> Self {
        self.create_info.mipmap_mode = mode;
        self
    }
    pub fn build(mut self, device: Arc<LogicalDevice>) -> VkResult<Arc<Sampler>> {
        if self.create_info.anisotropy_enable == 1 {
            let limits = device.physical_device.properties().limits;
            self.create_info.max_anisotropy = limits.max_sampler_anisotropy;
        }
        Sampler::new(device, self.create_info)
    }
}

impl Sampler {
    pub fn new(device: Arc<LogicalDevice>, create_info: vk::SamplerCreateInfo) -> VkResult<Arc<Self>> {
        let sampler = unsafe { device.device.create_sampler(&create_info, None)? };
        Ok(Arc::new(Self { sampler, device }))
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_sampler(self.sampler, None) };
    }
}