mod extensions;
mod features;
mod functions;
mod physical_device;
use std::{sync::Arc, time::Duration};

use ash::vk;
pub use extensions::*;
pub use features::*;
pub use functions::*;
pub use physical_device::*;

use crate::{error::VulkanError, instance::Instance, memory::{DeviceMemory, DevicePointer}, pipeline::shader::ShaderStageFlags, queue::{DeviceQueueCreateFlags, Queue, QueueBuilder}, sync::Semaphore, PNext, Version};

#[derive(Clone, Default, Debug)]
pub struct LogicalDeviceBuilder {
    pub enabled_extensions: DeviceExtensions,
    pub enabled_features: DeviceFeatures,
    pub queue_builders: Vec<QueueBuilder>,
}
impl LogicalDeviceBuilder {
    pub fn new() -> Self {
        Self {
            enabled_extensions: DeviceExtensions::default(),
            enabled_features: DeviceFeatures::default(),
            queue_builders: vec![],
        }
    }
    pub fn enable_swapchain_extensions(mut self) -> Self {
        self.enabled_extensions.khr_swapchain = true;
        self
    }
    pub fn enable_anisotropic_sampling(mut self) -> Self {
        self.enabled_features.sampler_anisotropy = true;
        self
    }
    pub fn enable_buffer_addressing(mut self) -> Self {
        self.enabled_features.buffer_device_address = true;
        self.enabled_extensions.khr_buffer_device_address = true;
        self
    }
    pub fn enable_float64(mut self) -> Self {
        self.enabled_features.shader_float64 = true;
        self
    }
    pub fn subgroup_ballot(mut self) -> Self {
        self.enabled_extensions.ext_shader_subgroup_ballot = true;
        self
    }
    pub fn descriptor_indexing(
        mut self, 
        partially_bound: bool,
        sampled_image_update_after_bind: bool,
        storage_buffer_update_after_bind: bool,
        storage_image_update_after_bind: bool,
        storage_texel_buffer_update_after_bind: bool,
        uniform_buffer_update_after_bind: bool,
        uniform_texel_buffer_update_after_bind: bool,
        update_unused_while_pending: bool,
        variable_descriptor_count: bool,
    ) -> Self {
        // required
        self.enabled_extensions.khr_maintenance3 = true;
        // enable
        self.enabled_features.descriptor_indexing = true;
        self.enabled_features.descriptor_binding_partially_bound =                        partially_bound;
        self.enabled_features.descriptor_binding_sampled_image_update_after_bind =        sampled_image_update_after_bind;
        self.enabled_features.descriptor_binding_storage_buffer_update_after_bind =       storage_buffer_update_after_bind;
        self.enabled_features.descriptor_binding_storage_image_update_after_bind =        storage_image_update_after_bind;
        self.enabled_features.descriptor_binding_storage_texel_buffer_update_after_bind = storage_texel_buffer_update_after_bind;
        self.enabled_features.descriptor_binding_uniform_buffer_update_after_bind =       uniform_buffer_update_after_bind;
        self.enabled_features.descriptor_binding_uniform_texel_buffer_update_after_bind = uniform_texel_buffer_update_after_bind;
        self.enabled_features.descriptor_binding_update_unused_while_pending =            update_unused_while_pending;
        self.enabled_features.descriptor_binding_variable_descriptor_count =              variable_descriptor_count;
        self.enabled_extensions.ext_descriptor_indexing = true;
        self
    }
    pub fn i16bit_storage_extension(mut self) -> Self {
        // required in Vulkan 1.0 optional in Vulkan 1.1
        self.enabled_extensions.khr_storage_buffer_storage_class = true;
        // enable
        self.enabled_extensions.khr_16bit_storage = true;
        self
    }
    pub fn i8bit_storage_extension(mut self) -> Self {
        // required in Vulkan 1.0 optional in Vulkan 1.1
        self.enabled_extensions.khr_storage_buffer_storage_class = true;
        // enable
        self.enabled_extensions.khr_8bit_storage = true;
        self
    }
    pub fn bind_memory(mut self) -> Self {
        self.enabled_extensions.khr_bind_memory2 = true;
        self
    }
    pub fn maintenance3(mut self) -> Self {
        self.enabled_extensions.khr_maintenance3 = true;
        self
    }
    pub fn device_group(mut self) -> Self {
        self.enabled_extensions.khr_device_group = true;
        self
    }
    pub fn add_queue(mut self, flags: DeviceQueueCreateFlags ,queue_family_index: u32, queue_count: u32, idx: u32,p_queue_priorities: *const f32) -> Self {
        self.queue_builders.push(QueueBuilder {
            flags,
            idx,
            p_queue_priorities,
            queue_count,
            queue_family_index
        });
        self
    }
    pub fn build(self, physical_device: Arc<PhysicalDevice>) -> Result<(Arc<LogicalDevice>, impl ExactSizeIterator<Item = Arc<Queue>>), VulkanError> {
        let mut p_next = PNext::new();
        let enabled_extensions: Vec<std::ffi::CString> = (&self.enabled_extensions).into();
        let enabled_extension_names = enabled_extensions.iter().map(|value|{ value.as_ptr() }).collect::<Vec<_>>();
        let vk10 = self.enabled_features.get_features();
        let mut vk11 = if self.enabled_features.features11_active() && physical_device.version.supports_version_1_1() {
            Some(self.enabled_features.get_vulkan11features())
        } else {
            None
        };
        let mut vk12 = if self.enabled_features.features12_active() && physical_device.version.supports_version_1_2() {
            Some(self.enabled_features.get_vulkan12features())
        } else {
            None
        };
        let mut vk13 = if self.enabled_features.features13_active() && physical_device.version.supports_version_1_3() {
            Some(self.enabled_features.get_vulkan13features())
        } else {
            None
        };
        if let Some(vk11) = &mut vk11 {
            p_next.push_back(vk11)
        }
        if let Some(vk12) = &mut vk12 {
            p_next.push_back(vk12)
        }
        if let Some(vk13) = &mut vk13 {
            p_next.push_back(vk13)
        }
        let queue_create_info = self.queue_builders.iter().map(|val|{
            vk::DeviceQueueCreateInfo {
                queue_family_index: val.queue_family_index,
                flags: vk::DeviceQueueCreateFlags::from_raw(val.flags.0),
                p_queue_priorities: &1.0,
                queue_count: val.queue_count,
                ..Default::default()
            }
        }).collect::<Vec<_>>();
        let create_info = vk::DeviceCreateInfo {
            enabled_extension_count: enabled_extension_names.len() as u32,
            pp_enabled_extension_names: enabled_extension_names.as_ptr(),
            p_enabled_features: &vk10,
            p_next: p_next.use_p_next(),
            queue_create_info_count: queue_create_info.len() as u32,
            p_queue_create_infos: queue_create_info.as_ptr(),
            ..Default::default()
        };
        let handle = unsafe { 
            physical_device.instance.instance.create_device(physical_device.handle, &create_info, None).map_err(|err|{
                VulkanError::from(err)
            })?
        };
        let fns = DeviceFunctions::load(|name| unsafe {
            (physical_device.instance.fns.v1_0.get_device_proc_addr)(handle.handle(), name.as_ptr())
                .map_or(std::ptr::null(), |func| func as _)
        });
        let qfp = physical_device.enumerate_queue_family_properties()
            .iter().map(|val|{ *val }).collect::<Vec<_>>();
    let device = Arc::new(LogicalDevice {
            // for cache reasons
            instance: physical_device.instance.clone(),
            physical_device: physical_device.clone(),
            device: handle,
            enabled_extensions: self.enabled_extensions,
            enabled_features: self.enabled_features,
            fns
        });
        let queues = {
            let device = device.clone();
            self.queue_builders.into_iter().map(move |value|{ unsafe { 
                Queue::new(device.clone(), value.flags, qfp[value.queue_family_index as usize].queue_flags, value.queue_family_index, value.idx) 
            }})
        };
        Ok((device, queues))
    }
}
pub struct LogicalDevice {
    pub(crate) device: ash::Device,
    pub(crate) instance: Arc<Instance>,
    pub(crate) physical_device: Arc<PhysicalDevice>,
    pub(crate) enabled_extensions: DeviceExtensions,
    pub(crate) enabled_features: DeviceFeatures,
    pub(crate) fns: DeviceFunctions,
}

impl LogicalDevice {
    pub fn new(physical_device: Arc<PhysicalDevice>, builder: LogicalDeviceBuilder) -> Result<(Arc<LogicalDevice>, impl ExactSizeIterator<Item = Arc<Queue>>), VulkanError> {
        builder.build(physical_device)
    }
    pub fn get_buffer_memory_requirements(&self, raw: vk::Buffer) -> vk::MemoryRequirements {
        unsafe { self.device.get_buffer_memory_requirements(raw) }
    }
    pub fn handle(&self) -> ash::vk::Device {
        self.device.handle()
    }
    pub fn device(&self) -> &ash::Device {
        &self.device
    }
    pub fn physical_device(&self) -> Arc<PhysicalDevice> {
        self.physical_device.clone()
    }
    pub fn instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }
    pub fn buffer_device_address(&self, buf: vk::Buffer) -> Result<DevicePointer, VulkanError> {
        if self.enabled_features.buffer_device_address {
            let bufinfo = vk::BufferDeviceAddressInfo {
                buffer: buf,
                ..Default::default()
            };
            Ok(DevicePointer(unsafe { (self.device.fp_v1_2().get_buffer_device_address)(self.device.handle(), &bufinfo as *const _) }))
        } else {
            Err(VulkanError::BufferDeviceAddressingDisabled)
        }
    }
    pub fn push_constants<P>(&self, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, stage_flags: ShaderStageFlags, offset: u32, constant: &P) {
        let ptr = unsafe { std::slice::from_raw_parts(constant as *const P as *const u8, std::mem::size_of::<P>()) };
        unsafe { self.device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::from_raw(stage_flags.0), offset, ptr) };
    }
    pub fn wait_semaphores(&self, timeout: Duration, semaphores: &mut impl ExactSizeIterator<Item = Semaphore>) -> Result<(), VulkanError> {
        let semaphores = semaphores.map(|value|{ value.semaphore }).collect::<Vec<_>>();
        let wait = vk::SemaphoreWaitInfo {
            semaphore_count: semaphores.len() as u32,
            p_semaphores: semaphores.as_ptr(),
            ..Default::default()
        };
        unsafe { 
            self.device.wait_semaphores(&wait, timeout.as_nanos() as u64).map_err(VulkanError::from) 
        }
    }
    pub(crate) fn create_image(
        self: Arc<Self>,
        info: &vk::ImageCreateInfo
    ) -> Result<(vk::Image, Arc<DeviceMemory>), VulkanError> {
        let image = unsafe { self.device.create_image(info, None).unwrap() };
        // Allocate Memory
        let requirements = unsafe { self.device.get_image_memory_requirements(image) };
        
        let memory_type_index = self.physical_device.memory_properties.memory_types.iter().enumerate().position(|(i, ty)|{
            requirements.memory_type_bits & (1 << i) == (1 << i) && ty.property_flags & vk::MemoryPropertyFlags::DEVICE_LOCAL == vk::MemoryPropertyFlags::DEVICE_LOCAL
        }).unwrap() as u32;

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: memory_type_index,
            ..Default::default()
        };
        let memory = Arc::new(DeviceMemory::allocate(self.clone(), requirements.size as usize, memory_type_index, std::ptr::null())?);
        
        unsafe { self.device.bind_image_memory(image, memory.handle, 0).unwrap() };
        Ok((image, memory))
    }
    pub fn wait(&self) {
        unsafe { self.device.device_wait_idle().unwrap() };
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe { self.device.destroy_device(None) };
    }
}