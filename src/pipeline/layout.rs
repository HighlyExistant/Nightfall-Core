use std::sync::Arc;

use ash::vk;

use crate::device::LogicalDevice;

use super::shader::ShaderStageFlags;


pub struct PipelineLayout {
    pub(crate) device: Arc<LogicalDevice>,
    pub(crate) layout: vk::PipelineLayout,
}
pub struct PipelineLayoutBuilder {
    push_constant_ranges: Vec<vk::PushConstantRange>,
    descriptor_set_layouts: Vec<vk::DescriptorSetLayout>
}

impl PipelineLayoutBuilder {
    #[inline]
    pub fn new() -> Self {
        Self { push_constant_ranges: Vec::new(), descriptor_set_layouts: Vec::new() }  
    }
    #[inline]
    pub fn add_push_constant<P: Sized>(mut self, flags: ShaderStageFlags) -> Self {
        let push_constant_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::from_raw(flags.0),
            size: std::mem::size_of::<P>() as u32,
            ..Default::default()
        };
        self.push_constant_ranges.push(push_constant_range);
        self
    }
    #[inline]
    pub fn add_descriptor_layout(mut self, layout: vk::DescriptorSetLayout) -> Self {
        self.descriptor_set_layouts.push(layout);
        self
    }
    pub fn build(&self, device: Arc<LogicalDevice>) -> Arc<PipelineLayout> {
        let layout_info = vk::PipelineLayoutCreateInfo {
            push_constant_range_count: self.push_constant_ranges.len() as u32,
            p_push_constant_ranges: self.push_constant_ranges.as_ptr(),
            p_set_layouts: self.descriptor_set_layouts.as_ptr(),
            set_layout_count: self.descriptor_set_layouts.len() as u32,
            ..Default::default()
        };
        let layout = unsafe { device.device.create_pipeline_layout(&layout_info, None).unwrap() };
        Arc::new(PipelineLayout { layout, device })
    }
}

impl PipelineLayout {
    #[inline]
    pub fn builder() -> PipelineLayoutBuilder {
        PipelineLayoutBuilder::new()
    }
    pub fn get_layout(&self) -> vk::PipelineLayout { self.layout }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_pipeline_layout(self.layout, None) };
    }
}