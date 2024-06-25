use std::sync::Arc;

use ash::vk;

use crate::device::LogicalDevice;

pub struct ImageView {
    device: Arc<LogicalDevice>,
    view: vk::ImageView,
}

impl ImageView {
    pub fn from_raw(device: Arc<LogicalDevice>, view: vk::ImageView) -> Self {
        Self { device, view }
    }
    #[inline]
    pub fn device(&self) -> Arc<LogicalDevice> {
        self.device.clone()
    }
    #[inline]
    pub fn handle(&self) -> vk::ImageView {
        self.view
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_image_view(self.view, None) };
    }
}