// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use bytemuck::cast_slice;

/// Properties of an extension in the loader or a physical device.
#[derive(Clone, Debug)]
pub struct ExtensionProperties {
    /// The name of the extension.
    pub extension_name: String,

    /// The version of the extension.
    pub spec_version: u32,
}

impl From<ash::vk::ExtensionProperties> for ExtensionProperties {
    #[inline]
    fn from(val: ash::vk::ExtensionProperties) -> Self {
        Self {
            extension_name: {
                let bytes = cast_slice(val.extension_name.as_slice());
                let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                String::from_utf8_lossy(&bytes[0..end]).into()
            },
            spec_version: val.spec_version,
        }
    }
}


#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct DeviceExtensions {
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_16bit_storage.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_storage_buffer_storage_class`](crate::device::DeviceExtensions::khr_storage_buffer_storage_class)"]
    pub khr_16bit_storage: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_8bit_storage.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_storage_buffer_storage_class`](crate::device::DeviceExtensions::khr_storage_buffer_storage_class)"]
    pub khr_8bit_storage: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html)\n- Requires all of:\n  - Vulkan API version 1.1\n  - Vulkan API version 1.2 or device extension [`ext_descriptor_indexing`](crate::device::DeviceExtensions::ext_descriptor_indexing)\n  - Vulkan API version 1.2 or device extension [`khr_buffer_device_address`](crate::device::DeviceExtensions::khr_buffer_device_address)\n  - device extension [`khr_deferred_host_operations`](crate::device::DeviceExtensions::khr_deferred_host_operations)"]
    pub khr_acceleration_structure: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_bind_memory2.html)\n- Promoted to Vulkan 1.1"]
    pub khr_bind_memory2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_buffer_device_address.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_device_group`](crate::device::DeviceExtensions::khr_device_group)\n- Conflicts with device extension: [`ext_buffer_device_address`](crate::device::DeviceExtensions::ext_buffer_device_address)"]
    pub khr_buffer_device_address: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_copy_commands2.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_copy_commands2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_create_renderpass2.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_multiview`](crate::device::DeviceExtensions::khr_multiview)\n  - Vulkan API version 1.1 or device extension [`khr_maintenance2`](crate::device::DeviceExtensions::khr_maintenance2)"]
    pub khr_create_renderpass2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dedicated_allocation.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_get_memory_requirements2`](crate::device::DeviceExtensions::khr_get_memory_requirements2)"]
    pub khr_dedicated_allocation: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_deferred_host_operations.html)"]
    pub khr_deferred_host_operations: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_depth_stencil_resolve.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_create_renderpass2`](crate::device::DeviceExtensions::khr_create_renderpass2)"]
    pub khr_depth_stencil_resolve: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_descriptor_update_template.html)\n- Promoted to Vulkan 1.1"]
    pub khr_descriptor_update_template: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_device_group_creation`](crate::instance::InstanceExtensions::khr_device_group_creation)"]
    pub khr_device_group: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display_swapchain.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - instance extension [`khr_display`](crate::instance::InstanceExtensions::khr_display)"]
    pub khr_display_swapchain: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_draw_indirect_count.html)\n- Promoted to Vulkan 1.2"]
    pub khr_draw_indirect_count: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_driver_properties.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_driver_properties: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_depth_stencil_resolve`](crate::device::DeviceExtensions::khr_depth_stencil_resolve)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_dynamic_rendering: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_external_fence_capabilities`](crate::instance::InstanceExtensions::khr_external_fence_capabilities)"]
    pub khr_external_fence: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_fd.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_fence`](crate::device::DeviceExtensions::khr_external_fence)"]
    pub khr_external_fence_fd: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_win32.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_fence`](crate::device::DeviceExtensions::khr_external_fence)"]
    pub khr_external_fence_win32: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_external_memory_capabilities`](crate::instance::InstanceExtensions::khr_external_memory_capabilities)"]
    pub khr_external_memory: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_fd.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)"]
    pub khr_external_memory_fd: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_win32.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)"]
    pub khr_external_memory_win32: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_external_semaphore_capabilities`](crate::instance::InstanceExtensions::khr_external_semaphore_capabilities)"]
    pub khr_external_semaphore: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_fd.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_semaphore`](crate::device::DeviceExtensions::khr_external_semaphore)"]
    pub khr_external_semaphore_fd: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_win32.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_semaphore`](crate::device::DeviceExtensions::khr_external_semaphore)"]
    pub khr_external_semaphore_win32: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_format_feature_flags2.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_format_feature_flags2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_fragment_shader_barycentric.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_fragment_shader_barycentric: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_fragment_shading_rate.html)\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_create_renderpass2`](crate::device::DeviceExtensions::khr_create_renderpass2)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_fragment_shading_rate: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_memory_requirements2.html)\n- Promoted to Vulkan 1.1"]
    pub khr_get_memory_requirements2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_global_priority.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_global_priority: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_image_format_list.html)\n- Promoted to Vulkan 1.2"]
    pub khr_image_format_list: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_imageless_framebuffer.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_maintenance2`](crate::device::DeviceExtensions::khr_maintenance2)\n  - Vulkan API version 1.2 or device extension [`khr_image_format_list`](crate::device::DeviceExtensions::khr_image_format_list)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_imageless_framebuffer: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_incremental_present.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)"]
    pub khr_incremental_present: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance1.html)\n- Promoted to Vulkan 1.1"]
    pub khr_maintenance1: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance2.html)\n- Promoted to Vulkan 1.1"]
    pub khr_maintenance2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance3.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_maintenance3: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance4.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1"]
    pub khr_maintenance4: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
    pub khr_map_memory2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_multiview.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_multiview: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_performance_query: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_executable_properties.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_pipeline_executable_properties: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_library.html)"]
    pub khr_pipeline_library: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_portability_subset.html)\n- Must be enabled if it is supported by the physical device\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_portability_subset: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_id.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_present_id: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_wait.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - device extension [`khr_present_id`](crate::device::DeviceExtensions::khr_present_id)"]
    pub khr_present_wait: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_push_descriptor.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_push_descriptor: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_query.html)\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_spirv_1_4`](crate::device::DeviceExtensions::khr_spirv_1_4)\n  - device extension [`khr_acceleration_structure`](crate::device::DeviceExtensions::khr_acceleration_structure)"]
    pub khr_ray_query: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_maintenance1.html)\n- Requires all of:\n  - device extension [`khr_acceleration_structure`](crate::device::DeviceExtensions::khr_acceleration_structure)"]
    pub khr_ray_tracing_maintenance1: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_pipeline.html)\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_spirv_1_4`](crate::device::DeviceExtensions::khr_spirv_1_4)\n  - device extension [`khr_acceleration_structure`](crate::device::DeviceExtensions::khr_acceleration_structure)"]
    pub khr_ray_tracing_pipeline: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_position_fetch.html)\n- Requires all of:\n  - device extension [`khr_acceleration_structure`](crate::device::DeviceExtensions::khr_acceleration_structure)"]
    pub khr_ray_tracing_position_fetch: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_relaxed_block_layout.html)\n- Promoted to Vulkan 1.1"]
    pub khr_relaxed_block_layout: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_sampler_mirror_clamp_to_edge.html)\n- Promoted to Vulkan 1.2"]
    pub khr_sampler_mirror_clamp_to_edge: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_sampler_ycbcr_conversion.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_maintenance1`](crate::device::DeviceExtensions::khr_maintenance1)\n  - Vulkan API version 1.1 or device extension [`khr_bind_memory2`](crate::device::DeviceExtensions::khr_bind_memory2)\n  - Vulkan API version 1.1 or device extension [`khr_get_memory_requirements2`](crate::device::DeviceExtensions::khr_get_memory_requirements2)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_sampler_ycbcr_conversion: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_separate_depth_stencil_layouts.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.2 or device extension [`khr_create_renderpass2`](crate::device::DeviceExtensions::khr_create_renderpass2)"]
    pub khr_separate_depth_stencil_layouts: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_atomic_int64.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_shader_atomic_int64: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_clock.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_shader_clock: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_draw_parameters.html)\n- Promoted to Vulkan 1.1"]
    pub khr_shader_draw_parameters: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_float16_int8.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_shader_float16_int8: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_float_controls.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_shader_float_controls: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_integer_dot_product.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_shader_integer_dot_product: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_non_semantic_info.html)\n- Promoted to Vulkan 1.3"]
    pub khr_shader_non_semantic_info: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_subgroup_extended_types.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1"]
    pub khr_shader_subgroup_extended_types: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_subgroup_uniform_control_flow.html)\n- Requires all of:\n  - Vulkan API version 1.1"]
    pub khr_shader_subgroup_uniform_control_flow: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_terminate_invocation.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_shader_terminate_invocation: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shared_presentable_image.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - instance extension [`khr_get_surface_capabilities2`](crate::instance::InstanceExtensions::khr_get_surface_capabilities2)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_shared_presentable_image: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_spirv_1_4.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1\n  - Vulkan API version 1.2 or device extension [`khr_shader_float_controls`](crate::device::DeviceExtensions::khr_shader_float_controls)"]
    pub khr_spirv_1_4: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_storage_buffer_storage_class.html)\n- Promoted to Vulkan 1.1"]
    pub khr_storage_buffer_storage_class: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_swapchain: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain_mutable_format.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - Vulkan API version 1.1 or device extension [`khr_maintenance2`](crate::device::DeviceExtensions::khr_maintenance2)\n  - Vulkan API version 1.2 or device extension [`khr_image_format_list`](crate::device::DeviceExtensions::khr_image_format_list)"]
    pub khr_swapchain_mutable_format: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_synchronization2.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_synchronization2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_timeline_semaphore.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_timeline_semaphore: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_uniform_buffer_standard_layout.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_uniform_buffer_standard_layout: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_variable_pointers.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_storage_buffer_storage_class`](crate::device::DeviceExtensions::khr_storage_buffer_storage_class)"]
    pub khr_variable_pointers: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_decode_h264.html)\n- Requires all of:\n  - device extension [`khr_video_decode_queue`](crate::device::DeviceExtensions::khr_video_decode_queue)"]
    pub khr_video_decode_h264: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_decode_h265.html)\n- Requires all of:\n  - device extension [`khr_video_decode_queue`](crate::device::DeviceExtensions::khr_video_decode_queue)"]
    pub khr_video_decode_h265: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_decode_queue.html)\n- Requires all of:\n  - device extension [`khr_video_queue`](crate::device::DeviceExtensions::khr_video_queue)\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)"]
    pub khr_video_decode_queue: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_encode_queue.html)\n- Requires all of:\n  - device extension [`khr_video_queue`](crate::device::DeviceExtensions::khr_video_queue)\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)"]
    pub khr_video_encode_queue: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_queue.html)\n- Requires all of:\n  - Vulkan API version 1.1\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)"]
    pub khr_video_queue: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_vulkan_memory_model.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_vulkan_memory_model: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_keyed_mutevalue.html)\n- Requires all of:\n  - device extension [`khr_external_memory_win32`](crate::device::DeviceExtensions::khr_external_memory_win32)"]
    pub khr_win32_keyed_mutex: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_workgroup_memory_explicit_layout.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_workgroup_memory_explicit_layout: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_zero_initialize_workgroup_memory.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_zero_initialize_workgroup_memory: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_4444_formats.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_4444_formats: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_astc_decode_mode.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_astc_decode_mode: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_attachment_feedback_loop_dynamic_state.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - device extension [`ext_attachment_feedback_loop_layout`](crate::device::DeviceExtensions::ext_attachment_feedback_loop_layout)"]
    pub ext_attachment_feedback_loop_dynamic_state: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_attachment_feedback_loop_layout.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_attachment_feedback_loop_layout: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_blend_operation_advanced.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_blend_operation_advanced: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_border_color_swizzle.html)\n- Requires all of:\n  - device extension [`ext_custom_border_color`](crate::device::DeviceExtensions::ext_custom_border_color)"]
    pub ext_border_color_swizzle: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_buffer_device_address.html)\n- Deprecated by [`khr_buffer_device_address`](crate::device::DeviceExtensions::khr_buffer_device_address)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n- Conflicts with device extension: [`khr_buffer_device_address`](crate::device::DeviceExtensions::khr_buffer_device_address)"]
    pub ext_buffer_device_address: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_calibrated_timestamps: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_color_write_enable.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_color_write_enable: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_conditional_rendering.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_conditional_rendering: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_conservative_rasterization.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_conservative_rasterization: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_custom_border_color.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_custom_border_color: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_marker.html)\n- Promoted to [`ext_debug_utils`](crate::instance::InstanceExtensions::ext_debug_utils)\n- Requires all of:\n  - instance extension [`ext_debug_report`](crate::instance::InstanceExtensions::ext_debug_report)"]
    pub ext_debug_marker: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clamp_zero_one.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_depth_clamp_zero_one: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_control.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_depth_clip_control: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_enable.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_depth_clip_enable: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_range_unrestricted.html)"]
    pub ext_depth_range_unrestricted: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.2 or device extension [`khr_buffer_device_address`](crate::device::DeviceExtensions::khr_buffer_device_address)\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)\n  - Vulkan API version 1.2 or device extension [`ext_descriptor_indexing`](crate::device::DeviceExtensions::ext_descriptor_indexing)"]
    pub ext_descriptor_buffer: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_indexing.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_maintenance3`](crate::device::DeviceExtensions::khr_maintenance3)"]
    pub ext_descriptor_indexing: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_device_address_binding_report.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - instance extension [`ext_debug_utils`](crate::instance::InstanceExtensions::ext_debug_utils)"]
    pub ext_device_address_binding_report: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_device_fault.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_device_fault: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_device_memory_report.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_device_memory_report: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_discard_rectangles.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_discard_rectangles: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_display_control.html)\n- Requires all of:\n  - instance extension [`ext_display_surface_counter`](crate::instance::InstanceExtensions::ext_display_surface_counter)\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)"]
    pub ext_display_control: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_dynamic_rendering_unused_attachments.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.3 or device extension [`khr_dynamic_rendering`](crate::device::DeviceExtensions::khr_dynamic_rendering)"]
    pub ext_dynamic_rendering_unused_attachments: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_extended_dynamic_state: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state2.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_extended_dynamic_state2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_extended_dynamic_state3: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_external_memory_dma_buf.html)\n- Requires all of:\n  - device extension [`khr_external_memory_fd`](crate::device::DeviceExtensions::khr_external_memory_fd)"]
    pub ext_external_memory_dma_buf: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_external_memory_host.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)"]
    pub ext_external_memory_host: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_filter_cubic.html)"]
    pub ext_filter_cubic: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_density_map.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_fragment_density_map: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_density_map2.html)\n- Requires all of:\n  - device extension [`ext_fragment_density_map`](crate::device::DeviceExtensions::ext_fragment_density_map)"]
    pub ext_fragment_density_map2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_shader_interlock.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_fragment_shader_interlock: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)\n  - instance extension [`khr_get_surface_capabilities2`](crate::instance::InstanceExtensions::khr_get_surface_capabilities2)\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)"]
    pub ext_full_screen_exclusive: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_global_priority.html)\n- Promoted to [`khr_global_priority`](crate::device::DeviceExtensions::khr_global_priority)"]
    pub ext_global_priority: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_global_priority_query.html)\n- Promoted to [`khr_global_priority`](crate::device::DeviceExtensions::khr_global_priority)\n- Requires all of:\n  - device extension [`khr_global_priority`](crate::device::DeviceExtensions::khr_global_priority) or device extension [`ext_global_priority`](crate::device::DeviceExtensions::ext_global_priority)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_global_priority_query: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_graphics_pipeline_library.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - device extension [`khr_pipeline_library`](crate::device::DeviceExtensions::khr_pipeline_library)"]
    pub ext_graphics_pipeline_library: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_hdr_metadata.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)"]
    pub ext_hdr_metadata: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_query_reset.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_host_query_reset: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_2d_view_of_3d.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_maintenance1`](crate::device::DeviceExtensions::khr_maintenance1)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_image_2d_view_of_3d: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_compression_control.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_image_compression_control: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_compression_control_swapchain.html)\n- Requires all of:\n  - device extension [`ext_image_compression_control`](crate::device::DeviceExtensions::ext_image_compression_control)"]
    pub ext_image_compression_control_swapchain: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_drm_format_modifier.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_bind_memory2`](crate::device::DeviceExtensions::khr_bind_memory2)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_sampler_ycbcr_conversion`](crate::device::DeviceExtensions::khr_sampler_ycbcr_conversion)\n  - Vulkan API version 1.2 or device extension [`khr_image_format_list`](crate::device::DeviceExtensions::khr_image_format_list)"]
    pub ext_image_drm_format_modifier: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_robustness.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_image_robustness: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_sliced_view_of_3d.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_maintenance1`](crate::device::DeviceExtensions::khr_maintenance1)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_image_sliced_view_of_3d: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_view_min_lod.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_image_view_min_lod: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_index_type_uint8.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_index_type_uint8: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_inline_uniform_block.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_maintenance1`](crate::device::DeviceExtensions::khr_maintenance1)"]
    pub ext_inline_uniform_block: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_legacy_dithering.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_legacy_dithering: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_line_rasterization.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_line_rasterization: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_load_store_op_none.html)"]
    pub ext_load_store_op_none: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_memory_budget.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_memory_budget: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_memory_priority.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_memory_priority: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html)\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_spirv_1_4`](crate::device::DeviceExtensions::khr_spirv_1_4)"]
    pub ext_mesh_shader: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_objects.html)"]
    pub ext_metal_objects: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_multi_draw.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_multi_draw: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_multisampled_render_to_single_sampled.html)\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_create_renderpass2`](crate::device::DeviceExtensions::khr_create_renderpass2)\n  - Vulkan API version 1.2 or device extension [`khr_depth_stencil_resolve`](crate::device::DeviceExtensions::khr_depth_stencil_resolve)"]
    pub ext_multisampled_render_to_single_sampled: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_mutable_descriptor_type.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_maintenance3`](crate::device::DeviceExtensions::khr_maintenance3)"]
    pub ext_mutable_descriptor_type: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_non_seamless_cube_map.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_non_seamless_cube_map: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_opacity_micromap.html)\n- Requires all of:\n  - device extension [`khr_acceleration_structure`](crate::device::DeviceExtensions::khr_acceleration_structure)\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)"]
    pub ext_opacity_micromap: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pageable_device_local_memory.html)\n- Requires all of:\n  - device extension [`ext_memory_priority`](crate::device::DeviceExtensions::ext_memory_priority)"]
    pub ext_pageable_device_local_memory: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pci_bus_info.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_pci_bus_info: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_physical_device_drm.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_physical_device_drm: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_creation_cache_control.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_pipeline_creation_cache_control: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_creation_feedback.html)\n- Promoted to Vulkan 1.3"]
    pub ext_pipeline_creation_feedback: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_library_group_handles.html)\n- Requires all of:\n  - device extension [`khr_ray_tracing_pipeline`](crate::device::DeviceExtensions::khr_ray_tracing_pipeline)\n  - device extension [`khr_pipeline_library`](crate::device::DeviceExtensions::khr_pipeline_library)"]
    pub ext_pipeline_library_group_handles: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_properties.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_pipeline_properties: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_protected_access.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_pipeline_protected_access: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_robustness.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_pipeline_robustness: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_post_depth_coverage.html)"]
    pub ext_post_depth_coverage: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_primitive_topology_list_restart.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_primitive_topology_list_restart: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_primitives_generated_query.html)\n- Requires all of:\n  - device extension [`ext_transform_feedback`](crate::device::DeviceExtensions::ext_transform_feedback)"]
    pub ext_primitives_generated_query: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_private_data.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_private_data: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_provoking_vertevalue.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_provoking_vertex: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_queue_family_foreign.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)"]
    pub ext_queue_family_foreign: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_rasterization_order_attachment_access.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_rasterization_order_attachment_access: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_rgba10x6_formats.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_sampler_ycbcr_conversion`](crate::device::DeviceExtensions::khr_sampler_ycbcr_conversion)"]
    pub ext_rgba10x6_formats: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_robustness2.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_robustness2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_sample_locations: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_sampler_filter_minmavalue.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_sampler_filter_minmax: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_scalar_block_layout.html)\n- Promoted to Vulkan 1.2\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_scalar_block_layout: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_separate_stencil_usage.html)\n- Promoted to Vulkan 1.2"]
    pub ext_separate_stencil_usage: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_atomic_float.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_shader_atomic_float: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_atomic_float2.html)\n- Requires all of:\n  - device extension [`ext_shader_atomic_float`](crate::device::DeviceExtensions::ext_shader_atomic_float)"]
    pub ext_shader_atomic_float2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_demote_to_helper_invocation.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_shader_demote_to_helper_invocation: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_image_atomic_int64.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_shader_image_atomic_int64: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_module_identifier.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.3 or device extension [`ext_pipeline_creation_cache_control`](crate::device::DeviceExtensions::ext_pipeline_creation_cache_control)"]
    pub ext_shader_module_identifier: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.3 or device extension [`khr_dynamic_rendering`](crate::device::DeviceExtensions::khr_dynamic_rendering)"]
    pub ext_shader_object: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_stencil_export.html)"]
    pub ext_shader_stencil_export: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_subgroup_ballot.html)\n- Deprecated by Vulkan 1.2"]
    pub ext_shader_subgroup_ballot: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_subgroup_vote.html)\n- Deprecated by Vulkan 1.1"]
    pub ext_shader_subgroup_vote: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_tile_image.html)\n- Requires all of:\n  - Vulkan API version 1.3"]
    pub ext_shader_tile_image: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_viewport_index_layer.html)\n- Promoted to Vulkan 1.2"]
    pub ext_shader_viewport_index_layer: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_subgroup_size_control.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1"]
    pub ext_subgroup_size_control: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_subpass_merge_feedback.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_subpass_merge_feedback: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_swapchain_maintenance1.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - instance extension [`ext_surface_maintenance1`](crate::instance::InstanceExtensions::ext_surface_maintenance1)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_swapchain_maintenance1: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_texel_buffer_alignment.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_texel_buffer_alignment: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_texture_compression_astc_hdr.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_texture_compression_astc_hdr: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_tooling_info.html)\n- Promoted to Vulkan 1.3"]
    pub ext_tooling_info: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_transform_feedback.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_transform_feedback: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_cache.html)"]
    pub ext_validation_cache: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_attribute_divisor.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_vertex_attribute_divisor: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_input_dynamic_state.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub ext_vertex_input_dynamic_state: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_encode_h264.html)\n- Requires all of:\n  - device extension [`khr_video_encode_queue`](crate::device::DeviceExtensions::khr_video_encode_queue)"]
    pub ext_video_encode_h264: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_encode_h265.html)\n- Requires all of:\n  - device extension [`khr_video_encode_queue`](crate::device::DeviceExtensions::khr_video_encode_queue)"]
    pub ext_video_encode_h265: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_ycbcr_2plane_444_formats.html)\n- Promoted to Vulkan 1.3\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_sampler_ycbcr_conversion`](crate::device::DeviceExtensions::khr_sampler_ycbcr_conversion)"]
    pub ext_ycbcr_2plane_444_formats: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_ycbcr_image_arrays.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_sampler_ycbcr_conversion`](crate::device::DeviceExtensions::khr_sampler_ycbcr_conversion)"]
    pub ext_ycbcr_image_arrays: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VALVE_mutable_descriptor_type.html)\n- Promoted to [`ext_mutable_descriptor_type`](crate::device::DeviceExtensions::ext_mutable_descriptor_type)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_maintenance3`](crate::device::DeviceExtensions::khr_maintenance3)"]
    pub ext_nested_command_buffer: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html)"]
    pub amd_buffer_marker: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_device_coherent_memory.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub amd_device_coherent_memory: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_display_native_hdr.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - instance extension [`khr_get_surface_capabilities2`](crate::instance::InstanceExtensions::khr_get_surface_capabilities2)\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)"]
    pub amd_display_native_hdr: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_draw_indirect_count.html)\n- Promoted to [`khr_draw_indirect_count`](crate::device::DeviceExtensions::khr_draw_indirect_count)"]
    pub amd_draw_indirect_count: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_gcn_shader.html)"]
    pub amd_gcn_shader: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_gpu_shader_half_float.html)\n- Deprecated by [`khr_shader_float16_int8`](crate::device::DeviceExtensions::khr_shader_float16_int8)"]
    pub amd_gpu_shader_half_float: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_gpu_shader_int16.html)\n- Deprecated by [`khr_shader_float16_int8`](crate::device::DeviceExtensions::khr_shader_float16_int8)"]
    pub amd_gpu_shader_int16: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_memory_overallocation_behavior.html)"]
    pub amd_memory_overallocation_behavior: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_mixed_attachment_samples.html)"]
    pub amd_mixed_attachment_samples: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_pipeline_compiler_control.html)"]
    pub amd_pipeline_compiler_control: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_rasterization_order.html)"]
    pub amd_rasterization_order: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_ballot.html)"]
    pub amd_shader_ballot: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub amd_shader_core_properties: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties2.html)\n- Requires all of:\n  - device extension [`amd_shader_core_properties`](crate::device::DeviceExtensions::amd_shader_core_properties)"]
    pub amd_shader_core_properties2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_early_and_late_fragment_tests.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub amd_shader_early_and_late_fragment_tests: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_explicit_vertex_parameter.html)"]
    pub amd_shader_explicit_vertex_parameter: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_fragment_mask.html)"]
    pub amd_shader_fragment_mask: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_image_load_store_lod.html)"]
    pub amd_shader_image_load_store_lod: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_info.html)"]
    pub amd_shader_info: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_trinary_minmavalue.html)"]
    pub amd_shader_trinary_minmax: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_texture_gather_bias_lod.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub amd_texture_gather_bias_lod: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_ANDROID_external_memory_android_hardware_buffer.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_sampler_ycbcr_conversion`](crate::device::DeviceExtensions::khr_sampler_ycbcr_conversion)\n  - Vulkan API version 1.1 or device extension [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)\n  - device extension [`ext_queue_family_foreign`](crate::device::DeviceExtensions::ext_queue_family_foreign)\n  - Vulkan API version 1.1 or device extension [`khr_dedicated_allocation`](crate::device::DeviceExtensions::khr_dedicated_allocation)"]
    pub android_external_memory_android_hardware_buffer: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_ARM_rasterization_order_attachment_access.html)\n- Promoted to [`ext_rasterization_order_attachment_access`](crate::device::DeviceExtensions::ext_rasterization_order_attachment_access)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub arm_rasterization_order_attachment_access: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_ARM_shader_core_builtins.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub arm_shader_core_builtins: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_ARM_shader_core_properties.html)\n- Requires all of:\n  - Vulkan API version 1.1"]
    pub arm_shader_core_properties: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_buffer_collection.html)\n- Requires all of:\n  - device extension [`fuchsia_external_memory`](crate::device::DeviceExtensions::fuchsia_external_memory)\n  - Vulkan API version 1.1 or device extension [`khr_sampler_ycbcr_conversion`](crate::device::DeviceExtensions::khr_sampler_ycbcr_conversion)"]
    pub fuchsia_buffer_collection: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_external_memory.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_external_memory_capabilities`](crate::instance::InstanceExtensions::khr_external_memory_capabilities)\n  - Vulkan API version 1.1 or device extension [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)"]
    pub fuchsia_external_memory: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_external_semaphore.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_external_semaphore_capabilities`](crate::instance::InstanceExtensions::khr_external_semaphore_capabilities)\n  - Vulkan API version 1.1 or device extension [`khr_external_semaphore`](crate::device::DeviceExtensions::khr_external_semaphore)"]
    pub fuchsia_external_semaphore: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GGP_frame_token.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - instance extension [`ggp_stream_descriptor_surface`](crate::instance::InstanceExtensions::ggp_stream_descriptor_surface)"]
    pub ggp_frame_token: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_decorate_string.html)"]
    pub google_decorate_string: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)"]
    pub google_display_timing: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_hlsl_functionality1.html)"]
    pub google_hlsl_functionality1: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_user_type.html)"]
    pub google_user_type: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_cluster_culling_shader.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub huawei_cluster_culling_shader: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_invocation_mask.html)\n- Requires all of:\n  - device extension [`khr_ray_tracing_pipeline`](crate::device::DeviceExtensions::khr_ray_tracing_pipeline)\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)"]
    pub huawei_invocation_mask: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_subpass_shading.html)\n- Requires all of:\n  - Vulkan API version 1.2 or device extension [`khr_create_renderpass2`](crate::device::DeviceExtensions::khr_create_renderpass2)\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)"]
    pub huawei_subpass_shading: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_IMG_filter_cubic.html)"]
    pub img_filter_cubic: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_IMG_format_pvrtc.html)\n- Deprecated without a replacement"]
    pub img_format_pvrtc: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_INTEL_performance_query.html)"]
    pub intel_performance_query: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_INTEL_shader_integer_functions2.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub intel_shader_integer_functions2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NVX_binary_import.html)"]
    pub nvx_binary_import: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NVX_image_view_handle.html)"]
    pub nvx_image_view_handle: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NVX_multiview_per_view_attributes.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_multiview`](crate::device::DeviceExtensions::khr_multiview)"]
    pub nvx_multiview_per_view_attributes: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_acquire_winrt_display.html)\n- Requires all of:\n  - instance extension [`ext_direct_mode_display`](crate::instance::InstanceExtensions::ext_direct_mode_display)"]
    pub nv_acquire_winrt_display: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_clip_space_w_scaling.html)"]
    pub nv_clip_space_w_scaling: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_compute_shader_derivatives.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_compute_shader_derivatives: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_cooperative_matrivalue.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_cooperative_matrix: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_copy_memory_indirect.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.2 or device extension [`khr_buffer_device_address`](crate::device::DeviceExtensions::khr_buffer_device_address)"]
    pub nv_copy_memory_indirect: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_corner_sampled_image.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_corner_sampled_image: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_coverage_reduction_mode.html)\n- Requires all of:\n  - device extension [`nv_framebuffer_mixed_samples`](crate::device::DeviceExtensions::nv_framebuffer_mixed_samples)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_coverage_reduction_mode: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_dedicated_allocation.html)\n- Deprecated by [`khr_dedicated_allocation`](crate::device::DeviceExtensions::khr_dedicated_allocation)"]
    pub nv_dedicated_allocation: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_dedicated_allocation_image_aliasing.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_dedicated_allocation`](crate::device::DeviceExtensions::khr_dedicated_allocation)\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_dedicated_allocation_image_aliasing: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostic_checkpoints.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_device_diagnostic_checkpoints: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostics_config.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_device_diagnostics_config: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_generated_commands.html)\n- Requires all of:\n  - Vulkan API version 1.1\n  - Vulkan API version 1.2 or device extension [`khr_buffer_device_address`](crate::device::DeviceExtensions::khr_buffer_device_address)"]
    pub nv_device_generated_commands: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_displacement_micromap.html)\n- Requires all of:\n  - device extension [`ext_opacity_micromap`](crate::device::DeviceExtensions::ext_opacity_micromap)"]
    pub nv_displacement_micromap: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory.html)\n- Deprecated by [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)\n- Requires all of:\n  - instance extension [`nv_external_memory_capabilities`](crate::instance::InstanceExtensions::nv_external_memory_capabilities)"]
    pub nv_external_memory: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_rdma.html)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_external_memory`](crate::device::DeviceExtensions::khr_external_memory)"]
    pub nv_external_memory_rdma: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_win32.html)\n- Deprecated by [`khr_external_memory_win32`](crate::device::DeviceExtensions::khr_external_memory_win32)\n- Requires all of:\n  - device extension [`nv_external_memory`](crate::device::DeviceExtensions::nv_external_memory)"]
    pub nv_external_memory_win32: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_fill_rectangle.html)"]
    pub nv_fill_rectangle: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_coverage_to_color.html)"]
    pub nv_fragment_coverage_to_color: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_shader_barycentric.html)\n- Promoted to [`khr_fragment_shader_barycentric`](crate::device::DeviceExtensions::khr_fragment_shader_barycentric)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_fragment_shader_barycentric: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_shading_rate_enums.html)\n- Requires all of:\n  - device extension [`khr_fragment_shading_rate`](crate::device::DeviceExtensions::khr_fragment_shading_rate)"]
    pub nv_fragment_shading_rate_enums: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_framebuffer_mixed_samples.html)"]
    pub nv_framebuffer_mixed_samples: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_geometry_shader_passthrough.html)"]
    pub nv_geometry_shader_passthrough: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_glsl_shader.html)\n- Deprecated without a replacement"]
    pub nv_glsl_shader: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_inherited_viewport_scissor.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_inherited_viewport_scissor: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_linear_color_attachment.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_linear_color_attachment: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_low_latency.html)"]
    pub nv_low_latency: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_memory_decompression.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.2 or device extension [`khr_buffer_device_address`](crate::device::DeviceExtensions::khr_buffer_device_address)"]
    pub nv_memory_decompression: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_mesh_shader.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_mesh_shader: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_optical_flow.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.3 or device extension [`khr_format_feature_flags2`](crate::device::DeviceExtensions::khr_format_feature_flags2)\n  - Vulkan API version 1.3 or device extension [`khr_synchronization2`](crate::device::DeviceExtensions::khr_synchronization2)"]
    pub nv_optical_flow: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_present_barrier.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)\n  - instance extension [`khr_get_surface_capabilities2`](crate::instance::InstanceExtensions::khr_get_surface_capabilities2)\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)"]
    pub nv_present_barrier: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_ray_tracing.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - Vulkan API version 1.1 or device extension [`khr_get_memory_requirements2`](crate::device::DeviceExtensions::khr_get_memory_requirements2)"]
    pub nv_ray_tracing: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_ray_tracing_invocation_reorder.html)\n- Requires all of:\n  - device extension [`khr_ray_tracing_pipeline`](crate::device::DeviceExtensions::khr_ray_tracing_pipeline)"]
    pub nv_ray_tracing_invocation_reorder: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_ray_tracing_motion_blur.html)\n- Requires all of:\n  - device extension [`khr_ray_tracing_pipeline`](crate::device::DeviceExtensions::khr_ray_tracing_pipeline)"]
    pub nv_ray_tracing_motion_blur: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_representative_fragment_test.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_representative_fragment_test: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_sample_mask_override_coverage.html)"]
    pub nv_sample_mask_override_coverage: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_scissor_exclusive.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_scissor_exclusive: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_shader_image_footprint.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_shader_image_footprint: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_shader_sm_builtins.html)\n- Requires all of:\n  - Vulkan API version 1.1"]
    pub nv_shader_sm_builtins: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_shader_subgroup_partitioned.html)\n- Requires all of:\n  - Vulkan API version 1.1"]
    pub nv_shader_subgroup_partitioned: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_shading_rate_image.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub nv_shading_rate_image: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_viewport_array2.html)"]
    pub nv_viewport_array2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_viewport_swizzle.html)"]
    pub nv_viewport_swizzle: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_win32_keyed_mutevalue.html)\n- Promoted to [`khr_win32_keyed_mutex`](crate::device::DeviceExtensions::khr_win32_keyed_mutex)\n- Requires all of:\n  - device extension [`nv_external_memory_win32`](crate::device::DeviceExtensions::nv_external_memory_win32)"]
    pub nv_win32_keyed_mutex: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_fragment_density_map_offset.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)\n  - device extension [`ext_fragment_density_map`](crate::device::DeviceExtensions::ext_fragment_density_map)"]
    pub qcom_fragment_density_map_offset: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_image_processing.html)\n- Requires all of:\n  - Vulkan API version 1.3 or device extension [`khr_format_feature_flags2`](crate::device::DeviceExtensions::khr_format_feature_flags2)"]
    pub qcom_image_processing: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_multiview_per_view_render_areas.html)"]
    pub qcom_multiview_per_view_render_areas: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_multiview_per_view_viewports.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub qcom_multiview_per_view_viewports: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_render_pass_shader_resolve.html)"]
    pub qcom_render_pass_shader_resolve: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_render_pass_store_ops.html)"]
    pub qcom_render_pass_store_ops: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_render_pass_transform.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub qcom_render_pass_transform: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_rotated_copy_commands.html)\n- Requires all of:\n  - device extension [`khr_swapchain`](crate::device::DeviceExtensions::khr_swapchain)\n  - Vulkan API version 1.3 or device extension [`khr_copy_commands2`](crate::device::DeviceExtensions::khr_copy_commands2)"]
    pub qcom_rotated_copy_commands: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QCOM_tile_properties.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub qcom_tile_properties: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_SEC_amigo_profiling.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub sec_amigo_profiling: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VALVE_descriptor_set_host_mapping.html)\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub valve_descriptor_set_host_mapping: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VALVE_mutable_descriptor_type.html)\n- Promoted to [`ext_mutable_descriptor_type`](crate::device::DeviceExtensions::ext_mutable_descriptor_type)\n- Requires all of:\n  - Vulkan API version 1.1 or device extension [`khr_maintenance3`](crate::device::DeviceExtensions::khr_maintenance3)"]
    pub valve_mutable_descriptor_type: bool,
}

impl<'a> From<&'a DeviceExtensions> for Vec<std::ffi::CString> {
    fn from(value: &'a DeviceExtensions) -> Self {
        let mut data = Self::new();
        if value.khr_16bit_storage {
            data.push(std::ffi::CString::new("VK_KHR_16bit_storage").unwrap());
        }
        if value.khr_8bit_storage {
            data.push(std::ffi::CString::new("VK_KHR_8bit_storage").unwrap());
        }
        if value.khr_acceleration_structure {
            data.push(std::ffi::CString::new("VK_KHR_acceleration_structure").unwrap());
        }
        if value.khr_bind_memory2 {
            data.push(std::ffi::CString::new("VK_KHR_bind_memory2").unwrap());
        }
        if value.khr_buffer_device_address {
            data.push(std::ffi::CString::new("VK_KHR_buffer_device_address").unwrap());
        }
        if value.khr_copy_commands2 {
            data.push(std::ffi::CString::new("VK_KHR_copy_commands2").unwrap());
        }
        if value.khr_create_renderpass2 {
            data.push(std::ffi::CString::new("VK_KHR_create_renderpass2").unwrap());
        }
        if value.khr_dedicated_allocation {
            data.push(std::ffi::CString::new("VK_KHR_dedicated_allocation").unwrap());
        }
        if value.khr_deferred_host_operations {
            data.push(std::ffi::CString::new("VK_KHR_deferred_host_operations").unwrap());
        }
        if value.khr_depth_stencil_resolve {
            data.push(std::ffi::CString::new("VK_KHR_depth_stencil_resolve").unwrap());
        }
        if value.khr_descriptor_update_template {
            data.push(std::ffi::CString::new("VK_KHR_descriptor_update_template").unwrap());
        }
        if value.khr_device_group {
            data.push(std::ffi::CString::new("VK_KHR_device_group").unwrap());
        }
        if value.khr_display_swapchain {
            data.push(std::ffi::CString::new("VK_KHR_display_swapchain").unwrap());
        }
        if value.khr_draw_indirect_count {
            data.push(std::ffi::CString::new("VK_KHR_draw_indirect_count").unwrap());
        }
        if value.khr_driver_properties {
            data.push(std::ffi::CString::new("VK_KHR_driver_properties").unwrap());
        }
        if value.khr_dynamic_rendering {
            data.push(std::ffi::CString::new("VK_KHR_dynamic_rendering").unwrap());
        }
        if value.khr_external_fence {
            data.push(std::ffi::CString::new("VK_KHR_external_fence").unwrap());
        }
        if value.khr_external_fence_fd {
            data.push(std::ffi::CString::new("VK_KHR_external_fence_fd").unwrap());
        }
        if value.khr_external_fence_win32 {
            data.push(std::ffi::CString::new("VK_KHR_external_fence_win32").unwrap());
        }
        if value.khr_external_memory {
            data.push(std::ffi::CString::new("VK_KHR_external_memory").unwrap());
        }
        if value.khr_external_memory_fd {
            data.push(std::ffi::CString::new("VK_KHR_external_memory_fd").unwrap());
        }
        if value.khr_external_memory_win32 {
            data.push(std::ffi::CString::new("VK_KHR_external_memory_win32").unwrap());
        }
        if value.khr_external_semaphore {
            data.push(std::ffi::CString::new("VK_KHR_external_semaphore").unwrap());
        }
        if value.khr_external_semaphore_fd {
            data.push(std::ffi::CString::new("VK_KHR_external_semaphore_fd").unwrap());
        }
        if value.khr_external_semaphore_win32 {
            data.push(std::ffi::CString::new("VK_KHR_external_semaphore_win32").unwrap());
        }
        if value.khr_format_feature_flags2 {
            data.push(std::ffi::CString::new("VK_KHR_format_feature_flags2").unwrap());
        }
        if value.khr_fragment_shader_barycentric {
            data.push(std::ffi::CString::new("VK_KHR_fragment_shader_barycentric").unwrap());
        }
        if value.khr_fragment_shading_rate {
            data.push(std::ffi::CString::new("VK_KHR_fragment_shading_rate").unwrap());
        }
        if value.khr_get_memory_requirements2 {
            data.push(std::ffi::CString::new("VK_KHR_get_memory_requirements2").unwrap());
        }
        if value.khr_global_priority {
            data.push(std::ffi::CString::new("VK_KHR_global_priority").unwrap());
        }
        if value.khr_image_format_list {
            data.push(std::ffi::CString::new("VK_KHR_image_format_list").unwrap());
        }
        if value.khr_imageless_framebuffer {
            data.push(std::ffi::CString::new("VK_KHR_imageless_framebuffer").unwrap());
        }
        if value.khr_incremental_present {
            data.push(std::ffi::CString::new("VK_KHR_incremental_present").unwrap());
        }
        if value.khr_maintenance1 {
            data.push(std::ffi::CString::new("VK_KHR_maintenance1").unwrap());
        }
        if value.khr_maintenance2 {
            data.push(std::ffi::CString::new("VK_KHR_maintenance2").unwrap());
        }
        if value.khr_maintenance3 {
            data.push(std::ffi::CString::new("VK_KHR_maintenance3").unwrap());
        }
        if value.khr_maintenance4 {
            data.push(std::ffi::CString::new("VK_KHR_maintenance4").unwrap());
        }
        if value.khr_map_memory2 {
            data.push(std::ffi::CString::new("VK_KHR_map_memory2").unwrap());
        }
        if value.khr_multiview {
            data.push(std::ffi::CString::new("VK_KHR_multiview").unwrap());
        }
        if value.khr_performance_query {
            data.push(std::ffi::CString::new("VK_KHR_performance_query").unwrap());
        }
        if value.khr_pipeline_executable_properties {
            data.push(std::ffi::CString::new("VK_KHR_pipeline_executable_properties").unwrap());
        }
        if value.khr_pipeline_library {
            data.push(std::ffi::CString::new("VK_KHR_pipeline_library").unwrap());
        }
        if value.khr_portability_subset {
            data.push(std::ffi::CString::new("VK_KHR_portability_subset").unwrap());
        }
        if value.khr_present_id {
            data.push(std::ffi::CString::new("VK_KHR_present_id").unwrap());
        }
        if value.khr_present_wait {
            data.push(std::ffi::CString::new("VK_KHR_present_wait").unwrap());
        }
        if value.khr_push_descriptor {
            data.push(std::ffi::CString::new("VK_KHR_push_descriptor").unwrap());
        }
        if value.khr_ray_query {
            data.push(std::ffi::CString::new("VK_KHR_ray_query").unwrap());
        }
        if value.khr_ray_tracing_maintenance1 {
            data.push(std::ffi::CString::new("VK_KHR_ray_tracing_maintenance1").unwrap());
        }
        if value.khr_ray_tracing_pipeline {
            data.push(std::ffi::CString::new("VK_KHR_ray_tracing_pipeline").unwrap());
        }
        if value.khr_ray_tracing_position_fetch {
            data.push(std::ffi::CString::new("VK_KHR_ray_tracing_position_fetch").unwrap());
        }
        if value.khr_relaxed_block_layout {
            data.push(std::ffi::CString::new("VK_KHR_relaxed_block_layout").unwrap());
        }
        if value.khr_sampler_mirror_clamp_to_edge {
            data.push(std::ffi::CString::new("VK_KHR_sampler_mirror_clamp_to_edge").unwrap());
        }
        if value.khr_sampler_ycbcr_conversion {
            data.push(std::ffi::CString::new("VK_KHR_sampler_ycbcr_conversion").unwrap());
        }
        if value.khr_separate_depth_stencil_layouts {
            data.push(std::ffi::CString::new("VK_KHR_separate_depth_stencil_layouts").unwrap());
        }
        if value.khr_shader_atomic_int64 {
            data.push(std::ffi::CString::new("VK_KHR_shader_atomic_int64").unwrap());
        }
        if value.khr_shader_clock {
            data.push(std::ffi::CString::new("VK_KHR_shader_clock").unwrap());
        }
        if value.khr_shader_draw_parameters {
            data.push(std::ffi::CString::new("VK_KHR_shader_draw_parameters").unwrap());
        }
        if value.khr_shader_float16_int8 {
            data.push(std::ffi::CString::new("VK_KHR_shader_float16_int8").unwrap());
        }
        if value.khr_shader_float_controls {
            data.push(std::ffi::CString::new("VK_KHR_shader_float_controls").unwrap());
        }
        if value.khr_shader_integer_dot_product {
            data.push(std::ffi::CString::new("VK_KHR_shader_integer_dot_product").unwrap());
        }
        if value.khr_shader_non_semantic_info {
            data.push(std::ffi::CString::new("VK_KHR_shader_non_semantic_info").unwrap());
        }
        if value.khr_shader_subgroup_extended_types {
            data.push(std::ffi::CString::new("VK_KHR_shader_subgroup_extended_types").unwrap());
        }
        if value.khr_shader_subgroup_uniform_control_flow {
            data.push(
                std::ffi::CString::new("VK_KHR_shader_subgroup_uniform_control_flow").unwrap(),
            );
        }
        if value.khr_shader_terminate_invocation {
            data.push(std::ffi::CString::new("VK_KHR_shader_terminate_invocation").unwrap());
        }
        if value.khr_shared_presentable_image {
            data.push(std::ffi::CString::new("VK_KHR_shared_presentable_image").unwrap());
        }
        if value.khr_spirv_1_4 {
            data.push(std::ffi::CString::new("VK_KHR_spirv_1_4").unwrap());
        }
        if value.khr_storage_buffer_storage_class {
            data.push(std::ffi::CString::new("VK_KHR_storage_buffer_storage_class").unwrap());
        }
        if value.khr_swapchain {
            data.push(std::ffi::CString::new("VK_KHR_swapchain").unwrap());
        }
        if value.khr_swapchain_mutable_format {
            data.push(std::ffi::CString::new("VK_KHR_swapchain_mutable_format").unwrap());
        }
        if value.khr_synchronization2 {
            data.push(std::ffi::CString::new("VK_KHR_synchronization2").unwrap());
        }
        if value.khr_timeline_semaphore {
            data.push(std::ffi::CString::new("VK_KHR_timeline_semaphore").unwrap());
        }
        if value.khr_uniform_buffer_standard_layout {
            data.push(std::ffi::CString::new("VK_KHR_uniform_buffer_standard_layout").unwrap());
        }
        if value.khr_variable_pointers {
            data.push(std::ffi::CString::new("VK_KHR_variable_pointers").unwrap());
        }
        if value.khr_video_decode_h264 {
            data.push(std::ffi::CString::new("VK_KHR_video_decode_h264").unwrap());
        }
        if value.khr_video_decode_h265 {
            data.push(std::ffi::CString::new("VK_KHR_video_decode_h265").unwrap());
        }
        if value.khr_video_decode_queue {
            data.push(std::ffi::CString::new("VK_KHR_video_decode_queue").unwrap());
        }
        if value.khr_video_encode_queue {
            data.push(std::ffi::CString::new("VK_KHR_video_encode_queue").unwrap());
        }
        if value.khr_video_queue {
            data.push(std::ffi::CString::new("VK_KHR_video_queue").unwrap());
        }
        if value.khr_vulkan_memory_model {
            data.push(std::ffi::CString::new("VK_KHR_vulkan_memory_model").unwrap());
        }
        if value.khr_win32_keyed_mutex {
            data.push(std::ffi::CString::new("VK_KHR_win32_keyed_mutex").unwrap());
        }
        if value.khr_workgroup_memory_explicit_layout {
            data.push(std::ffi::CString::new("VK_KHR_workgroup_memory_explicit_layout").unwrap());
        }
        if value.khr_zero_initialize_workgroup_memory {
            data.push(std::ffi::CString::new("VK_KHR_zero_initialize_workgroup_memory").unwrap());
        }
        if value.ext_4444_formats {
            data.push(std::ffi::CString::new("VK_EXT_4444_formats").unwrap());
        }
        if value.ext_astc_decode_mode {
            data.push(std::ffi::CString::new("VK_EXT_astc_decode_mode").unwrap());
        }
        if value.ext_attachment_feedback_loop_dynamic_state {
            data.push(
                std::ffi::CString::new("VK_EXT_attachment_feedback_loop_dynamic_state").unwrap(),
            );
        }
        if value.ext_attachment_feedback_loop_layout {
            data.push(std::ffi::CString::new("VK_EXT_attachment_feedback_loop_layout").unwrap());
        }
        if value.ext_blend_operation_advanced {
            data.push(std::ffi::CString::new("VK_EXT_blend_operation_advanced").unwrap());
        }
        if value.ext_border_color_swizzle {
            data.push(std::ffi::CString::new("VK_EXT_border_color_swizzle").unwrap());
        }
        if value.ext_buffer_device_address {
            data.push(std::ffi::CString::new("VK_EXT_buffer_device_address").unwrap());
        }
        if value.ext_calibrated_timestamps {
            data.push(std::ffi::CString::new("VK_EXT_calibrated_timestamps").unwrap());
        }
        if value.ext_color_write_enable {
            data.push(std::ffi::CString::new("VK_EXT_color_write_enable").unwrap());
        }
        if value.ext_conditional_rendering {
            data.push(std::ffi::CString::new("VK_EXT_conditional_rendering").unwrap());
        }
        if value.ext_conservative_rasterization {
            data.push(std::ffi::CString::new("VK_EXT_conservative_rasterization").unwrap());
        }
        if value.ext_custom_border_color {
            data.push(std::ffi::CString::new("VK_EXT_custom_border_color").unwrap());
        }
        if value.ext_debug_marker {
            data.push(std::ffi::CString::new("VK_EXT_debug_marker").unwrap());
        }
        if value.ext_depth_clamp_zero_one {
            data.push(std::ffi::CString::new("VK_EXT_depth_clamp_zero_one").unwrap());
        }
        if value.ext_depth_clip_control {
            data.push(std::ffi::CString::new("VK_EXT_depth_clip_control").unwrap());
        }
        if value.ext_depth_clip_enable {
            data.push(std::ffi::CString::new("VK_EXT_depth_clip_enable").unwrap());
        }
        if value.ext_depth_range_unrestricted {
            data.push(std::ffi::CString::new("VK_EXT_depth_range_unrestricted").unwrap());
        }
        if value.ext_descriptor_buffer {
            data.push(std::ffi::CString::new("VK_EXT_descriptor_buffer").unwrap());
        }
        if value.ext_descriptor_indexing {
            data.push(std::ffi::CString::new("VK_EXT_descriptor_indexing").unwrap());
        }
        if value.ext_device_address_binding_report {
            data.push(std::ffi::CString::new("VK_EXT_device_address_binding_report").unwrap());
        }
        if value.ext_device_fault {
            data.push(std::ffi::CString::new("VK_EXT_device_fault").unwrap());
        }
        if value.ext_device_memory_report {
            data.push(std::ffi::CString::new("VK_EXT_device_memory_report").unwrap());
        }
        if value.ext_discard_rectangles {
            data.push(std::ffi::CString::new("VK_EXT_discard_rectangles").unwrap());
        }
        if value.ext_display_control {
            data.push(std::ffi::CString::new("VK_EXT_display_control").unwrap());
        }
        if value.ext_dynamic_rendering_unused_attachments {
            data.push(
                std::ffi::CString::new("VK_EXT_dynamic_rendering_unused_attachments").unwrap(),
            );
        }
        if value.ext_extended_dynamic_state {
            data.push(std::ffi::CString::new("VK_EXT_extended_dynamic_state").unwrap());
        }
        if value.ext_extended_dynamic_state2 {
            data.push(std::ffi::CString::new("VK_EXT_extended_dynamic_state2").unwrap());
        }
        if value.ext_extended_dynamic_state3 {
            data.push(std::ffi::CString::new("VK_EXT_extended_dynamic_state3").unwrap());
        }
        if value.ext_external_memory_dma_buf {
            data.push(std::ffi::CString::new("VK_EXT_external_memory_dma_buf").unwrap());
        }
        if value.ext_external_memory_host {
            data.push(std::ffi::CString::new("VK_EXT_external_memory_host").unwrap());
        }
        if value.ext_filter_cubic {
            data.push(std::ffi::CString::new("VK_EXT_filter_cubic").unwrap());
        }
        if value.ext_fragment_density_map {
            data.push(std::ffi::CString::new("VK_EXT_fragment_density_map").unwrap());
        }
        if value.ext_fragment_density_map2 {
            data.push(std::ffi::CString::new("VK_EXT_fragment_density_map2").unwrap());
        }
        if value.ext_fragment_shader_interlock {
            data.push(std::ffi::CString::new("VK_EXT_fragment_shader_interlock").unwrap());
        }
        if value.ext_full_screen_exclusive {
            data.push(std::ffi::CString::new("VK_EXT_full_screen_exclusive").unwrap());
        }
        if value.ext_global_priority {
            data.push(std::ffi::CString::new("VK_EXT_global_priority").unwrap());
        }
        if value.ext_global_priority_query {
            data.push(std::ffi::CString::new("VK_EXT_global_priority_query").unwrap());
        }
        if value.ext_graphics_pipeline_library {
            data.push(std::ffi::CString::new("VK_EXT_graphics_pipeline_library").unwrap());
        }
        if value.ext_hdr_metadata {
            data.push(std::ffi::CString::new("VK_EXT_hdr_metadata").unwrap());
        }
        if value.ext_host_query_reset {
            data.push(std::ffi::CString::new("VK_EXT_host_query_reset").unwrap());
        }
        if value.ext_image_2d_view_of_3d {
            data.push(std::ffi::CString::new("VK_EXT_image_2d_view_of_3d").unwrap());
        }
        if value.ext_image_compression_control {
            data.push(std::ffi::CString::new("VK_EXT_image_compression_control").unwrap());
        }
        if value.ext_image_compression_control_swapchain {
            data.push(
                std::ffi::CString::new("VK_EXT_image_compression_control_swapchain").unwrap(),
            );
        }
        if value.ext_image_drm_format_modifier {
            data.push(std::ffi::CString::new("VK_EXT_image_drm_format_modifier").unwrap());
        }
        if value.ext_image_robustness {
            data.push(std::ffi::CString::new("VK_EXT_image_robustness").unwrap());
        }
        if value.ext_image_sliced_view_of_3d {
            data.push(std::ffi::CString::new("VK_EXT_image_sliced_view_of_3d").unwrap());
        }
        if value.ext_image_view_min_lod {
            data.push(std::ffi::CString::new("VK_EXT_image_view_min_lod").unwrap());
        }
        if value.ext_index_type_uint8 {
            data.push(std::ffi::CString::new("VK_EXT_index_type_uint8").unwrap());
        }
        if value.ext_inline_uniform_block {
            data.push(std::ffi::CString::new("VK_EXT_inline_uniform_block").unwrap());
        }
        if value.ext_legacy_dithering {
            data.push(std::ffi::CString::new("VK_EXT_legacy_dithering").unwrap());
        }
        if value.ext_line_rasterization {
            data.push(std::ffi::CString::new("VK_EXT_line_rasterization").unwrap());
        }
        if value.ext_load_store_op_none {
            data.push(std::ffi::CString::new("VK_EXT_load_store_op_none").unwrap());
        }
        if value.ext_memory_budget {
            data.push(std::ffi::CString::new("VK_EXT_memory_budget").unwrap());
        }
        if value.ext_memory_priority {
            data.push(std::ffi::CString::new("VK_EXT_memory_priority").unwrap());
        }
        if value.ext_mesh_shader {
            data.push(std::ffi::CString::new("VK_EXT_mesh_shader").unwrap());
        }
        if value.ext_metal_objects {
            data.push(std::ffi::CString::new("VK_EXT_metal_objects").unwrap());
        }
        if value.ext_multi_draw {
            data.push(std::ffi::CString::new("VK_EXT_multi_draw").unwrap());
        }
        if value.ext_multisampled_render_to_single_sampled {
            data.push(
                std::ffi::CString::new("VK_EXT_multisampled_render_to_single_sampled").unwrap(),
            );
        }
        if value.ext_mutable_descriptor_type {
            data.push(std::ffi::CString::new("VK_EXT_mutable_descriptor_type").unwrap());
        }
        if value.ext_non_seamless_cube_map {
            data.push(std::ffi::CString::new("VK_EXT_non_seamless_cube_map").unwrap());
        }
        if value.ext_opacity_micromap {
            data.push(std::ffi::CString::new("VK_EXT_opacity_micromap").unwrap());
        }
        if value.ext_pageable_device_local_memory {
            data.push(std::ffi::CString::new("VK_EXT_pageable_device_local_memory").unwrap());
        }
        if value.ext_pci_bus_info {
            data.push(std::ffi::CString::new("VK_EXT_pci_bus_info").unwrap());
        }
        if value.ext_physical_device_drm {
            data.push(std::ffi::CString::new("VK_EXT_physical_device_drm").unwrap());
        }
        if value.ext_pipeline_creation_cache_control {
            data.push(std::ffi::CString::new("VK_EXT_pipeline_creation_cache_control").unwrap());
        }
        if value.ext_pipeline_creation_feedback {
            data.push(std::ffi::CString::new("VK_EXT_pipeline_creation_feedback").unwrap());
        }
        if value.ext_pipeline_library_group_handles {
            data.push(std::ffi::CString::new("VK_EXT_pipeline_library_group_handles").unwrap());
        }
        if value.ext_pipeline_properties {
            data.push(std::ffi::CString::new("VK_EXT_pipeline_properties").unwrap());
        }
        if value.ext_pipeline_protected_access {
            data.push(std::ffi::CString::new("VK_EXT_pipeline_protected_access").unwrap());
        }
        if value.ext_pipeline_robustness {
            data.push(std::ffi::CString::new("VK_EXT_pipeline_robustness").unwrap());
        }
        if value.ext_post_depth_coverage {
            data.push(std::ffi::CString::new("VK_EXT_post_depth_coverage").unwrap());
        }
        if value.ext_primitive_topology_list_restart {
            data.push(std::ffi::CString::new("VK_EXT_primitive_topology_list_restart").unwrap());
        }
        if value.ext_primitives_generated_query {
            data.push(std::ffi::CString::new("VK_EXT_primitives_generated_query").unwrap());
        }
        if value.ext_private_data {
            data.push(std::ffi::CString::new("VK_EXT_private_data").unwrap());
        }
        if value.ext_provoking_vertex {
            data.push(std::ffi::CString::new("VK_EXT_provoking_vertex").unwrap());
        }
        if value.ext_queue_family_foreign {
            data.push(std::ffi::CString::new("VK_EXT_queue_family_foreign").unwrap());
        }
        if value.ext_rasterization_order_attachment_access {
            data.push(
                std::ffi::CString::new("VK_EXT_rasterization_order_attachment_access").unwrap(),
            );
        }
        if value.ext_rgba10x6_formats {
            data.push(std::ffi::CString::new("VK_EXT_rgba10x6_formats").unwrap());
        }
        if value.ext_robustness2 {
            data.push(std::ffi::CString::new("VK_EXT_robustness2").unwrap());
        }
        if value.ext_sample_locations {
            data.push(std::ffi::CString::new("VK_EXT_sample_locations").unwrap());
        }
        if value.ext_sampler_filter_minmax {
            data.push(std::ffi::CString::new("VK_EXT_sampler_filter_minmax").unwrap());
        }
        if value.ext_scalar_block_layout {
            data.push(std::ffi::CString::new("VK_EXT_scalar_block_layout").unwrap());
        }
        if value.ext_separate_stencil_usage {
            data.push(std::ffi::CString::new("VK_EXT_separate_stencil_usage").unwrap());
        }
        if value.ext_shader_atomic_float {
            data.push(std::ffi::CString::new("VK_EXT_shader_atomic_float").unwrap());
        }
        if value.ext_shader_atomic_float2 {
            data.push(std::ffi::CString::new("VK_EXT_shader_atomic_float2").unwrap());
        }
        if value.ext_shader_demote_to_helper_invocation {
            data.push(std::ffi::CString::new("VK_EXT_shader_demote_to_helper_invocation").unwrap());
        }
        if value.ext_shader_image_atomic_int64 {
            data.push(std::ffi::CString::new("VK_EXT_shader_image_atomic_int64").unwrap());
        }
        if value.ext_shader_module_identifier {
            data.push(std::ffi::CString::new("VK_EXT_shader_module_identifier").unwrap());
        }
        if value.ext_shader_object {
            data.push(std::ffi::CString::new("VK_EXT_shader_object").unwrap());
        }
        if value.ext_shader_stencil_export {
            data.push(std::ffi::CString::new("VK_EXT_shader_stencil_export").unwrap());
        }
        if value.ext_shader_subgroup_ballot {
            data.push(std::ffi::CString::new("VK_EXT_shader_subgroup_ballot").unwrap());
        }
        if value.ext_shader_subgroup_vote {
            data.push(std::ffi::CString::new("VK_EXT_shader_subgroup_vote").unwrap());
        }
        if value.ext_shader_tile_image {
            data.push(std::ffi::CString::new("VK_EXT_shader_tile_image").unwrap());
        }
        if value.ext_shader_viewport_index_layer {
            data.push(std::ffi::CString::new("VK_EXT_shader_viewport_index_layer").unwrap());
        }
        if value.ext_subgroup_size_control {
            data.push(std::ffi::CString::new("VK_EXT_subgroup_size_control").unwrap());
        }
        if value.ext_subpass_merge_feedback {
            data.push(std::ffi::CString::new("VK_EXT_subpass_merge_feedback").unwrap());
        }
        if value.ext_swapchain_maintenance1 {
            data.push(std::ffi::CString::new("VK_EXT_swapchain_maintenance1").unwrap());
        }
        if value.ext_texel_buffer_alignment {
            data.push(std::ffi::CString::new("VK_EXT_texel_buffer_alignment").unwrap());
        }
        if value.ext_texture_compression_astc_hdr {
            data.push(std::ffi::CString::new("VK_EXT_texture_compression_astc_hdr").unwrap());
        }
        if value.ext_tooling_info {
            data.push(std::ffi::CString::new("VK_EXT_tooling_info").unwrap());
        }
        if value.ext_transform_feedback {
            data.push(std::ffi::CString::new("VK_EXT_transform_feedback").unwrap());
        }
        if value.ext_validation_cache {
            data.push(std::ffi::CString::new("VK_EXT_validation_cache").unwrap());
        }
        if value.ext_vertex_attribute_divisor {
            data.push(std::ffi::CString::new("VK_EXT_vertex_attribute_divisor").unwrap());
        }
        if value.ext_vertex_input_dynamic_state {
            data.push(std::ffi::CString::new("VK_EXT_vertex_input_dynamic_state").unwrap());
        }
        if value.ext_video_encode_h264 {
            data.push(std::ffi::CString::new("VK_EXT_video_encode_h264").unwrap());
        }
        if value.ext_video_encode_h265 {
            data.push(std::ffi::CString::new("VK_EXT_video_encode_h265").unwrap());
        }
        if value.ext_ycbcr_2plane_444_formats {
            data.push(std::ffi::CString::new("VK_EXT_ycbcr_2plane_444_formats").unwrap());
        }
        if value.ext_ycbcr_image_arrays {
            data.push(std::ffi::CString::new("VK_EXT_ycbcr_image_arrays").unwrap());
        }
        if value.ext_nested_command_buffer {
            data.push(std::ffi::CString::new("VK_EXT_nested_command_buffer").unwrap());
        }
        if value.amd_buffer_marker {
            data.push(std::ffi::CString::new("VK_AMD_buffer_marker").unwrap());
        }
        if value.amd_device_coherent_memory {
            data.push(std::ffi::CString::new("VK_AMD_device_coherent_memory").unwrap());
        }
        if value.amd_display_native_hdr {
            data.push(std::ffi::CString::new("VK_AMD_display_native_hdr").unwrap());
        }
        if value.amd_draw_indirect_count {
            data.push(std::ffi::CString::new("VK_AMD_draw_indirect_count").unwrap());
        }
        if value.amd_gcn_shader {
            data.push(std::ffi::CString::new("VK_AMD_gcn_shader").unwrap());
        }
        if value.amd_gpu_shader_half_float {
            data.push(std::ffi::CString::new("VK_AMD_gpu_shader_half_float").unwrap());
        }
        if value.amd_gpu_shader_int16 {
            data.push(std::ffi::CString::new("VK_AMD_gpu_shader_int16").unwrap());
        }
        if value.amd_memory_overallocation_behavior {
            data.push(std::ffi::CString::new("VK_AMD_memory_overallocation_behavior").unwrap());
        }
        if value.amd_mixed_attachment_samples {
            data.push(std::ffi::CString::new("VK_AMD_mixed_attachment_samples").unwrap());
        }
        if value.amd_pipeline_compiler_control {
            data.push(std::ffi::CString::new("VK_AMD_pipeline_compiler_control").unwrap());
        }
        if value.amd_rasterization_order {
            data.push(std::ffi::CString::new("VK_AMD_rasterization_order").unwrap());
        }
        if value.amd_shader_ballot {
            data.push(std::ffi::CString::new("VK_AMD_shader_ballot").unwrap());
        }
        if value.amd_shader_core_properties {
            data.push(std::ffi::CString::new("VK_AMD_shader_core_properties").unwrap());
        }
        if value.amd_shader_core_properties2 {
            data.push(std::ffi::CString::new("VK_AMD_shader_core_properties2").unwrap());
        }
        if value.amd_shader_early_and_late_fragment_tests {
            data.push(
                std::ffi::CString::new("VK_AMD_shader_early_and_late_fragment_tests").unwrap(),
            );
        }
        if value.amd_shader_explicit_vertex_parameter {
            data.push(std::ffi::CString::new("VK_AMD_shader_explicit_vertex_parameter").unwrap());
        }
        if value.amd_shader_fragment_mask {
            data.push(std::ffi::CString::new("VK_AMD_shader_fragment_mask").unwrap());
        }
        if value.amd_shader_image_load_store_lod {
            data.push(std::ffi::CString::new("VK_AMD_shader_image_load_store_lod").unwrap());
        }
        if value.amd_shader_info {
            data.push(std::ffi::CString::new("VK_AMD_shader_info").unwrap());
        }
        if value.amd_shader_trinary_minmax {
            data.push(std::ffi::CString::new("VK_AMD_shader_trinary_minmax").unwrap());
        }
        if value.amd_texture_gather_bias_lod {
            data.push(std::ffi::CString::new("VK_AMD_texture_gather_bias_lod").unwrap());
        }
        if value.android_external_memory_android_hardware_buffer {
            data.push(
                std::ffi::CString::new("VK_ANDROID_external_memory_android_hardware_buffer")
                    .unwrap(),
            );
        }
        if value.arm_rasterization_order_attachment_access {
            data.push(
                std::ffi::CString::new("VK_ARM_rasterization_order_attachment_access").unwrap(),
            );
        }
        if value.arm_shader_core_builtins {
            data.push(std::ffi::CString::new("VK_ARM_shader_core_builtins").unwrap());
        }
        if value.arm_shader_core_properties {
            data.push(std::ffi::CString::new("VK_ARM_shader_core_properties").unwrap());
        }
        if value.fuchsia_buffer_collection {
            data.push(std::ffi::CString::new("VK_FUCHSIA_buffer_collection").unwrap());
        }
        if value.fuchsia_external_memory {
            data.push(std::ffi::CString::new("VK_FUCHSIA_external_memory").unwrap());
        }
        if value.fuchsia_external_semaphore {
            data.push(std::ffi::CString::new("VK_FUCHSIA_external_semaphore").unwrap());
        }
        if value.ggp_frame_token {
            data.push(std::ffi::CString::new("VK_GGP_frame_token").unwrap());
        }
        if value.google_decorate_string {
            data.push(std::ffi::CString::new("VK_GOOGLE_decorate_string").unwrap());
        }
        if value.google_display_timing {
            data.push(std::ffi::CString::new("VK_GOOGLE_display_timing").unwrap());
        }
        if value.google_hlsl_functionality1 {
            data.push(std::ffi::CString::new("VK_GOOGLE_hlsl_functionality1").unwrap());
        }
        if value.google_user_type {
            data.push(std::ffi::CString::new("VK_GOOGLE_user_type").unwrap());
        }
        if value.huawei_cluster_culling_shader {
            data.push(std::ffi::CString::new("VK_HUAWEI_cluster_culling_shader").unwrap());
        }
        if value.huawei_invocation_mask {
            data.push(std::ffi::CString::new("VK_HUAWEI_invocation_mask").unwrap());
        }
        if value.huawei_subpass_shading {
            data.push(std::ffi::CString::new("VK_HUAWEI_subpass_shading").unwrap());
        }
        if value.img_filter_cubic {
            data.push(std::ffi::CString::new("VK_IMG_filter_cubic").unwrap());
        }
        if value.img_format_pvrtc {
            data.push(std::ffi::CString::new("VK_IMG_format_pvrtc").unwrap());
        }
        if value.intel_performance_query {
            data.push(std::ffi::CString::new("VK_INTEL_performance_query").unwrap());
        }
        if value.intel_shader_integer_functions2 {
            data.push(std::ffi::CString::new("VK_INTEL_shader_integer_functions2").unwrap());
        }
        if value.nvx_binary_import {
            data.push(std::ffi::CString::new("VK_NVX_binary_import").unwrap());
        }
        if value.nvx_image_view_handle {
            data.push(std::ffi::CString::new("VK_NVX_image_view_handle").unwrap());
        }
        if value.nvx_multiview_per_view_attributes {
            data.push(std::ffi::CString::new("VK_NVX_multiview_per_view_attributes").unwrap());
        }
        if value.nv_acquire_winrt_display {
            data.push(std::ffi::CString::new("VK_NV_acquire_winrt_display").unwrap());
        }
        if value.nv_clip_space_w_scaling {
            data.push(std::ffi::CString::new("VK_NV_clip_space_w_scaling").unwrap());
        }
        if value.nv_compute_shader_derivatives {
            data.push(std::ffi::CString::new("VK_NV_compute_shader_derivatives").unwrap());
        }
        if value.nv_cooperative_matrix {
            data.push(std::ffi::CString::new("VK_NV_cooperative_matrix").unwrap());
        }
        if value.nv_copy_memory_indirect {
            data.push(std::ffi::CString::new("VK_NV_copy_memory_indirect").unwrap());
        }
        if value.nv_corner_sampled_image {
            data.push(std::ffi::CString::new("VK_NV_corner_sampled_image").unwrap());
        }
        if value.nv_coverage_reduction_mode {
            data.push(std::ffi::CString::new("VK_NV_coverage_reduction_mode").unwrap());
        }
        if value.nv_dedicated_allocation {
            data.push(std::ffi::CString::new("VK_NV_dedicated_allocation").unwrap());
        }
        if value.nv_dedicated_allocation_image_aliasing {
            data.push(std::ffi::CString::new("VK_NV_dedicated_allocation_image_aliasing").unwrap());
        }
        if value.nv_device_diagnostic_checkpoints {
            data.push(std::ffi::CString::new("VK_NV_device_diagnostic_checkpoints").unwrap());
        }
        if value.nv_device_diagnostics_config {
            data.push(std::ffi::CString::new("VK_NV_device_diagnostics_config").unwrap());
        }
        if value.nv_device_generated_commands {
            data.push(std::ffi::CString::new("VK_NV_device_generated_commands").unwrap());
        }
        if value.nv_displacement_micromap {
            data.push(std::ffi::CString::new("VK_NV_displacement_micromap").unwrap());
        }
        if value.nv_external_memory {
            data.push(std::ffi::CString::new("VK_NV_external_memory").unwrap());
        }
        if value.nv_external_memory_rdma {
            data.push(std::ffi::CString::new("VK_NV_external_memory_rdma").unwrap());
        }
        if value.nv_external_memory_win32 {
            data.push(std::ffi::CString::new("VK_NV_external_memory_win32").unwrap());
        }
        if value.nv_fill_rectangle {
            data.push(std::ffi::CString::new("VK_NV_fill_rectangle").unwrap());
        }
        if value.nv_fragment_coverage_to_color {
            data.push(std::ffi::CString::new("VK_NV_fragment_coverage_to_color").unwrap());
        }
        if value.nv_fragment_shader_barycentric {
            data.push(std::ffi::CString::new("VK_NV_fragment_shader_barycentric").unwrap());
        }
        if value.nv_fragment_shading_rate_enums {
            data.push(std::ffi::CString::new("VK_NV_fragment_shading_rate_enums").unwrap());
        }
        if value.nv_framebuffer_mixed_samples {
            data.push(std::ffi::CString::new("VK_NV_framebuffer_mixed_samples").unwrap());
        }
        if value.nv_geometry_shader_passthrough {
            data.push(std::ffi::CString::new("VK_NV_geometry_shader_passthrough").unwrap());
        }
        if value.nv_glsl_shader {
            data.push(std::ffi::CString::new("VK_NV_glsl_shader").unwrap());
        }
        if value.nv_inherited_viewport_scissor {
            data.push(std::ffi::CString::new("VK_NV_inherited_viewport_scissor").unwrap());
        }
        if value.nv_linear_color_attachment {
            data.push(std::ffi::CString::new("VK_NV_linear_color_attachment").unwrap());
        }
        if value.nv_low_latency {
            data.push(std::ffi::CString::new("VK_NV_low_latency").unwrap());
        }
        if value.nv_memory_decompression {
            data.push(std::ffi::CString::new("VK_NV_memory_decompression").unwrap());
        }
        if value.nv_mesh_shader {
            data.push(std::ffi::CString::new("VK_NV_mesh_shader").unwrap());
        }
        if value.nv_optical_flow {
            data.push(std::ffi::CString::new("VK_NV_optical_flow").unwrap());
        }
        if value.nv_present_barrier {
            data.push(std::ffi::CString::new("VK_NV_present_barrier").unwrap());
        }
        if value.nv_ray_tracing {
            data.push(std::ffi::CString::new("VK_NV_ray_tracing").unwrap());
        }
        if value.nv_ray_tracing_invocation_reorder {
            data.push(std::ffi::CString::new("VK_NV_ray_tracing_invocation_reorder").unwrap());
        }
        if value.nv_ray_tracing_motion_blur {
            data.push(std::ffi::CString::new("VK_NV_ray_tracing_motion_blur").unwrap());
        }
        if value.nv_representative_fragment_test {
            data.push(std::ffi::CString::new("VK_NV_representative_fragment_test").unwrap());
        }
        if value.nv_sample_mask_override_coverage {
            data.push(std::ffi::CString::new("VK_NV_sample_mask_override_coverage").unwrap());
        }
        if value.nv_scissor_exclusive {
            data.push(std::ffi::CString::new("VK_NV_scissor_exclusive").unwrap());
        }
        if value.nv_shader_image_footprint {
            data.push(std::ffi::CString::new("VK_NV_shader_image_footprint").unwrap());
        }
        if value.nv_shader_sm_builtins {
            data.push(std::ffi::CString::new("VK_NV_shader_sm_builtins").unwrap());
        }
        if value.nv_shader_subgroup_partitioned {
            data.push(std::ffi::CString::new("VK_NV_shader_subgroup_partitioned").unwrap());
        }
        if value.nv_shading_rate_image {
            data.push(std::ffi::CString::new("VK_NV_shading_rate_image").unwrap());
        }
        if value.nv_viewport_array2 {
            data.push(std::ffi::CString::new("VK_NV_viewport_array2").unwrap());
        }
        if value.nv_viewport_swizzle {
            data.push(std::ffi::CString::new("VK_NV_viewport_swizzle").unwrap());
        }
        if value.nv_win32_keyed_mutex {
            data.push(std::ffi::CString::new("VK_NV_win32_keyed_mutex").unwrap());
        }
        if value.qcom_fragment_density_map_offset {
            data.push(std::ffi::CString::new("VK_QCOM_fragment_density_map_offset").unwrap());
        }
        if value.qcom_image_processing {
            data.push(std::ffi::CString::new("VK_QCOM_image_processing").unwrap());
        }
        if value.qcom_multiview_per_view_render_areas {
            data.push(std::ffi::CString::new("VK_QCOM_multiview_per_view_render_areas").unwrap());
        }
        if value.qcom_multiview_per_view_viewports {
            data.push(std::ffi::CString::new("VK_QCOM_multiview_per_view_viewports").unwrap());
        }
        if value.qcom_render_pass_shader_resolve {
            data.push(std::ffi::CString::new("VK_QCOM_render_pass_shader_resolve").unwrap());
        }
        if value.qcom_render_pass_store_ops {
            data.push(std::ffi::CString::new("VK_QCOM_render_pass_store_ops").unwrap());
        }
        if value.qcom_render_pass_transform {
            data.push(std::ffi::CString::new("VK_QCOM_render_pass_transform").unwrap());
        }
        if value.qcom_rotated_copy_commands {
            data.push(std::ffi::CString::new("VK_QCOM_rotated_copy_commands").unwrap());
        }
        if value.qcom_tile_properties {
            data.push(std::ffi::CString::new("VK_QCOM_tile_properties").unwrap());
        }
        if value.sec_amigo_profiling {
            data.push(std::ffi::CString::new("VK_SEC_amigo_profiling").unwrap());
        }
        if value.valve_descriptor_set_host_mapping {
            data.push(std::ffi::CString::new("VK_VALVE_descriptor_set_host_mapping").unwrap());
        }
        if value.valve_mutable_descriptor_type {
            data.push(std::ffi::CString::new("VK_VALVE_mutable_descriptor_type").unwrap());
        }
        data
    }
}

impl<'a> FromIterator<&'a str> for DeviceExtensions {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut extensions = Self::default();
        for name in iter {
            match name {
                "VK_KHR_16bit_storage" => {
                    extensions.khr_16bit_storage = true;
                }
                "VK_KHR_8bit_storage" => {
                    extensions.khr_8bit_storage = true;
                }
                "VK_KHR_acceleration_structure" => {
                    extensions.khr_acceleration_structure = true;
                }
                "VK_KHR_bind_memory2" => {
                    extensions.khr_bind_memory2 = true;
                }
                "VK_KHR_buffer_device_address" => {
                    extensions.khr_buffer_device_address = true;
                }
                "VK_KHR_copy_commands2" => {
                    extensions.khr_copy_commands2 = true;
                }
                "VK_KHR_create_renderpass2" => {
                    extensions.khr_create_renderpass2 = true;
                }
                "VK_KHR_dedicated_allocation" => {
                    extensions.khr_dedicated_allocation = true;
                }
                "VK_KHR_deferred_host_operations" => {
                    extensions.khr_deferred_host_operations = true;
                }
                "VK_KHR_depth_stencil_resolve" => {
                    extensions.khr_depth_stencil_resolve = true;
                }
                "VK_KHR_descriptor_update_template" => {
                    extensions.khr_descriptor_update_template = true;
                }
                "VK_KHR_device_group" => {
                    extensions.khr_device_group = true;
                }
                "VK_KHR_display_swapchain" => {
                    extensions.khr_display_swapchain = true;
                }
                "VK_KHR_draw_indirect_count" => {
                    extensions.khr_draw_indirect_count = true;
                }
                "VK_KHR_driver_properties" => {
                    extensions.khr_driver_properties = true;
                }
                "VK_KHR_dynamic_rendering" => {
                    extensions.khr_dynamic_rendering = true;
                }
                "VK_KHR_external_fence" => {
                    extensions.khr_external_fence = true;
                }
                "VK_KHR_external_fence_fd" => {
                    extensions.khr_external_fence_fd = true;
                }
                "VK_KHR_external_fence_win32" => {
                    extensions.khr_external_fence_win32 = true;
                }
                "VK_KHR_external_memory" => {
                    extensions.khr_external_memory = true;
                }
                "VK_KHR_external_memory_fd" => {
                    extensions.khr_external_memory_fd = true;
                }
                "VK_KHR_external_memory_win32" => {
                    extensions.khr_external_memory_win32 = true;
                }
                "VK_KHR_external_semaphore" => {
                    extensions.khr_external_semaphore = true;
                }
                "VK_KHR_external_semaphore_fd" => {
                    extensions.khr_external_semaphore_fd = true;
                }
                "VK_KHR_external_semaphore_win32" => {
                    extensions.khr_external_semaphore_win32 = true;
                }
                "VK_KHR_format_feature_flags2" => {
                    extensions.khr_format_feature_flags2 = true;
                }
                "VK_KHR_fragment_shader_barycentric" => {
                    extensions.khr_fragment_shader_barycentric = true;
                }
                "VK_KHR_fragment_shading_rate" => {
                    extensions.khr_fragment_shading_rate = true;
                }
                "VK_KHR_get_memory_requirements2" => {
                    extensions.khr_get_memory_requirements2 = true;
                }
                "VK_KHR_global_priority" => {
                    extensions.khr_global_priority = true;
                }
                "VK_KHR_image_format_list" => {
                    extensions.khr_image_format_list = true;
                }
                "VK_KHR_imageless_framebuffer" => {
                    extensions.khr_imageless_framebuffer = true;
                }
                "VK_KHR_incremental_present" => {
                    extensions.khr_incremental_present = true;
                }
                "VK_KHR_maintenance1" => {
                    extensions.khr_maintenance1 = true;
                }
                "VK_KHR_maintenance2" => {
                    extensions.khr_maintenance2 = true;
                }
                "VK_KHR_maintenance3" => {
                    extensions.khr_maintenance3 = true;
                }
                "VK_KHR_maintenance4" => {
                    extensions.khr_maintenance4 = true;
                }
                "VK_KHR_map_memory2" => {
                    extensions.khr_map_memory2 = true;
                }
                "VK_KHR_multiview" => {
                    extensions.khr_multiview = true;
                }
                "VK_KHR_performance_query" => {
                    extensions.khr_performance_query = true;
                }
                "VK_KHR_pipeline_executable_properties" => {
                    extensions.khr_pipeline_executable_properties = true;
                }
                "VK_KHR_pipeline_library" => {
                    extensions.khr_pipeline_library = true;
                }
                "VK_KHR_portability_subset" => {
                    extensions.khr_portability_subset = true;
                }
                "VK_KHR_present_id" => {
                    extensions.khr_present_id = true;
                }
                "VK_KHR_present_wait" => {
                    extensions.khr_present_wait = true;
                }
                "VK_KHR_push_descriptor" => {
                    extensions.khr_push_descriptor = true;
                }
                "VK_KHR_ray_query" => {
                    extensions.khr_ray_query = true;
                }
                "VK_KHR_ray_tracing_maintenance1" => {
                    extensions.khr_ray_tracing_maintenance1 = true;
                }
                "VK_KHR_ray_tracing_pipeline" => {
                    extensions.khr_ray_tracing_pipeline = true;
                }
                "VK_KHR_ray_tracing_position_fetch" => {
                    extensions.khr_ray_tracing_position_fetch = true;
                }
                "VK_KHR_relaxed_block_layout" => {
                    extensions.khr_relaxed_block_layout = true;
                }
                "VK_KHR_sampler_mirror_clamp_to_edge" => {
                    extensions.khr_sampler_mirror_clamp_to_edge = true;
                }
                "VK_KHR_sampler_ycbcr_conversion" => {
                    extensions.khr_sampler_ycbcr_conversion = true;
                }
                "VK_KHR_separate_depth_stencil_layouts" => {
                    extensions.khr_separate_depth_stencil_layouts = true;
                }
                "VK_KHR_shader_atomic_int64" => {
                    extensions.khr_shader_atomic_int64 = true;
                }
                "VK_KHR_shader_clock" => {
                    extensions.khr_shader_clock = true;
                }
                "VK_KHR_shader_draw_parameters" => {
                    extensions.khr_shader_draw_parameters = true;
                }
                "VK_KHR_shader_float16_int8" => {
                    extensions.khr_shader_float16_int8 = true;
                }
                "VK_KHR_shader_float_controls" => {
                    extensions.khr_shader_float_controls = true;
                }
                "VK_KHR_shader_integer_dot_product" => {
                    extensions.khr_shader_integer_dot_product = true;
                }
                "VK_KHR_shader_non_semantic_info" => {
                    extensions.khr_shader_non_semantic_info = true;
                }
                "VK_KHR_shader_subgroup_extended_types" => {
                    extensions.khr_shader_subgroup_extended_types = true;
                }
                "VK_KHR_shader_subgroup_uniform_control_flow" => {
                    extensions.khr_shader_subgroup_uniform_control_flow = true;
                }
                "VK_KHR_shader_terminate_invocation" => {
                    extensions.khr_shader_terminate_invocation = true;
                }
                "VK_KHR_shared_presentable_image" => {
                    extensions.khr_shared_presentable_image = true;
                }
                "VK_KHR_spirv_1_4" => {
                    extensions.khr_spirv_1_4 = true;
                }
                "VK_KHR_storage_buffer_storage_class" => {
                    extensions.khr_storage_buffer_storage_class = true;
                }
                "VK_KHR_swapchain" => {
                    extensions.khr_swapchain = true;
                }
                "VK_KHR_swapchain_mutable_format" => {
                    extensions.khr_swapchain_mutable_format = true;
                }
                "VK_KHR_synchronization2" => {
                    extensions.khr_synchronization2 = true;
                }
                "VK_KHR_timeline_semaphore" => {
                    extensions.khr_timeline_semaphore = true;
                }
                "VK_KHR_uniform_buffer_standard_layout" => {
                    extensions.khr_uniform_buffer_standard_layout = true;
                }
                "VK_KHR_variable_pointers" => {
                    extensions.khr_variable_pointers = true;
                }
                "VK_KHR_video_decode_h264" => {
                    extensions.khr_video_decode_h264 = true;
                }
                "VK_KHR_video_decode_h265" => {
                    extensions.khr_video_decode_h265 = true;
                }
                "VK_KHR_video_decode_queue" => {
                    extensions.khr_video_decode_queue = true;
                }
                "VK_KHR_video_encode_queue" => {
                    extensions.khr_video_encode_queue = true;
                }
                "VK_KHR_video_queue" => {
                    extensions.khr_video_queue = true;
                }
                "VK_KHR_vulkan_memory_model" => {
                    extensions.khr_vulkan_memory_model = true;
                }
                "VK_KHR_win32_keyed_mutex" => {
                    extensions.khr_win32_keyed_mutex = true;
                }
                "VK_KHR_workgroup_memory_explicit_layout" => {
                    extensions.khr_workgroup_memory_explicit_layout = true;
                }
                "VK_KHR_zero_initialize_workgroup_memory" => {
                    extensions.khr_zero_initialize_workgroup_memory = true;
                }
                "VK_EXT_4444_formats" => {
                    extensions.ext_4444_formats = true;
                }
                "VK_EXT_astc_decode_mode" => {
                    extensions.ext_astc_decode_mode = true;
                }
                "VK_EXT_attachment_feedback_loop_dynamic_state" => {
                    extensions.ext_attachment_feedback_loop_dynamic_state = true;
                }
                "VK_EXT_attachment_feedback_loop_layout" => {
                    extensions.ext_attachment_feedback_loop_layout = true;
                }
                "VK_EXT_blend_operation_advanced" => {
                    extensions.ext_blend_operation_advanced = true;
                }
                "VK_EXT_border_color_swizzle" => {
                    extensions.ext_border_color_swizzle = true;
                }
                "VK_EXT_buffer_device_address" => {
                    extensions.ext_buffer_device_address = true;
                }
                "VK_EXT_calibrated_timestamps" => {
                    extensions.ext_calibrated_timestamps = true;
                }
                "VK_EXT_color_write_enable" => {
                    extensions.ext_color_write_enable = true;
                }
                "VK_EXT_conditional_rendering" => {
                    extensions.ext_conditional_rendering = true;
                }
                "VK_EXT_conservative_rasterization" => {
                    extensions.ext_conservative_rasterization = true;
                }
                "VK_EXT_custom_border_color" => {
                    extensions.ext_custom_border_color = true;
                }
                "VK_EXT_debug_marker" => {
                    extensions.ext_debug_marker = true;
                }
                "VK_EXT_depth_clamp_zero_one" => {
                    extensions.ext_depth_clamp_zero_one = true;
                }
                "VK_EXT_depth_clip_control" => {
                    extensions.ext_depth_clip_control = true;
                }
                "VK_EXT_depth_clip_enable" => {
                    extensions.ext_depth_clip_enable = true;
                }
                "VK_EXT_depth_range_unrestricted" => {
                    extensions.ext_depth_range_unrestricted = true;
                }
                "VK_EXT_descriptor_buffer" => {
                    extensions.ext_descriptor_buffer = true;
                }
                "VK_EXT_descriptor_indexing" => {
                    extensions.ext_descriptor_indexing = true;
                }
                "VK_EXT_device_address_binding_report" => {
                    extensions.ext_device_address_binding_report = true;
                }
                "VK_EXT_device_fault" => {
                    extensions.ext_device_fault = true;
                }
                "VK_EXT_device_memory_report" => {
                    extensions.ext_device_memory_report = true;
                }
                "VK_EXT_discard_rectangles" => {
                    extensions.ext_discard_rectangles = true;
                }
                "VK_EXT_display_control" => {
                    extensions.ext_display_control = true;
                }
                "VK_EXT_dynamic_rendering_unused_attachments" => {
                    extensions.ext_dynamic_rendering_unused_attachments = true;
                }
                "VK_EXT_extended_dynamic_state" => {
                    extensions.ext_extended_dynamic_state = true;
                }
                "VK_EXT_extended_dynamic_state2" => {
                    extensions.ext_extended_dynamic_state2 = true;
                }
                "VK_EXT_extended_dynamic_state3" => {
                    extensions.ext_extended_dynamic_state3 = true;
                }
                "VK_EXT_external_memory_dma_buf" => {
                    extensions.ext_external_memory_dma_buf = true;
                }
                "VK_EXT_external_memory_host" => {
                    extensions.ext_external_memory_host = true;
                }
                "VK_EXT_filter_cubic" => {
                    extensions.ext_filter_cubic = true;
                }
                "VK_EXT_fragment_density_map" => {
                    extensions.ext_fragment_density_map = true;
                }
                "VK_EXT_fragment_density_map2" => {
                    extensions.ext_fragment_density_map2 = true;
                }
                "VK_EXT_fragment_shader_interlock" => {
                    extensions.ext_fragment_shader_interlock = true;
                }
                "VK_EXT_full_screen_exclusive" => {
                    extensions.ext_full_screen_exclusive = true;
                }
                "VK_EXT_global_priority" => {
                    extensions.ext_global_priority = true;
                }
                "VK_EXT_global_priority_query" => {
                    extensions.ext_global_priority_query = true;
                }
                "VK_EXT_graphics_pipeline_library" => {
                    extensions.ext_graphics_pipeline_library = true;
                }
                "VK_EXT_hdr_metadata" => {
                    extensions.ext_hdr_metadata = true;
                }
                "VK_EXT_host_query_reset" => {
                    extensions.ext_host_query_reset = true;
                }
                "VK_EXT_image_2d_view_of_3d" => {
                    extensions.ext_image_2d_view_of_3d = true;
                }
                "VK_EXT_image_compression_control" => {
                    extensions.ext_image_compression_control = true;
                }
                "VK_EXT_image_compression_control_swapchain" => {
                    extensions.ext_image_compression_control_swapchain = true;
                }
                "VK_EXT_image_drm_format_modifier" => {
                    extensions.ext_image_drm_format_modifier = true;
                }
                "VK_EXT_image_robustness" => {
                    extensions.ext_image_robustness = true;
                }
                "VK_EXT_image_sliced_view_of_3d" => {
                    extensions.ext_image_sliced_view_of_3d = true;
                }
                "VK_EXT_image_view_min_lod" => {
                    extensions.ext_image_view_min_lod = true;
                }
                "VK_EXT_index_type_uint8" => {
                    extensions.ext_index_type_uint8 = true;
                }
                "VK_EXT_inline_uniform_block" => {
                    extensions.ext_inline_uniform_block = true;
                }
                "VK_EXT_legacy_dithering" => {
                    extensions.ext_legacy_dithering = true;
                }
                "VK_EXT_line_rasterization" => {
                    extensions.ext_line_rasterization = true;
                }
                "VK_EXT_load_store_op_none" => {
                    extensions.ext_load_store_op_none = true;
                }
                "VK_EXT_memory_budget" => {
                    extensions.ext_memory_budget = true;
                }
                "VK_EXT_memory_priority" => {
                    extensions.ext_memory_priority = true;
                }
                "VK_EXT_mesh_shader" => {
                    extensions.ext_mesh_shader = true;
                }
                "VK_EXT_metal_objects" => {
                    extensions.ext_metal_objects = true;
                }
                "VK_EXT_multi_draw" => {
                    extensions.ext_multi_draw = true;
                }
                "VK_EXT_multisampled_render_to_single_sampled" => {
                    extensions.ext_multisampled_render_to_single_sampled = true;
                }
                "VK_EXT_mutable_descriptor_type" => {
                    extensions.ext_mutable_descriptor_type = true;
                }
                "VK_EXT_non_seamless_cube_map" => {
                    extensions.ext_non_seamless_cube_map = true;
                }
                "VK_EXT_opacity_micromap" => {
                    extensions.ext_opacity_micromap = true;
                }
                "VK_EXT_pageable_device_local_memory" => {
                    extensions.ext_pageable_device_local_memory = true;
                }
                "VK_EXT_pci_bus_info" => {
                    extensions.ext_pci_bus_info = true;
                }
                "VK_EXT_physical_device_drm" => {
                    extensions.ext_physical_device_drm = true;
                }
                "VK_EXT_pipeline_creation_cache_control" => {
                    extensions.ext_pipeline_creation_cache_control = true;
                }
                "VK_EXT_pipeline_creation_feedback" => {
                    extensions.ext_pipeline_creation_feedback = true;
                }
                "VK_EXT_pipeline_library_group_handles" => {
                    extensions.ext_pipeline_library_group_handles = true;
                }
                "VK_EXT_pipeline_properties" => {
                    extensions.ext_pipeline_properties = true;
                }
                "VK_EXT_pipeline_protected_access" => {
                    extensions.ext_pipeline_protected_access = true;
                }
                "VK_EXT_pipeline_robustness" => {
                    extensions.ext_pipeline_robustness = true;
                }
                "VK_EXT_post_depth_coverage" => {
                    extensions.ext_post_depth_coverage = true;
                }
                "VK_EXT_primitive_topology_list_restart" => {
                    extensions.ext_primitive_topology_list_restart = true;
                }
                "VK_EXT_primitives_generated_query" => {
                    extensions.ext_primitives_generated_query = true;
                }
                "VK_EXT_private_data" => {
                    extensions.ext_private_data = true;
                }
                "VK_EXT_provoking_vertex" => {
                    extensions.ext_provoking_vertex = true;
                }
                "VK_EXT_queue_family_foreign" => {
                    extensions.ext_queue_family_foreign = true;
                }
                "VK_EXT_rasterization_order_attachment_access" => {
                    extensions.ext_rasterization_order_attachment_access = true;
                }
                "VK_EXT_rgba10x6_formats" => {
                    extensions.ext_rgba10x6_formats = true;
                }
                "VK_EXT_robustness2" => {
                    extensions.ext_robustness2 = true;
                }
                "VK_EXT_sample_locations" => {
                    extensions.ext_sample_locations = true;
                }
                "VK_EXT_sampler_filter_minmax" => {
                    extensions.ext_sampler_filter_minmax = true;
                }
                "VK_EXT_scalar_block_layout" => {
                    extensions.ext_scalar_block_layout = true;
                }
                "VK_EXT_separate_stencil_usage" => {
                    extensions.ext_separate_stencil_usage = true;
                }
                "VK_EXT_shader_atomic_float" => {
                    extensions.ext_shader_atomic_float = true;
                }
                "VK_EXT_shader_atomic_float2" => {
                    extensions.ext_shader_atomic_float2 = true;
                }
                "VK_EXT_shader_demote_to_helper_invocation" => {
                    extensions.ext_shader_demote_to_helper_invocation = true;
                }
                "VK_EXT_shader_image_atomic_int64" => {
                    extensions.ext_shader_image_atomic_int64 = true;
                }
                "VK_EXT_shader_module_identifier" => {
                    extensions.ext_shader_module_identifier = true;
                }
                "VK_EXT_shader_object" => {
                    extensions.ext_shader_object = true;
                }
                "VK_EXT_shader_stencil_export" => {
                    extensions.ext_shader_stencil_export = true;
                }
                "VK_EXT_shader_subgroup_ballot" => {
                    extensions.ext_shader_subgroup_ballot = true;
                }
                "VK_EXT_shader_subgroup_vote" => {
                    extensions.ext_shader_subgroup_vote = true;
                }
                "VK_EXT_shader_tile_image" => {
                    extensions.ext_shader_tile_image = true;
                }
                "VK_EXT_shader_viewport_index_layer" => {
                    extensions.ext_shader_viewport_index_layer = true;
                }
                "VK_EXT_subgroup_size_control" => {
                    extensions.ext_subgroup_size_control = true;
                }
                "VK_EXT_subpass_merge_feedback" => {
                    extensions.ext_subpass_merge_feedback = true;
                }
                "VK_EXT_swapchain_maintenance1" => {
                    extensions.ext_swapchain_maintenance1 = true;
                }
                "VK_EXT_texel_buffer_alignment" => {
                    extensions.ext_texel_buffer_alignment = true;
                }
                "VK_EXT_texture_compression_astc_hdr" => {
                    extensions.ext_texture_compression_astc_hdr = true;
                }
                "VK_EXT_tooling_info" => {
                    extensions.ext_tooling_info = true;
                }
                "VK_EXT_transform_feedback" => {
                    extensions.ext_transform_feedback = true;
                }
                "VK_EXT_validation_cache" => {
                    extensions.ext_validation_cache = true;
                }
                "VK_EXT_vertex_attribute_divisor" => {
                    extensions.ext_vertex_attribute_divisor = true;
                }
                "VK_EXT_vertex_input_dynamic_state" => {
                    extensions.ext_vertex_input_dynamic_state = true;
                }
                "VK_EXT_video_encode_h264" => {
                    extensions.ext_video_encode_h264 = true;
                }
                "VK_EXT_video_encode_h265" => {
                    extensions.ext_video_encode_h265 = true;
                }
                "VK_EXT_ycbcr_2plane_444_formats" => {
                    extensions.ext_ycbcr_2plane_444_formats = true;
                }
                "VK_EXT_ycbcr_image_arrays" => {
                    extensions.ext_ycbcr_image_arrays = true;
                }
                "VK_EXT_nested_command_buffer " => {
                    extensions.ext_nested_command_buffer = true;
                }
                "VK_AMD_buffer_marker" => {
                    extensions.amd_buffer_marker = true;
                }
                "VK_AMD_device_coherent_memory" => {
                    extensions.amd_device_coherent_memory = true;
                }
                "VK_AMD_display_native_hdr" => {
                    extensions.amd_display_native_hdr = true;
                }
                "VK_AMD_draw_indirect_count" => {
                    extensions.amd_draw_indirect_count = true;
                }
                "VK_AMD_gcn_shader" => {
                    extensions.amd_gcn_shader = true;
                }
                "VK_AMD_gpu_shader_half_float" => {
                    extensions.amd_gpu_shader_half_float = true;
                }
                "VK_AMD_gpu_shader_int16" => {
                    extensions.amd_gpu_shader_int16 = true;
                }
                "VK_AMD_memory_overallocation_behavior" => {
                    extensions.amd_memory_overallocation_behavior = true;
                }
                "VK_AMD_mixed_attachment_samples" => {
                    extensions.amd_mixed_attachment_samples = true;
                }
                "VK_AMD_pipeline_compiler_control" => {
                    extensions.amd_pipeline_compiler_control = true;
                }
                "VK_AMD_rasterization_order" => {
                    extensions.amd_rasterization_order = true;
                }
                "VK_AMD_shader_ballot" => {
                    extensions.amd_shader_ballot = true;
                }
                "VK_AMD_shader_core_properties" => {
                    extensions.amd_shader_core_properties = true;
                }
                "VK_AMD_shader_core_properties2" => {
                    extensions.amd_shader_core_properties2 = true;
                }
                "VK_AMD_shader_early_and_late_fragment_tests" => {
                    extensions.amd_shader_early_and_late_fragment_tests = true;
                }
                "VK_AMD_shader_explicit_vertex_parameter" => {
                    extensions.amd_shader_explicit_vertex_parameter = true;
                }
                "VK_AMD_shader_fragment_mask" => {
                    extensions.amd_shader_fragment_mask = true;
                }
                "VK_AMD_shader_image_load_store_lod" => {
                    extensions.amd_shader_image_load_store_lod = true;
                }
                "VK_AMD_shader_info" => {
                    extensions.amd_shader_info = true;
                }
                "VK_AMD_shader_trinary_minmax" => {
                    extensions.amd_shader_trinary_minmax = true;
                }
                "VK_AMD_texture_gather_bias_lod" => {
                    extensions.amd_texture_gather_bias_lod = true;
                }
                "VK_ANDROID_external_memory_android_hardware_buffer" => {
                    extensions.android_external_memory_android_hardware_buffer = true;
                }
                "VK_ARM_rasterization_order_attachment_access" => {
                    extensions.arm_rasterization_order_attachment_access = true;
                }
                "VK_ARM_shader_core_builtins" => {
                    extensions.arm_shader_core_builtins = true;
                }
                "VK_ARM_shader_core_properties" => {
                    extensions.arm_shader_core_properties = true;
                }
                "VK_FUCHSIA_buffer_collection" => {
                    extensions.fuchsia_buffer_collection = true;
                }
                "VK_FUCHSIA_external_memory" => {
                    extensions.fuchsia_external_memory = true;
                }
                "VK_FUCHSIA_external_semaphore" => {
                    extensions.fuchsia_external_semaphore = true;
                }
                "VK_GGP_frame_token" => {
                    extensions.ggp_frame_token = true;
                }
                "VK_GOOGLE_decorate_string" => {
                    extensions.google_decorate_string = true;
                }
                "VK_GOOGLE_display_timing" => {
                    extensions.google_display_timing = true;
                }
                "VK_GOOGLE_hlsl_functionality1" => {
                    extensions.google_hlsl_functionality1 = true;
                }
                "VK_GOOGLE_user_type" => {
                    extensions.google_user_type = true;
                }
                "VK_HUAWEI_cluster_culling_shader" => {
                    extensions.huawei_cluster_culling_shader = true;
                }
                "VK_HUAWEI_invocation_mask" => {
                    extensions.huawei_invocation_mask = true;
                }
                "VK_HUAWEI_subpass_shading" => {
                    extensions.huawei_subpass_shading = true;
                }
                "VK_IMG_filter_cubic" => {
                    extensions.img_filter_cubic = true;
                }
                "VK_IMG_format_pvrtc" => {
                    extensions.img_format_pvrtc = true;
                }
                "VK_INTEL_performance_query" => {
                    extensions.intel_performance_query = true;
                }
                "VK_INTEL_shader_integer_functions2" => {
                    extensions.intel_shader_integer_functions2 = true;
                }
                "VK_NVX_binary_import" => {
                    extensions.nvx_binary_import = true;
                }
                "VK_NVX_image_view_handle" => {
                    extensions.nvx_image_view_handle = true;
                }
                "VK_NVX_multiview_per_view_attributes" => {
                    extensions.nvx_multiview_per_view_attributes = true;
                }
                "VK_NV_acquire_winrt_display" => {
                    extensions.nv_acquire_winrt_display = true;
                }
                "VK_NV_clip_space_w_scaling" => {
                    extensions.nv_clip_space_w_scaling = true;
                }
                "VK_NV_compute_shader_derivatives" => {
                    extensions.nv_compute_shader_derivatives = true;
                }
                "VK_NV_cooperative_matrix" => {
                    extensions.nv_cooperative_matrix = true;
                }
                "VK_NV_copy_memory_indirect" => {
                    extensions.nv_copy_memory_indirect = true;
                }
                "VK_NV_corner_sampled_image" => {
                    extensions.nv_corner_sampled_image = true;
                }
                "VK_NV_coverage_reduction_mode" => {
                    extensions.nv_coverage_reduction_mode = true;
                }
                "VK_NV_dedicated_allocation" => {
                    extensions.nv_dedicated_allocation = true;
                }
                "VK_NV_dedicated_allocation_image_aliasing" => {
                    extensions.nv_dedicated_allocation_image_aliasing = true;
                }
                "VK_NV_device_diagnostic_checkpoints" => {
                    extensions.nv_device_diagnostic_checkpoints = true;
                }
                "VK_NV_device_diagnostics_config" => {
                    extensions.nv_device_diagnostics_config = true;
                }
                "VK_NV_device_generated_commands" => {
                    extensions.nv_device_generated_commands = true;
                }
                "VK_NV_displacement_micromap" => {
                    extensions.nv_displacement_micromap = true;
                }
                "VK_NV_external_memory" => {
                    extensions.nv_external_memory = true;
                }
                "VK_NV_external_memory_rdma" => {
                    extensions.nv_external_memory_rdma = true;
                }
                "VK_NV_external_memory_win32" => {
                    extensions.nv_external_memory_win32 = true;
                }
                "VK_NV_fill_rectangle" => {
                    extensions.nv_fill_rectangle = true;
                }
                "VK_NV_fragment_coverage_to_color" => {
                    extensions.nv_fragment_coverage_to_color = true;
                }
                "VK_NV_fragment_shader_barycentric" => {
                    extensions.nv_fragment_shader_barycentric = true;
                }
                "VK_NV_fragment_shading_rate_enums" => {
                    extensions.nv_fragment_shading_rate_enums = true;
                }
                "VK_NV_framebuffer_mixed_samples" => {
                    extensions.nv_framebuffer_mixed_samples = true;
                }
                "VK_NV_geometry_shader_passthrough" => {
                    extensions.nv_geometry_shader_passthrough = true;
                }
                "VK_NV_glsl_shader" => {
                    extensions.nv_glsl_shader = true;
                }
                "VK_NV_inherited_viewport_scissor" => {
                    extensions.nv_inherited_viewport_scissor = true;
                }
                "VK_NV_linear_color_attachment" => {
                    extensions.nv_linear_color_attachment = true;
                }
                "VK_NV_low_latency" => {
                    extensions.nv_low_latency = true;
                }
                "VK_NV_memory_decompression" => {
                    extensions.nv_memory_decompression = true;
                }
                "VK_NV_mesh_shader" => {
                    extensions.nv_mesh_shader = true;
                }
                "VK_NV_optical_flow" => {
                    extensions.nv_optical_flow = true;
                }
                "VK_NV_present_barrier" => {
                    extensions.nv_present_barrier = true;
                }
                "VK_NV_ray_tracing" => {
                    extensions.nv_ray_tracing = true;
                }
                "VK_NV_ray_tracing_invocation_reorder" => {
                    extensions.nv_ray_tracing_invocation_reorder = true;
                }
                "VK_NV_ray_tracing_motion_blur" => {
                    extensions.nv_ray_tracing_motion_blur = true;
                }
                "VK_NV_representative_fragment_test" => {
                    extensions.nv_representative_fragment_test = true;
                }
                "VK_NV_sample_mask_override_coverage" => {
                    extensions.nv_sample_mask_override_coverage = true;
                }
                "VK_NV_scissor_exclusive" => {
                    extensions.nv_scissor_exclusive = true;
                }
                "VK_NV_shader_image_footprint" => {
                    extensions.nv_shader_image_footprint = true;
                }
                "VK_NV_shader_sm_builtins" => {
                    extensions.nv_shader_sm_builtins = true;
                }
                "VK_NV_shader_subgroup_partitioned" => {
                    extensions.nv_shader_subgroup_partitioned = true;
                }
                "VK_NV_shading_rate_image" => {
                    extensions.nv_shading_rate_image = true;
                }
                "VK_NV_viewport_array2" => {
                    extensions.nv_viewport_array2 = true;
                }
                "VK_NV_viewport_swizzle" => {
                    extensions.nv_viewport_swizzle = true;
                }
                "VK_NV_win32_keyed_mutex" => {
                    extensions.nv_win32_keyed_mutex = true;
                }
                "VK_QCOM_fragment_density_map_offset" => {
                    extensions.qcom_fragment_density_map_offset = true;
                }
                "VK_QCOM_image_processing" => {
                    extensions.qcom_image_processing = true;
                }
                "VK_QCOM_multiview_per_view_render_areas" => {
                    extensions.qcom_multiview_per_view_render_areas = true;
                }
                "VK_QCOM_multiview_per_view_viewports" => {
                    extensions.qcom_multiview_per_view_viewports = true;
                }
                "VK_QCOM_render_pass_shader_resolve" => {
                    extensions.qcom_render_pass_shader_resolve = true;
                }
                "VK_QCOM_render_pass_store_ops" => {
                    extensions.qcom_render_pass_store_ops = true;
                }
                "VK_QCOM_render_pass_transform" => {
                    extensions.qcom_render_pass_transform = true;
                }
                "VK_QCOM_rotated_copy_commands" => {
                    extensions.qcom_rotated_copy_commands = true;
                }
                "VK_QCOM_tile_properties" => {
                    extensions.qcom_tile_properties = true;
                }
                "VK_SEC_amigo_profiling" => {
                    extensions.sec_amigo_profiling = true;
                }
                "VK_VALVE_descriptor_set_host_mapping" => {
                    extensions.valve_descriptor_set_host_mapping = true;
                }
                "VK_VALVE_mutable_descriptor_type" => {
                    extensions.valve_mutable_descriptor_type = true;
                }
                _ => (),
            }
        }
        extensions
    }
}