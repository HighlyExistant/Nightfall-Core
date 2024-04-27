mod texture;
mod sampler;
use std::sync::{Arc, Mutex};

use ash::vk::{self, ComponentMapping, ComponentSwizzle, Extent3D, Format};
pub use texture::*;
pub use sampler::*;
mod definitions;
pub use definitions::*;
use crate::{commands::CommandPool, error::VulkanError, memory::DeviceMemory};

use super::{device::LogicalDevice, queue::Queue};

pub struct RawImage {
    image: vk::Image,
    memory: Arc<DeviceMemory>,
    access: vk::AccessFlags,
    format: vk::Format,
    queue: Arc<Queue>,
    width: u32,
    height: u32,
}
pub struct RawImageBuilder {
    create_info: vk::ImageCreateInfo,
}
impl RawImageBuilder {
    pub fn new() -> Self {
        Self { 
            create_info: vk::ImageCreateInfo {
                format: vk::Format::R8G8B8A8_SRGB,
                tiling: vk::ImageTiling::OPTIMAL,
                initial_layout: vk::ImageLayout::UNDEFINED,
                ..Default::default()
            } 
        }
    }
    pub fn set_format(mut self, format: Format) -> Self {
        self.create_info.format = format;
        self
    }
    pub fn initial_layout(mut self, initial_layout: ImageLayout) -> Self {
        self.create_info.initial_layout = vk::ImageLayout::from_raw(initial_layout.0);
        self
    }
    pub fn set_extent(mut self, x: u32, y: u32, z: u32) -> Self {
        self.create_info.extent = Extent3D { width: x, height: y, depth: z };
        self
    }
    pub fn array_layers(mut self, array_layers: u32) -> Self {
        self.create_info.array_layers = array_layers;
        self
    }
    pub fn image_type(mut self, image_type: ImageType) -> Self {
        self.create_info.image_type = vk::ImageType::from_raw(image_type as i32);
        self
    }
    pub fn flags(mut self, flags: ImageCreateFlags) -> Self {
        self.create_info.flags = vk::ImageCreateFlags::from_raw(flags.0);
        self
    }
    pub fn samples(mut self, samples: SampleCountFlags) -> Self {
        self.create_info.samples = vk::SampleCountFlags::from_raw(samples.0);
        self
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.create_info.mip_levels = mip_levels;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self.create_info.sharing_mode = vk::SharingMode::from_raw(sharing_mode.0);
        self
    }
    pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
        self.create_info.usage = vk::ImageUsageFlags::from_raw(usage.0);
        self
    }
    pub fn build(mut self, queue: Arc<Queue>) -> Result<Arc<RawImage>, VulkanError> {
        let (image, memory) = queue.device().create_image(&self.create_info)?;
        Ok(Arc::new(
            RawImage { 
                image, 
                memory, 
                access: vk::AccessFlags::empty(),
                format: self.create_info.format,
                queue,
                width: self.create_info.extent.width,
                height: self.create_info.extent.height,
            }
        ))
    }
}
impl RawImage {
    pub fn from_raw_info(queue: Arc<Queue>, create_info: &vk::ImageCreateInfo) -> Result<Arc<Self>, VulkanError> {
        let (image, memory) = queue.device().create_image(&create_info)?;
        Ok(Arc::new(
            Self { 
                image, 
                memory, 
                access: vk::AccessFlags::empty(),
                format: create_info.format,
                queue,
                width: create_info.extent.width,
                height: create_info.extent.height,
            }
        ))
    }
    fn get_transition(&self, new_layout: vk::ImageLayout, dst_access_mask: vk::AccessFlags, dst_queue_index: Option<u32>) -> Result<vk::ImageMemoryBarrier, VulkanError> {
        let mut barrier = vk::ImageMemoryBarrier {
            new_layout,
            src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            dst_queue_family_index : vk::QUEUE_FAMILY_IGNORED,
            image: self.image,
            src_access_mask: self.access,
            dst_access_mask,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1
            },
            ..Default::default()
        };
        if let Some(dst_idx) = dst_queue_index {
            barrier.src_queue_family_index = self.queue.family_index();
            barrier.dst_queue_family_index = dst_idx;
        }
        Ok(barrier)
    }
    pub fn transition(&self, pool: Arc<CommandPool>, new_layout: vk::ImageLayout, 
        dst_access_mask: vk::AccessFlags, 
        dst_queue_index: Option<u32>, 
        src_stage_mask: vk::PipelineStageFlags, 
        dst_stage_mask: vk::PipelineStageFlags) -> Result<(), VulkanError> {
        self.transition_extended(
            pool, 
            new_layout, 
            dst_access_mask, 
            dst_queue_index, 
            src_stage_mask, 
            dst_stage_mask, 
            &[], 
            &[]
        )
    }
    pub fn transition_extended(
        &self,
        pool: Arc<CommandPool>,
        new_layout: vk::ImageLayout, 
        dst_access_mask: vk::AccessFlags, 
        dst_queue_index: Option<u32>, 
        src_stage_mask: vk::PipelineStageFlags, 
        dst_stage_mask: vk::PipelineStageFlags, 
        memory_barriers: &[vk::MemoryBarrier], 
        buffer_memory_barriers: &[vk::BufferMemoryBarrier]) -> Result<(), VulkanError> {
        let barrier = self.get_transition(new_layout, dst_access_mask, dst_queue_index)?;
        let cmd = self.queue.single_time_commands(pool.clone())?;
        unsafe { 
            self.queue.device().device.cmd_pipeline_barrier(
                    cmd.get_command_buffer(), 
                    src_stage_mask, 
                    dst_stage_mask, 
                    vk::DependencyFlags::empty(), 
                    memory_barriers, 
                    buffer_memory_barriers, 
                    &[barrier]
                ) 
        };
        self.queue.end_single_time_commands(pool, cmd.get_command_buffer());
        Ok(())
    }
    pub fn create_view(&self, subresource_range: vk::ImageSubresourceRange, components: ComponentMapping) -> vk::ImageView {
        let view_info = vk::ImageViewCreateInfo {
            image: self.image,
            view_type: vk::ImageViewType::TYPE_2D,
            format: self.format,
            subresource_range,
            components,
            ..Default::default()
        };
        let view = unsafe { self.queue.device().device.create_image_view(&view_info, None).unwrap() };
        view
    }
    pub fn copy_buffer(&self, pool: Arc<CommandPool>, regions: &[vk::BufferImageCopy], buffer: vk::Buffer) -> Result<(), VulkanError> {
        let cmd = self.queue.single_time_commands(pool.clone())?;
        unsafe { self.queue.device().device.cmd_copy_buffer_to_image(cmd.get_command_buffer(), buffer, self.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, regions) };
        self.queue.end_single_time_commands(pool.clone(), cmd.get_command_buffer());
        Ok(())
    }
    #[inline]
    pub fn handle(&self) -> vk::Image {
        self.image
    }
    #[inline]
    pub fn width(&self) -> usize {
        self.width as usize
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.height as usize
    }
}
impl Drop for RawImage {
    fn drop(&mut self) {
        unsafe { self.queue.device().device.destroy_image(self.image, None) };
    }
}