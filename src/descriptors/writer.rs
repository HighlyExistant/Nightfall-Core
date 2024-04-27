use std::sync::Arc;

use ash::vk;

use crate::device::LogicalDevice;

use super::DescriptorBufferInfo;


#[derive(Clone)]
pub struct DescriptorWriter {
    writers: Vec<vk::WriteDescriptorSet>,
}

impl DescriptorWriter {
    pub fn new() -> Self {
        Self { writers: vec![] }
    }
    pub fn reset(&mut self) {
        self.writers.clear();
    }
    pub fn add_storage_buffer(mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &DescriptorBufferInfo) -> Self {
        let info: &vk::DescriptorBufferInfo = info.into();
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
        self
    }
    pub fn add_storage_buffer_dynamic(mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &DescriptorBufferInfo) -> Self {
        let info: &vk::DescriptorBufferInfo = info.into();
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
        self
    }
    pub fn add_image_buffer(mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &vk::DescriptorImageInfo) -> Self {
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            dst_binding: binding,
            dst_array_element: array_element,
            p_image_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
        self
    }
    pub fn add_storage_image(mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &vk::DescriptorImageInfo) -> Self {
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::STORAGE_IMAGE,
            dst_binding: binding,
            dst_array_element: array_element,
            p_image_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
        self
    }
    pub fn add_combined_image_sampler(mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &[vk::DescriptorImageInfo]) -> Self {
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            dst_binding: binding,
            dst_array_element: array_element,
            p_image_info: info.as_ptr(),
            ..Default::default()
        };
        self.writers.push(writer);
        self
    }
    pub fn add_uniform_buffer(mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &DescriptorBufferInfo) -> Self {
        let info: &vk::DescriptorBufferInfo = info.into();
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
        self
    }
    pub fn add_uniform_buffer_dynamic(mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &DescriptorBufferInfo) -> Self {
        let info: &vk::DescriptorBufferInfo = info.into();
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
        self
    }
    pub fn push_storage_buffer(&mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &DescriptorBufferInfo) {
        let info: &vk::DescriptorBufferInfo = info.into();
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
    }
    pub fn push_storage_buffer_dynamic(&mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &DescriptorBufferInfo) {
        let info: &vk::DescriptorBufferInfo = info.into();
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
    }
    pub fn push_image_buffer(&mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &vk::DescriptorImageInfo) {
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            dst_binding: binding,
            dst_array_element: array_element,
            p_image_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
    }
    pub fn push_storage_image(&mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &vk::DescriptorImageInfo) {
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::STORAGE_IMAGE,
            dst_binding: binding,
            dst_array_element: array_element,
            p_image_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
    }
    pub fn push_uniform_buffer(&mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &DescriptorBufferInfo) {
        let info: &vk::DescriptorBufferInfo = info.into();
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
    }
    pub fn push_uniform_buffer_dynamic(&mut self, set: vk::DescriptorSet, count: u32, binding: u32, array_element: u32, info: &vk::DescriptorBufferInfo) {
        let writer = vk::WriteDescriptorSet {
            dst_set: set,
            descriptor_count: count,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            dst_binding: binding,
            dst_array_element: array_element,
            p_buffer_info: info,
            ..Default::default()
        };
        self.writers.push(writer);
    }
    pub fn write(&self, device: Arc<LogicalDevice>) {
        unsafe { device.device.update_descriptor_sets(self.writers.as_slice(), &[]) };
    }
}
