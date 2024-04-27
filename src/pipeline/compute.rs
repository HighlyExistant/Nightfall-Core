use std::sync::Arc;

use ash::vk;

use crate::{buffers::BufferOffset, device::LogicalDevice, error::VulkanError};

use super::{layout::PipelineLayout, shader::{HasShaderStages, Shader}, VulkanPipeline};

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDispatchIndirectCommand.html>"]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl DispatchIndirectCommand {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

pub struct ComputePipeline {
    pub(crate) layout: Arc<PipelineLayout>,
    pub(crate) shader: Arc<Shader>,
    pub(crate) pipeline: vk::Pipeline,
}
impl ComputePipeline {
    pub fn new(
        device: Arc<LogicalDevice>, 
        layout: Arc<PipelineLayout>, 
        shader: Arc<Shader>
    ) -> Result<ComputePipeline, VulkanError> {
        let create_info = vk::ComputePipelineCreateInfo {
            stage: shader.stage,
            layout: layout.get_layout(),
            ..Default::default()
        };
        let pipeline = unsafe { device.device.create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None) }
        .map_err(|err|{VulkanError::from(err.1)})?;
        Ok(unsafe { Self::from_handle(layout.clone(), shader.clone(), pipeline[0]) })
    }
    pub fn new_group<'a>(
        device: Arc<LogicalDevice>, 
        layouts: &'a[Arc<PipelineLayout>], 
        shaders: &'a[Arc<Shader>]
    ) -> Result<impl ExactSizeIterator<Item = ComputePipeline>+'a, VulkanError>  {
        let create_infos = shaders.iter().enumerate().map(|(i, value)|{
            let layout = layouts.get(i).unwrap_or_else(||{
                layouts.last().unwrap()
            }).get_layout();
            vk::ComputePipelineCreateInfo {
                stage: value.stage,
                layout: layout,
                ..Default::default()
            }
        }).collect::<Vec<_>>();
        Self::make_pipelines(device, layouts, shaders, create_infos)
    }
    pub fn new_with_handles<'a>(
        device: Arc<LogicalDevice>,
        layouts: &'a[Arc<PipelineLayout>], 
        shaders: &'a[Arc<Shader>], 
        base_pipeline_handles: &[Option<(&impl VulkanPipeline, i32)>]
    ) -> Result<impl ExactSizeIterator<Item = ComputePipeline>+'a, VulkanError> {
        let create_infos = shaders.iter().enumerate().map(|(i, value)|{
            let (base_pipeline_handle, base_pipeline_index) = base_pipeline_handles[i].map_or((vk::Pipeline::null(), 0), |value|{ (value.0.pipeline_handle(), value.1) });
            let layout = layouts.get(i).unwrap_or_else(||{
                layouts.last().unwrap()
            }).get_layout();
            vk::ComputePipelineCreateInfo {
                stage: value.stage,
                base_pipeline_handle,
                base_pipeline_index,
                layout: layout,
                ..Default::default()
            }
        }).collect::<Vec<_>>();
        Self::make_pipelines(device, layouts, shaders, create_infos)
    }
    fn make_pipelines<'a>(device: Arc<LogicalDevice>, layouts: &'a [Arc<PipelineLayout>], shaders: &'a [Arc<Shader>], create_infos: Vec<vk::ComputePipelineCreateInfo>) -> Result<impl ExactSizeIterator<Item = ComputePipeline>+'a, VulkanError> {
        let mut new_shaders = shaders.iter();
        let pipelines = unsafe { device.device.create_compute_pipelines(vk::PipelineCache::null(), &create_infos, None) }
            .map_err(|err|{VulkanError::from(err.1)})?
            .into_iter()
            .enumerate()
            .map(move |(i, pipeline)|{
                let layout = layouts.get(i).unwrap_or_else(||{
                    layouts.last().unwrap()
                });
                unsafe {
                    ComputePipeline::from_handle(
                        layout.clone(), 
                        new_shaders.next().unwrap().clone(), 
                        pipeline
                    )
                }
            });
        
        Ok(pipelines)
    }
    unsafe fn from_handle(layout: Arc<PipelineLayout>, shader: Arc<Shader>, pipeline: vk::Pipeline) -> Self {
        Self { layout, shader, pipeline }
    }
    pub fn dispatch(&self, cmd: vk::CommandBuffer, x: u32, y: u32, z: u32) {
        unsafe { self.layout.device.device.cmd_dispatch(cmd, x, y, z) };
    }
    pub fn dispatch_indirect(&self, cmd: vk::CommandBuffer, indirect: BufferOffset) {
        unsafe { self.layout.device.device.cmd_dispatch_indirect(cmd, indirect.handle, indirect.offset) }
    }
    pub fn dispatch_base(&self, cmd: vk::CommandBuffer, bx: u32, by: u32, bz: u32, gx: u32, gy: u32, gz: u32) {
        unsafe { self.layout.device.device.cmd_dispatch_base(
        cmd, 
        bx, 
        by, 
        bz,
        gx,
        gy,
        gz
        ) };
    }
    pub fn bind(&self, command_buffer: vk::CommandBuffer) {
        unsafe { self.layout.device.device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, self.pipeline) };
    }
    pub fn device(&self) -> Arc<LogicalDevice> {
        self.layout.device.clone()
    }
    pub fn layout(&self) -> Arc<PipelineLayout> {
        self.layout.clone()
    }
}
impl VulkanPipeline for ComputePipeline {
    fn pipeline_handle(&self) -> vk::Pipeline {
        self.pipeline
    }
}
impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe { self.shader.device.device.destroy_pipeline(self.pipeline, None); }
    }
}