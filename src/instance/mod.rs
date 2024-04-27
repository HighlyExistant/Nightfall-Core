mod extensions;
mod functions;
pub use extensions::*;
pub use functions::*;

use std::borrow::Cow;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::{ffi::CString, sync::Arc};

use ash::vk::{self, SurfaceKHR};

use crate::device::PhysicalDevice;
use crate::entry::ENTRY;
use crate::error::VulkanError;
use crate::swapchain::surface::Surface;
use crate::Version;

use self::{extensions::InstanceExtensions, functions::InstanceFunctions};
use raw_window_handle::{HasDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle, WindowsDisplayHandle};
use raw_window_handle::RawDisplayHandle;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceCreateFlags(pub(crate) u32);
ash::vk_bitflags_wrapped!(InstanceCreateFlags, u32);
impl InstanceCreateFlags {
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)"]
    pub const ENUMERATE_PORTABILITY: Self = Self(1);
}
pub struct InstanceBuilder {
    pub version: Version,
    pub flags: InstanceCreateFlags,
    pub extensions: InstanceExtensions,
    pub layers: Vec<String>,
}
#[cfg(windows)]
const fn raw_display_type() -> RawDisplayHandle {
    use std::mem::MaybeUninit;
    RawDisplayHandle::Windows(unsafe { MaybeUninit::uninit().assume_init() })
}
impl InstanceBuilder {
    pub fn new() -> Self {
        Self { 
            version: Version::new(1, 0, 0), 
            flags: InstanceCreateFlags::empty(), 
            extensions: InstanceExtensions::default(), 
            layers: vec![]
        }
    }
    pub fn set_version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }
    pub fn validation_layers(mut self) -> Self {
        self.layers.push("VK_LAYER_KHRONOS_validation".into());
        self
    }
    pub fn required_windowing_extensions(mut self) -> Self {
        let window_required_extensions = ash_window::enumerate_required_extensions(raw_display_type())
        .unwrap()
        .iter()
        .map(|value|{
            unsafe { CStr::from_ptr(*value as *mut _) }
        });
        let extensions = InstanceExtensions::from_iter(window_required_extensions.map(|value|{value.to_str().unwrap()}));
        self.extensions = extensions | self.extensions;
        self
    }
    pub fn direct_mode_display(mut self) -> Self {
        self.extensions.ext_direct_mode_display = true;
        self.extensions.ext_display_surface_counter = true;
        self.extensions.khr_display = true;
        self.extensions.khr_surface = true;
        self
    }
    pub fn display(mut self) -> Self {
        self.extensions.khr_display = true;
        self.extensions.khr_surface = true;
        self
    }
    pub fn physical_device_property_extension(mut self) -> Self {
        self.extensions.khr_get_display_properties2 = true;
        self
    }
    pub fn get_physical_device_properties2(mut self) -> Self {
        self.extensions.khr_get_physical_device_properties2 = true;
        self
    }
    pub fn device_group_creation_extension(mut self) -> Self {
        self.extensions.khr_device_group_creation = true;
        self
    }
    pub fn build(self) -> Result<Arc<Instance>, VulkanError> {
        let extensions = Vec::<std::ffi::CString>::from(&self.extensions);
        let enabled_extension_names = extensions.iter().map(|str|{ str.as_ptr() }).collect::<Vec<_>>();
        let layers = self.layers.iter().map(|value|{ CString::new(value.as_str()).unwrap() }).collect::<Vec<_>>();
        let enabled_layers = layers.iter().map(|value|{ value.as_ptr() }).collect::<Vec<_>>();
        let application_info = vk::ApplicationInfo {
            api_version: u32::try_from(self.version).unwrap(),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            enabled_extension_count: enabled_extension_names.len() as u32,
            pp_enabled_extension_names: enabled_extension_names.as_ptr(),
            enabled_layer_count: enabled_layers.len() as u32,
            pp_enabled_layer_names: enabled_layers.as_ptr(),
            p_application_info: &application_info,
            flags: vk::InstanceCreateFlags::from_raw(self.flags.0),
            ..Default::default()
        };
        let instance = unsafe { ENTRY.create_instance(&create_info, None) }
            .map_err(|value|{
                VulkanError::from(value)
            }
        )?;
        
        let fns = unsafe { 
            InstanceFunctions::load(|name| {
                    ENTRY
                        .get_instance_proc_addr(instance.handle(), name.as_ptr())
                        .map_or(std::ptr::null(), |func| func as _)
                }
            ) 
        };
    Ok(Arc::new(Instance { 
            instance, 
            fns, 
            version: self.version, 
            flags: self.flags, 
            enabled_extensions: self.extensions, 
            enabled_layers: self.layers, 
        }))
    }
}
pub struct Instance {
    pub(crate) instance: ash::Instance,
    pub(crate) fns: InstanceFunctions,
    pub(crate) version: Version,
    pub(crate) flags: InstanceCreateFlags,
    pub(crate) enabled_extensions: InstanceExtensions,
    pub(crate) enabled_layers: Vec<String>,
}

impl Instance {
    pub fn builder() -> InstanceBuilder {
        InstanceBuilder::new()
    }
    pub fn api_version(&self) -> Version {
        self.version
    }
    pub fn enumerate_physical_devices(self: Arc<Self>) -> Result<impl ExactSizeIterator<Item = Arc<PhysicalDevice>>, VulkanError> {
        
        Ok(unsafe { self.instance.enumerate_physical_devices()
            .map_err(|value|{VulkanError::from(value)})?
            .into_iter()
            .map(|handle|{PhysicalDevice::from_handle(self.clone(), handle)})
            .collect::<Vec<_>>().into_iter()
        })
    }
    pub fn create_surface(self: Arc<Self>, window: &(impl HasRawWindowHandle + HasRawDisplayHandle)) -> Result<Surface, VulkanError> {
        let display = window.raw_display_handle();
        let window_hwnd = window.raw_window_handle();
        self.surface_from_raw_handles(display, window_hwnd)
    }
    pub fn surface_from_raw_handles(self: Arc<Self>, display: RawDisplayHandle, window: RawWindowHandle) -> Result<Surface, VulkanError> {
        unsafe { Surface::from_handles(self.clone(), display, window) }
    }
    pub fn get_physical_device_surface_capabilities(&self, physical_device: Arc<PhysicalDevice>, surface: SurfaceKHR) -> Result<vk::SurfaceCapabilitiesKHR, VulkanError> {
        let mut handle = MaybeUninit::uninit();
        unsafe {
            (self.fns.khr_surface.get_physical_device_surface_capabilities_khr)(
                physical_device.handle,
                surface,
                handle.as_mut_ptr()
            ).result().map_err(VulkanError::from)?;
            Ok(handle.assume_init())
        }
    }
    pub fn get_physical_device_surface_formats(&self, physical_device: Arc<PhysicalDevice>, surface: SurfaceKHR) -> Result<Vec<vk::SurfaceFormatKHR>, VulkanError> {
        let mut count = 0;
        let mut handle = Vec::with_capacity(count as usize);
        unsafe {
            (self.fns.khr_surface.get_physical_device_surface_formats_khr)(
                physical_device.handle,
                surface,
                &mut count,
                std::ptr::null_mut()
            ).result().map_err(VulkanError::from)?;
            (self.fns.khr_surface.get_physical_device_surface_formats_khr)(
                physical_device.handle,
                surface,
                &mut count,
                handle.as_mut_ptr()
            ).result().map_err(VulkanError::from)?;
            handle.set_len(count as usize);
        }
        Ok(handle)
    }
    pub fn get_physical_device_surface_present_modes(&self, physical_device: Arc<PhysicalDevice>, surface: SurfaceKHR) -> Result<Vec<vk::PresentModeKHR>, VulkanError> {
        let mut count = 0;
        let mut handle = Vec::with_capacity(count as usize);
        unsafe {
            (self.fns.khr_surface.get_physical_device_surface_present_modes_khr)(
                physical_device.handle,
                surface,
                &mut count,
                std::ptr::null_mut()
            ).result().map_err(VulkanError::from)?;
            (self.fns.khr_surface.get_physical_device_surface_present_modes_khr)(
                physical_device.handle,
                surface,
                &mut count,
                handle.as_mut_ptr()
            ).result().map_err(VulkanError::from)?;
            handle.set_len(count as usize);
            Ok(handle)
        }
    }
    pub fn instance(&self) -> &ash::Instance {
        &self.instance
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}