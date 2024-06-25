use std::sync::Arc;

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use crate::{entry::ENTRY, error::VulkanError, instance::Instance};

pub struct Surface {
    handle: vk::SurfaceKHR,
    instance: Arc<Instance>,
}

impl Surface {
    pub unsafe fn from_handles(instance: Arc<Instance>, display: RawDisplayHandle, window: RawWindowHandle) -> Result<Self, VulkanError> {
        let handle = ash_window::create_surface(
            &ENTRY, 
            &instance.instance, 
            display, 
            window, 
            None).map_err(VulkanError::from)?;
        Ok(
            Self { handle, instance: instance.clone() }
        )
    }
    pub unsafe fn from_raw(handle: vk::SurfaceKHR, instance: Arc<Instance>) -> Self {
        Self { handle, instance }
    }
    pub fn handle(&self) -> vk::SurfaceKHR {
        self.handle
    }
    pub fn instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { 
            (self.instance.fns.khr_surface.destroy_surface_khr)(
                    self.instance.instance.handle(),
                    self.handle,
                    std::ptr::null()
            ) 
        };
    }
}