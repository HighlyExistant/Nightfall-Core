use std::sync::Arc;

use ash::vk::{self, AttachmentDescription, RenderPassCreateInfo};

use crate::{device::LogicalDevice, error::NightfallError};
pub struct RenderPass {
    device: Arc<LogicalDevice>,
    renderpass: vk::RenderPass,
}

impl RenderPass {
    pub fn new(device: Arc<LogicalDevice>, flags: vk::RenderPassCreateFlags, attachments: &[AttachmentDescription], dependencies: &[vk::SubpassDependency], subpasses: &[vk::SubpassDescription]) -> Result<Arc<Self>, NightfallError> {
        let info = RenderPassCreateInfo {
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: subpasses.len() as u32,
            p_subpasses: subpasses.as_ptr(),
            dependency_count: dependencies.len() as u32,
            p_dependencies: dependencies.as_ptr(),
            flags,
            ..Default::default()
        };
        let renderpass = unsafe {
            device.device.create_render_pass(&info, None).map_err(NightfallError::from)?
        };
        Ok(Arc::new(Self { device, renderpass }))
    }
    #[inline]
    pub fn handle(&self) -> vk::RenderPass {
        self.renderpass
    }
    #[inline]
    pub fn device(&self) -> Arc<LogicalDevice> {
        self.device.clone()
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_render_pass(self.renderpass, None) };
    }
}