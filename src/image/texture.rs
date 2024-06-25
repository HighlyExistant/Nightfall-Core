use std::sync::Arc;

use ash::vk::{self, Extent3D, QUEUE_FAMILY_IGNORED, ImageSubresourceRange, ComponentMapping};

use crate::{buffers::{Buffer, BufferCreateInfo, BufferUsageFlags, MemoryPropertyFlags}, commands::CommandPool, device::LogicalDevice, error::VulkanError, queue::Queue, swapchain::Format};

use super::{ImageUsageFlags, ImageView, PipelineStageFlags, RawImage, Sampler, SamplerBuilder};

pub struct TextureImage {
    image: Arc<RawImage>,
    view: ImageView,
    sampler: Arc<Sampler>,
}

impl TextureImage {
    pub fn new(queue: Arc<Queue>, width: u32, height: u32, usage: ImageUsageFlags) -> Result<Self, VulkanError> {
        let image = Self::raw_create_image(queue.clone(), width, height, vk::Format::R8G8B8A8_SRGB, vk::ImageUsageFlags::from_raw(usage.0))?;
        let sampler = Self::create_sampler(queue.device());
        let view = image.create_view(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
            ..Default::default()
        }, ComponentMapping::default());
        Ok(Self { image, view, sampler })
    }
    pub fn from_image(raw_image: Arc<RawImage>) -> Result<Self, VulkanError> {
        let sampler = Self::create_sampler(raw_image.device());
        let view = raw_image.create_view(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
            ..Default::default()
        }, ComponentMapping::default());
        Ok(Self { image: raw_image.clone(), view, sampler })
    }
    pub fn from_data(queue: Arc<Queue>, width: u32, height: u32, usage: ImageUsageFlags, format: Format, dst_pipeline: PipelineStageFlags, pool: Arc<CommandPool>, pixels: &[u8]) -> Result<Self, VulkanError> {
        let channels = Self::format_size(format);
        let image_size = (width*height*channels) as usize;
        let stage = Buffer::new(queue.device(), BufferCreateInfo {
                size: image_size,
                usage: BufferUsageFlags::TRANSFER_SRC,
                properties: MemoryPropertyFlags::HOST_VISIBLE| MemoryPropertyFlags::HOST_COHERENT,
                ..Default::default()
            }, 
        )?;
        unsafe {
            let guard = stage.raw_map::<u8>(image_size, 0).unwrap();
            std::ptr::copy_nonoverlapping(pixels as *const _ as _, guard, image_size);
            stage.raw_unmap();
        }
        
        let this = Self::new(queue, width, height, usage)?;
        this.buffer_to_image(pool, width, height, stage.buffer(), dst_pipeline)?;
        Ok(this)
    }
    pub fn buffer_to_image(&self, pool: Arc<CommandPool>, width: u32, height: u32, buffer: vk::Buffer, pipeline: PipelineStageFlags) -> Result<(), VulkanError> {
        self.image.transition(
            pool.clone(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL, 
            vk::AccessFlags::TRANSFER_WRITE, 
            None, 
            vk::PipelineStageFlags::TOP_OF_PIPE, 
            vk::PipelineStageFlags::TRANSFER)?;
        self.copy_buffer_to_image(pool.clone(), buffer, width, height)?;
        self.image.transition(
            pool.clone(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL, 
            vk::AccessFlags::SHADER_READ, 
            None, 
            vk::PipelineStageFlags::TRANSFER, 
            vk::PipelineStageFlags::from_raw(pipeline.0))
    }
    fn format_size(format: Format) -> u32 {
        match format {
            Format::R10X6_UNORM_PACK16          |
            Format::R12X4_UNORM_PACK16          |
            Format::B5G6R5_UNORM_PACK16         |
            Format::R5G6B5_UNORM_PACK16         |
            Format::A1R5G5B5_UNORM_PACK16       |
            Format::A4B4G4R4_UNORM_PACK16       |
            Format::A4R4G4B4_UNORM_PACK16       |
            Format::B5G5R5A1_UNORM_PACK16       |
            Format::R4G4B4A4_UNORM_PACK16       |
            Format::R5G5B5A1_UNORM_PACK16       |
            Format::R10X6G10X6_UNORM_2PACK16    |
            Format::R12X4G12X4_UNORM_2PACK16    |
            Format::B4G4R4A4_UNORM_PACK16 => {
                2u32
            }
            Format::R8G8B8_SINT                 |
            Format::R8G8B8_SNORM                |
            Format::R8G8B8_SRGB                 |
            Format::R8G8B8_SSCALED              |
            Format::R8G8B8_UINT                 |
            Format::R8G8B8_UNORM                |
            Format::R8G8B8_USCALED => {
                3u32
            }
            Format::A2B10G10R10_SINT_PACK32     |
            Format::A2B10G10R10_SNORM_PACK32    |
            Format::A2B10G10R10_SSCALED_PACK32  |
            Format::A2B10G10R10_UINT_PACK32     |
            Format::A2B10G10R10_UNORM_PACK32    |
            Format::A8B8G8R8_SINT_PACK32        |
            Format::A8B8G8R8_SNORM_PACK32       |
            Format::A8B8G8R8_SRGB_PACK32        |
            Format::A8B8G8R8_SSCALED_PACK32     |
            Format::A8B8G8R8_UINT_PACK32        |
            Format::A8B8G8R8_UNORM_PACK32       |
            Format::A8B8G8R8_USCALED_PACK32     |
            Format::R8G8B8A8_SINT               |
            Format::R8G8B8A8_SNORM              |
            Format::R8G8B8A8_SRGB               |
            Format::R8G8B8A8_SSCALED            |
            Format::R8G8B8A8_UINT               |
            Format::R8G8B8A8_UNORM              |
            Format::R8G8B8A8_USCALED            |
            Format::B8G8R8A8_SINT               |
            Format::B8G8R8A8_SNORM              |
            Format::B8G8R8A8_SRGB               |
            Format::B8G8R8A8_SSCALED            |
            Format::B8G8R8A8_UINT               |
            Format::B8G8R8A8_UNORM              |
            Format::B8G8R8A8_USCALED            |
            Format::A2B10G10R10_USCALED_PACK32  => {
                4u32
            }
            Format::R16G16B16A16_SINT           |
            Format::R16G16B16A16_SNORM          |
            Format::R16G16B16A16_SSCALED        |
            Format::R16G16B16A16_UINT           |
            Format::R16G16B16A16_UNORM          |
            Format::R16G16B16A16_USCALED  => {
                8u32
            }
            _ => {
                panic!("Invalid Format")
            }
        }
    }
    pub fn raw_create_image(queue: Arc<Queue>, width: u32, height: u32, format: vk::Format, usage: vk::ImageUsageFlags) -> Result<Arc<RawImage>, VulkanError> {
        let create_info = vk::ImageCreateInfo {
            image_type: vk::ImageType::TYPE_2D,
            extent: Extent3D {
                depth: 1,
                width,
                height
            },
            mip_levels: 1,
            array_layers: 1,
            format,
            tiling: vk::ImageTiling::OPTIMAL,
            initial_layout: vk::ImageLayout::UNDEFINED,
            usage: vk::ImageUsageFlags::TRANSFER_DST | usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };
        RawImage::from_raw_info(queue.device(), Some(queue.clone()), &create_info)
    }
    fn copy_buffer_to_image(&self, pool: Arc<CommandPool>, buffer: vk::Buffer, width: u32, height: u32) -> Result<(), VulkanError> {
        let region = vk::BufferImageCopy {
            image_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            image_offset: vk::Offset3D { x: 0, y: 0, z: 0, },
            image_extent: Extent3D { width, height, depth: 1 },
            ..Default::default()
        };
        self.image.copy_buffer(pool, &[region], buffer)
    }
    pub fn create_sampler(device: Arc<LogicalDevice>) -> Arc<Sampler> {
        SamplerBuilder::new()
            .set_all_address_modes(vk::SamplerAddressMode::REPEAT)
            .set_all_filters(vk::Filter::LINEAR)
            .compare_op(vk::CompareOp::ALWAYS)
            .set_mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .border_colors(vk::BorderColor::INT_OPAQUE_BLACK)
            .enable_anistropy(true)
            .build(device.clone()).unwrap()
    }
    pub fn info(&self, layout: vk::ImageLayout) -> vk::DescriptorImageInfo {
        vk::DescriptorImageInfo {
            image_layout: layout,
            sampler: self.sampler.sampler,
            image_view: self.view.handle(),
            ..Default::default()
        }
    }
    #[inline]
    pub fn handle(&self) -> vk::Image {
        self.image.handle()
    }
    #[inline]
    pub fn width(&self) -> usize {
        self.image.width()
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.image.height()
    }
}