use std::{mem::MaybeUninit, sync::Arc};


use ash::vk;

use crate::{entry::ENTRY, instance::Instance, queue::QueueFamilyProperties, swapchain::Format, PNext, Version};

use super::{DeviceExtensions, DeviceFeatures, ExtensionProperties};

pub struct PhysicalDevice {
    pub(crate) handle: ash::vk::PhysicalDevice,
    pub(crate) instance: Arc<Instance>,
    pub(crate) version: Version,
    pub(crate) supported_extensions: DeviceExtensions,
    pub(crate) supported_features: DeviceFeatures,
    pub(crate) extension_properties: Vec<ExtensionProperties>,
    pub(crate) queue_family_properties: Vec<QueueFamilyProperties>,
    pub(crate) memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub(crate) properties: vk::PhysicalDeviceProperties,
}

impl PhysicalDevice {
    unsafe fn get_api_version(instance: Arc<Instance>, handle: ash::vk::PhysicalDevice) -> Version {
        let mut output = MaybeUninit::uninit();
        (instance.fns.v1_0.get_physical_device_properties)(handle, output.as_mut_ptr());
        let api_version = output.assume_init().api_version;
        Version::from(std::cmp::min(api_version, instance.api_version().try_into().unwrap()))
    }
    pub fn properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.properties
    }
    pub fn query_subgroup_properties(&self) -> vk::PhysicalDeviceSubgroupProperties {
        let mut p_next = PNext::new();
        let subgroup_propeties = vk::PhysicalDeviceSubgroupProperties::default();
        p_next.push_front(&subgroup_propeties);
        let mut properties = vk::PhysicalDeviceProperties2 {
            p_next: p_next.use_p_next(),
            ..Default::default()
        };
        unsafe { 
            (self.instance.fns.v1_1.get_physical_device_properties2)(
                    self.handle,
                    &mut properties
        )}
        subgroup_propeties
    }
    pub fn get_supported_extensions(&self) -> &DeviceExtensions {
        &self.supported_extensions
    }
    pub fn get_supported_features(&self) -> &DeviceFeatures {
        &self.supported_features
    }
    pub fn enumerate_queue_family_properties(&self) -> &Vec<QueueFamilyProperties> {
        &self.queue_family_properties
    }
    pub fn get_physical_device_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.memory_properties
    }
    // This function should not fail so the provided device and instance must be valid.
    pub(crate) unsafe fn from_handle(instance: Arc<Instance>, handle: ash::vk::PhysicalDevice) -> Arc<Self> {
        let extension_properties: Vec<ExtensionProperties> = unsafe { instance.instance.enumerate_device_extension_properties(handle) }.unwrap()
            .into_iter()
            .map(Into::into)
            .collect();
            
        let supported_features = DeviceFeatures::get_all_features(&instance.clone(), handle);
        let supported_extensions = extension_properties
            .iter()
            .map(|property| property.extension_name.as_str())
            .collect();
        let version = Self::get_api_version(instance.clone(), handle);
        let queue_family_properties = instance.instance.get_physical_device_queue_family_properties(handle)
            .iter()
            .map(|value|{
                QueueFamilyProperties::from(value)
            }).collect();
        let memory_properties = instance.instance.get_physical_device_memory_properties(handle);
        let properties = instance.instance.get_physical_device_properties(handle);
        Arc::new(PhysicalDevice { 
            handle,
            instance: instance.clone(),
            extension_properties,
            supported_features,
            version,
            supported_extensions,
            queue_family_properties,
            memory_properties,
            properties
        })
    }
    pub fn get_format_properties(&self, format: Format) -> vk::FormatProperties {
        unsafe { self.instance.instance.get_physical_device_format_properties(self.handle(), vk::Format::from_raw(format.0)) }
    }
    pub fn handle(&self) -> vk::PhysicalDevice {
        self.handle
    }
}