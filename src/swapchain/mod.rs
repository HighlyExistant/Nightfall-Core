use std::{cell::Cell, mem::MaybeUninit, sync::Arc};

use ash::{prelude::VkResult, vk::{self, ComponentMapping, ComponentSwizzle, SurfaceKHR}};

mod definitions;
pub mod surface;
pub use definitions::*;

use crate::{device::LogicalDevice, error::VulkanError, image::ImageUsageFlags, sync::{Fence, Semaphore}};
#[derive(Default, Clone)]
pub struct SwapchainBuilder {
    pub sharing_queues: Vec<u32>,
    pub flags: SwapchainCreateFlagsKHR,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: [u32; 2],
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub pre_transform: SurfaceTransformFlagsKHR,
    pub composite_alpha: CompositeAlphaFlagsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: bool,
    pub old: Option<vk::SwapchainKHR>
}
pub(crate) unsafe fn read_into_uninitialized_vector<N: Copy + Default + TryInto<usize>, T>(
    f: impl Fn(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: std::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, std::ptr::null_mut()).result()?;
        let mut data =
            Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            err_code.result()?;
            data.set_len(count.try_into().expect("`N` failed to convert to `usize`"));
            break Ok(data);
        }
    }
}
impl SwapchainBuilder {
    pub fn new() -> Self {
        Self {  
            sharing_queues: vec![],
            flags: SwapchainCreateFlagsKHR::empty(),
            min_image_count: 0,
            image_format: Format::B8G8R8A8_UNORM,
            image_color_space: ColorSpaceKHR::SRGB_NONLINEAR,
            image_extent: [0u32; 2],
            image_array_layers: 0,
            image_usage: ImageUsageFlags::empty(),
            pre_transform: SurfaceTransformFlagsKHR::empty(),
            composite_alpha: CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: PresentModeKHR::FIFO,
            clipped: false,
            old: None
        }
    }
    pub fn old_swapchain(mut self, old: vk::SwapchainKHR) -> Self {
        self.old = Some(old);
        self
    }
    pub fn share_queue(mut self, queue_family_idx: u32) -> Self {
        self.sharing_queues.push(queue_family_idx);
        self
    }
    pub fn set_format(mut self, image_format: Format, image_color_space: ColorSpaceKHR) -> Self {
        self.image_format = image_format;
        self.image_color_space = image_color_space;
        self
    }
    pub fn set_image_count(mut self, image_count: u32) -> Self {
        self.min_image_count = image_count;
        self
    }
    pub fn set_extent(mut self, extent: [u32; 2]) -> Self {
        self.image_extent = extent;
        self
    }
    pub fn set_image_usage(mut self, image_usage: ImageUsageFlags  ) -> Self {
        self.image_usage = image_usage;
        self
    }
    pub fn set_image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.image_array_layers = image_array_layers;
        self
    }
    pub fn clipped(mut self) -> Self { self.clipped = true; self }
    fn choose_extent(capabilities: &vk::SurfaceCapabilitiesKHR, extent: [u32; 2]) -> [u32; 2] {
        if capabilities.current_extent.width != std::u32::MAX {
            return [capabilities.current_extent.width, capabilities.current_extent.height];
        }
        let mut actual_extent = extent;
        actual_extent[0] = std::cmp::max(
			capabilities.min_image_extent.width,
			std::cmp::min(capabilities.min_image_extent.width, actual_extent[0]));
            actual_extent[1] = std::cmp::max(
			capabilities.min_image_extent.height,
			std::cmp::min(capabilities.min_image_extent.height, actual_extent[1])
        );
        actual_extent
    }
    fn format_is_present(format: vk::SurfaceFormatKHR, surface_formats: &[vk::SurfaceFormatKHR]) -> bool {
        for f in surface_formats {
            if *f == format {
                return true;
            }
        }
        false
    }
    fn create_image_views(image_count: u32, images: &Vec<vk::Image>, format: vk::Format, device: &std::sync::Arc<LogicalDevice>) -> Vec<vk::ImageView> {
        let views: Vec<vk::ImageView> = images.iter().map(|image| {
            let create_info = vk::ImageViewCreateInfo {
                image: *image,
                format: format,
                view_type: vk::ImageViewType::TYPE_2D,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1
                },
                components: ComponentMapping {
                    r: ComponentSwizzle::IDENTITY,
                    g: ComponentSwizzle::IDENTITY,
                    b: ComponentSwizzle::IDENTITY,
                    a: ComponentSwizzle::IDENTITY,
                },
                
                ..Default::default()
            };

            let view = unsafe { device.device.create_image_view(&create_info, None).unwrap() };
            view
        }).collect();
        views
    }
    pub fn build(self, device: Arc<LogicalDevice>, surface: SurfaceKHR) -> Result<Arc<Swapchain>, VulkanError> {
        let surface_capabilities: vk::SurfaceCapabilitiesKHR = device.instance.get_physical_device_surface_capabilities(
            device.physical_device.clone(), 
            surface
        )?;
        let image_extent = Self::choose_extent(&surface_capabilities, self.image_extent);
        let min_image_count = if !(self.min_image_count > surface_capabilities.min_image_count && 
            self.min_image_count < surface_capabilities.max_image_count) {
                surface_capabilities.min_image_count
        } else {
            self.min_image_count
        };
        let image_sharing_mode = if self.sharing_queues.len() > 1 {
            vk::SharingMode::CONCURRENT
        } else {
            vk::SharingMode::EXCLUSIVE
        };
        let pre_transform = vk::SurfaceTransformFlagsKHR::from_raw(surface_capabilities.current_transform.as_raw());
        let image_array_layers = if self.image_array_layers == 0 { 1 } else { self.image_array_layers };
        let create_info = vk::SwapchainCreateInfoKHR {
            surface,
            clipped: self.clipped as u32,
            composite_alpha: vk::CompositeAlphaFlagsKHR::from_raw(self.composite_alpha.0),
            flags: vk::SwapchainCreateFlagsKHR::from_raw(self.flags.0),
            image_array_layers,
            image_color_space: vk::ColorSpaceKHR::from_raw(self.image_color_space.0),
            image_extent: vk::Extent2D { width: image_extent[0], height: image_extent[1] },
            image_format: vk::Format::from_raw(self.image_format.0),
            image_sharing_mode: image_sharing_mode,
            image_usage: vk::ImageUsageFlags::from_raw(self.image_usage.0),
            min_image_count: min_image_count,
            p_queue_family_indices: self.sharing_queues.as_ptr(),
            queue_family_index_count: self.sharing_queues.len() as u32,
            pre_transform,
            present_mode: vk::PresentModeKHR::from_raw(self.present_mode.0),
            old_swapchain: self.old.map_or(vk::SwapchainKHR::null(), |v|{v}),
            ..Default::default()
        };
        let mut swapchain = MaybeUninit::uninit();
        unsafe {
            (device.fns.khr_swapchain.create_swapchain_khr)(
                device.handle(),
                &create_info,
                std::ptr::null(),
                swapchain.as_mut_ptr()
            ).result().map_err(VulkanError::from)?;
        }
        let handle = unsafe { swapchain.assume_init() };
        // let mut count = 0;
        let images = unsafe { read_into_uninitialized_vector(|count, data| {
                    (device.fns.khr_swapchain.get_swapchain_images_khr)(device.handle(), handle, count, data)
                })? };
        // unsafe {  
        //     (device.fns.khr_swapchain.get_swapchain_images_khr)(
        //         device.handle(),
        //         handle,
        //         &mut count,
        //         std::ptr::null_mut()
        //     ).result().map_err(VulkanError::from)?;
        //     (device.fns.khr_swapchain.get_swapchain_images_khr)(
        //         device.handle(),
        //         handle,
        //         &mut count,
        //         images.as_mut_ptr()
        //     ).result().map_err(VulkanError::from)?;
        //     images.set_len(count as usize) 
        // };
        let views = Self::create_image_views(min_image_count, &images, vk::Format::from_raw(self.image_format.0), &device);
        Ok(Arc::new(Swapchain {  
            handle,
            surface,
            device,
            image_idx: Cell::new(0),
            images,
            views,
            image_format: self.image_format,
            image_usage: self.image_usage,
            color_space: self.image_color_space,
            extent: image_extent,
            cache: SwapchainCached { present_info: vk::PresentInfoKHR::default() },
        }))
    }
}
pub struct SwapchainCached {
    present_info: vk::PresentInfoKHR
}
pub struct Swapchain {
    handle: vk::SwapchainKHR,
    surface: vk::SurfaceKHR,
    device: Arc<LogicalDevice>,
    image_idx: Cell<u32>,
    images: Vec<vk::Image>,
    views: Vec<vk::ImageView>,
    image_format: Format,
    image_usage: ImageUsageFlags,
    color_space: ColorSpaceKHR,
    extent: [u32; 2],
    cache: SwapchainCached,
}

impl Swapchain {
    pub fn builder() -> SwapchainBuilder {
        SwapchainBuilder::new()
    }
    pub fn device(&self) -> Arc<LogicalDevice> {
        self.device.clone()
    }
    pub fn next_image(&self, semaphore: Option<&Semaphore>, fence: Option<Fence>) -> Result<bool, VulkanError> { 
        // let (idx, suboptimal) = unsafe { 
        let mut idx = 0;
        unsafe {
            let err_code = (self.device.fns.khr_swapchain.acquire_next_image_khr)(
                self.device.handle(),
                self.handle, 
                std::u64::MAX, 
                semaphore.map(|v| v.get() ).unwrap_or_default(), 
                fence.map(|v| v.get() ).unwrap_or_default(),
                &mut idx
            );
            self.image_idx.set(idx);
            return match err_code {
                vk::Result::SUCCESS => Ok(false),
                vk::Result::SUBOPTIMAL_KHR => Ok(true),
                _ => Err(VulkanError::from(err_code)),
            };
        }
    }
    pub fn get_images(&self) -> &Vec<vk::Image> { 
        &self.images
    }
    pub fn get_image_index(&self) -> u32 { self.image_idx.get() }
    pub fn recreate(&self, extent: [u32; 2]) -> Result<Arc<Swapchain>, VulkanError> {
        self.device.wait();
        if self.handle == vk::SwapchainKHR::null() {
            SwapchainBuilder::new()
                .clipped()
                .set_extent(extent)
                .set_image_usage(self.image_usage)
                .set_format(self.image_format, self.color_space)
                .build(self.device.clone(), self.surface)
        } else {
            SwapchainBuilder::new()
                .clipped()
                .set_extent(extent)
                .set_image_usage(self.image_usage)
                .old_swapchain(self.handle)
                .set_format(self.image_format, self.color_space)
                .build(self.device.clone(), self.surface)
        }
    }
    pub fn present(&self, queue: vk::Queue, wait: &[vk::Semaphore]) -> Result<bool, VulkanError> {
        let swapchains = [self.handle];
        let idx = self.image_idx.get();
        
        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: wait.len() as u32,
            p_wait_semaphores: wait.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &idx as *const u32,
            ..Default::default()
        };
        let err_code = unsafe { (self.device.fns.khr_swapchain.queue_present_khr)(queue, &present_info) };
        match err_code {
            vk::Result::SUCCESS => Ok(false),
            vk::Result::SUBOPTIMAL_KHR => Ok(true),
            _ => Err(VulkanError::from(err_code)),
        }
    }
    pub fn get_current_image(&self) -> vk::Image {
        self.images[self.get_image_index() as usize]
    }
    pub fn get_current_view(&self) -> vk::ImageView {
        self.views[self.get_image_index() as usize]
    }
    pub fn get_view(&self, idx: usize) -> vk::ImageView {
        self.views[idx]
    }
    pub fn format(&self) -> Format {
        self.image_format
    }
    pub fn color_space(&self) -> ColorSpaceKHR {
        self.color_space
    }
    pub fn image_views(&self) -> &[vk::ImageView] {
        &self.views
    }
    #[inline]
    pub fn extent(&self) -> [u32; 2] { self.extent }
    #[inline]
    pub fn width(&self) -> u32 { self.extent[0] }
    #[inline]
    pub fn height(&self) -> u32 { self.extent[1] }
    #[inline]
    pub fn swapchain(&self) -> vk::SwapchainKHR { self.handle }
    #[inline]
    pub fn image_count(&self) -> usize { self.images.len() }
    #[inline]
    pub fn image_index(&self) -> usize { self.image_idx.get() as usize }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        for view in self.views.iter() {
            unsafe { self.device.device.destroy_image_view(*view, None) };
        }
        unsafe { (self.device.fns.khr_swapchain.destroy_swapchain_khr)(self.device.handle(), self.handle, std::ptr::null()) };
    }
}
