use std::{sync::Arc};

use ash::vk::{self, FramebufferCreateInfo};

use crate::{device::LogicalDevice, error::NightfallError, image::{ImageView, RawImage}, swapchain::Swapchain};

use super::RenderPass;

pub struct Framebuffer {
    device: Arc<LogicalDevice>,
    framebuffer: vk::Framebuffer,
    render_pass: Arc<RenderPass>
}
impl Framebuffer {
    pub fn new(render_pass: Arc<RenderPass>, attachments: &[vk::ImageView], width: u32, height: u32, layers: u32) -> Result<Arc<Self>, NightfallError> {
        let framebuffer = unsafe {
            render_pass.device().device.create_framebuffer(&FramebufferCreateInfo {
                render_pass: render_pass.handle(),
                attachment_count: attachments.len() as u32,
                p_attachments: attachments.as_ptr(),
                width,
                height,
                layers,
                ..Default::default()
            }, None).map_err(NightfallError::from)?
        };

        Ok(Arc::new(Self { device: render_pass.device(), render_pass: render_pass.clone(), framebuffer }))
    }
    pub fn from_swapchain(swapchain: Arc<Swapchain>, render_pass: Arc<RenderPass>) -> Vec<Arc<Self>> {
        swapchain
            .image_views()
            .iter()
            .map(|view|{ Self::new(render_pass.clone(), &[*view], swapchain.width(), swapchain.height(), 1).unwrap() })
            .collect()
    }
    pub fn from_swapchain_ext(swapchain: Arc<Swapchain>, depth: &Vec<ImageView>, render_pass: Arc<RenderPass>) -> Vec<Arc<Self>> {
        swapchain
            .image_views()
            .iter()
            .zip(depth.iter())
            .map(|(swapchain_view, depth_view)|{ Self::new(render_pass.clone(), &[*swapchain_view, depth_view.handle()], swapchain.width(), swapchain.height(), 1).unwrap() })
            .collect()
    }
    #[inline]
    pub fn handle(&self) -> vk::Framebuffer {
        self.framebuffer
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_framebuffer(self.framebuffer, None) };
    }
}