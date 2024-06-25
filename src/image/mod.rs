mod texture;
mod sampler;
mod view;
use std::sync::{Arc, Mutex};

use ash::vk::{self, ComponentMapping, ComponentSwizzle, Extent3D, TaggedStructure};
pub use texture::*;
pub use sampler::*;
pub use view::*;
mod definitions;
pub use definitions::*;
use crate::{commands::CommandPool, error::VulkanError, memory::DeviceMemory, swapchain::Format};

use super::{device::LogicalDevice, queue::Queue};

pub struct RawImage {
    image: vk::Image,
    memory: Arc<DeviceMemory>,
    access: vk::AccessFlags,
    format: vk::Format,
    queue: Option<Arc<Queue>>,
    width: u32,
    height: u32,
}
pub struct RawImageBuilder {
    create_info: vk::ImageCreateInfo,
}
impl RawImageBuilder {
    pub const TYPE_1D: Self = Self::new().image_type(ImageType::Type1D);
    pub const TYPE_2D: Self = Self::new().image_type(ImageType::Type2D);
    pub const TYPE_3D: Self = Self::new().image_type(ImageType::Type3D);
    pub const DEPTH_STENCIL_1D: Self = Self::new().image_type(ImageType::Type1D).usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT);
    pub const DEPTH_STENCIL_2D: Self = Self::new().image_type(ImageType::Type2D).usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT);
    pub const DEPTH_STENCIL_3D: Self = Self::new().image_type(ImageType::Type3D).usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT);
    pub const fn new() -> Self {
        Self { 
            create_info: vk::ImageCreateInfo {
                s_type: vk::ImageCreateInfo::STRUCTURE_TYPE,
                p_next: std::ptr::null(),
                flags: vk::ImageCreateFlags::empty(),
                image_type: vk::ImageType::TYPE_1D,
                format: vk::Format::R8G8B8A8_SRGB,
                extent: vk::Extent3D { width: 0, height: 0, depth: 1 },
                mip_levels: 1,
                array_layers: 1,
                samples: vk::SampleCountFlags::TYPE_1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::empty(),
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 0,
                p_queue_family_indices: ::std::ptr::null(),
                initial_layout: vk::ImageLayout::UNDEFINED,
            } 
        }
    }
    pub const fn set_format(mut self, format: Format) -> Self {
        self.create_info.format = vk::Format::from_raw(format.0);
        self
    }
    pub const fn initial_layout(mut self, initial_layout: ImageLayout) -> Self {
        self.create_info.initial_layout = vk::ImageLayout::from_raw(initial_layout.0);
        self
    }
    pub const fn set_extent(mut self, x: u32, y: u32, z: u32) -> Self {
        self.create_info.extent = Extent3D { width: x, height: y, depth: z };
        self
    }
    pub const fn array_layers(mut self, array_layers: u32) -> Self {
        self.create_info.array_layers = array_layers;
        self
    }
    pub const fn image_type(mut self, image_type: ImageType) -> Self {
        self.create_info.image_type = vk::ImageType::from_raw(image_type as i32);
        self
    }
    pub const fn flags(mut self, flags: ImageCreateFlags) -> Self {
        self.create_info.flags = vk::ImageCreateFlags::from_raw(flags.0);
        self
    }
    pub const fn samples(mut self, samples: SampleCountFlags) -> Self {
        self.create_info.samples = vk::SampleCountFlags::from_raw(samples.0);
        self
    }
    pub const fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.create_info.mip_levels = mip_levels;
        self
    }
    pub const fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self.create_info.sharing_mode = vk::SharingMode::from_raw(sharing_mode.0);
        self
    }
    pub const fn usage(mut self, usage: ImageUsageFlags) -> Self {
        self.create_info.usage = vk::ImageUsageFlags::from_raw(usage.0);
        self
    }
    pub fn build(mut self, device: Arc<LogicalDevice>, queue: Option<Arc<Queue>>) -> Result<Arc<RawImage>, VulkanError> {
        let (image, memory) = device.create_image(&self.create_info)?;
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
    #[inline]
    pub const fn builder() -> RawImageBuilder {
        RawImageBuilder::new()
    }
    pub fn from_raw_info(device: Arc<LogicalDevice>, queue: Option<Arc<Queue>>, create_info: &vk::ImageCreateInfo) -> Result<Arc<Self>, VulkanError> {
        let (image, memory) = device.create_image(&create_info)?;
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
    fn get_transition(&self, queue: Arc<Queue>, new_layout: vk::ImageLayout, dst_access_mask: vk::AccessFlags, dst_queue_index: Option<u32>) -> Result<vk::ImageMemoryBarrier, VulkanError> {
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
            barrier.src_queue_family_index = queue.family_index();
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
        if let Some(queue) = &self.queue {
            let barrier = self.get_transition(queue.clone(), new_layout, dst_access_mask, dst_queue_index)?;
            let cmd = queue.single_time_commands(pool.clone())?;
            unsafe { 
                queue.device().device.cmd_pipeline_barrier(
                        cmd.get_command_buffer(), 
                        src_stage_mask, 
                        dst_stage_mask, 
                        vk::DependencyFlags::empty(), 
                        memory_barriers, 
                        buffer_memory_barriers, 
                        &[barrier]
                    ) 
            };
            queue.end_single_time_commands(pool, &cmd);
        } else {
            panic!("No queue selected")
        }
        Ok(())
    }
    pub fn create_view(&self, subresource_range: vk::ImageSubresourceRange, components: ComponentMapping) -> ImageView {
        let view_info = vk::ImageViewCreateInfo {
            image: self.image,
            view_type: vk::ImageViewType::TYPE_2D,
            format: self.format,
            subresource_range,
            components,
            ..Default::default()
        };
        let view = unsafe { self.memory.device.device.create_image_view(&view_info, None).unwrap() };
        ImageView::from_raw(self.memory.device.clone(), view)
    }
    pub fn copy_buffer(&self, pool: Arc<CommandPool>, regions: &[vk::BufferImageCopy], buffer: vk::Buffer) -> Result<(), VulkanError> {
        if let Some(queue) = &self.queue {
            let cmd = queue.single_time_commands(pool.clone())?;
            unsafe { queue.device().device.cmd_copy_buffer_to_image(cmd.get_command_buffer(), buffer, self.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, regions) };
            queue.end_single_time_commands(pool.clone(), &cmd);
        } else {
            panic!("No queue selected")
        }
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
    pub fn device(&self) -> Arc<LogicalDevice> {
        self.memory.device.clone()
    }
}
impl Drop for RawImage {
    fn drop(&mut self) {
        unsafe { self.memory.device.device.destroy_image(self.image, None) };
    }
}