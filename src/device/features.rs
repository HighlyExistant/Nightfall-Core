use std::sync::Arc;

use ash::vk;

use crate::instance::Instance;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct DeviceFeatures {
    // Vulkan Physical Device Features 13
    pub robust_image_access: bool,
    pub inline_uniform_block: bool,
    pub descriptor_binding_inline_uniform_block_update_after_bind: bool,
    pub pipeline_creation_cache_control: bool,
    pub private_data: bool,
    pub shader_demote_to_helper_invocation: bool,
    pub shader_terminate_invocation: bool,
    pub subgroup_size_control: bool,
    pub compute_full_subgroups: bool,
    pub synchronization2: bool,
    pub texture_compression_astc_hdr: bool,
    pub shader_zero_initialize_workgroup_memory: bool,
    pub dynamic_rendering: bool,
    pub shader_integer_dot_product: bool,
    pub maintenance4: bool,
    // Vulkan Physical Device Features 12
    pub sampler_mirror_clamp_to_edge: bool,
    pub draw_indirect_count: bool,
    pub storage_buffer8_bit_access: bool,
    pub uniform_and_storage_buffer8_bit_access: bool,
    pub storage_push_constant8: bool,
    pub shader_buffer_int64_atomics: bool,
    pub shader_shared_int64_atomics: bool,
    pub shader_float16: bool,
    pub shader_int8: bool,
    pub descriptor_indexing: bool,
    pub shader_input_attachment_array_dynamic_indexing: bool,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
    pub shader_uniform_buffer_array_non_uniform_indexing: bool,
    pub shader_sampled_image_array_non_uniform_indexing: bool,
    pub shader_storage_buffer_array_non_uniform_indexing: bool,
    pub shader_storage_image_array_non_uniform_indexing: bool,
    pub shader_input_attachment_array_non_uniform_indexing: bool,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    pub descriptor_binding_uniform_buffer_update_after_bind: bool,
    pub descriptor_binding_sampled_image_update_after_bind: bool,
    pub descriptor_binding_storage_image_update_after_bind: bool,
    pub descriptor_binding_storage_buffer_update_after_bind: bool,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    pub descriptor_binding_update_unused_while_pending: bool,
    pub descriptor_binding_partially_bound: bool,
    pub descriptor_binding_variable_descriptor_count: bool,
    pub runtime_descriptor_array: bool,
    pub sampler_filter_minmax: bool,
    pub scalar_block_layout: bool,
    pub imageless_framebuffer: bool,
    pub uniform_buffer_standard_layout: bool,
    pub shader_subgroup_extended_types: bool,
    pub separate_depth_stencil_layouts: bool,
    pub host_query_reset: bool,
    pub timeline_semaphore: bool,
    pub buffer_device_address: bool,
    pub buffer_device_address_capture_replay: bool,
    pub buffer_device_address_multi_device: bool,
    pub vulkan_memory_model: bool,
    pub vulkan_memory_model_device_scope: bool,
    pub vulkan_memory_model_availability_visibility_chains: bool,
    pub shader_output_viewport_index: bool,
    pub shader_output_layer: bool,
    pub subgroup_broadcast_dynamic_id: bool,
    // Vulkan Physical Device Features 11
    pub storage_buffer16_bit_access: bool,
    pub uniform_and_storage_buffer16_bit_access: bool,
    pub storage_push_constant16: bool,
    pub storage_input_output16: bool,
    pub multiview: bool,
    pub multiview_geometry_shader: bool,
    pub multiview_tessellation_shader: bool,
    pub variable_pointers_storage_buffer: bool,
    pub variable_pointers: bool,
    pub protected_memory: bool,
    pub sampler_ycbcr_conversion: bool,
    pub shader_draw_parameters: bool,
    // Vulkan Physical Device Features
    pub robust_buffer_access: bool,
    pub full_draw_index_uint32: bool,
    pub image_cube_array: bool,
    pub independent_blend: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    pub sample_rate_shading: bool,
    pub dual_src_blend: bool,
    pub logic_op: bool,
    pub multi_draw_indirect: bool,
    pub draw_indirect_first_instance: bool,
    pub depth_clamp: bool,
    pub depth_bias_clamp: bool,
    pub fill_mode_non_solid: bool,
    pub depth_bounds: bool,
    pub wide_lines: bool,
    pub large_points: bool,
    pub alpha_to_one: bool,
    pub multi_viewport: bool,
    pub sampler_anisotropy: bool,
    pub texture_compression_etc2: bool,
    pub texture_compression_astc_ldr: bool,
    pub texture_compression_bc: bool,
    pub occlusion_query_precise: bool,
    pub pipeline_statistics_query: bool,
    pub vertex_pipeline_stores_and_atomics: bool,
    pub fragment_stores_and_atomics: bool,
    pub shader_tessellation_and_geometry_point_size: bool,
    pub shader_image_gather_extended: bool,
    pub shader_storage_image_extended_formats: bool,
    pub shader_storage_image_multisample: bool,
    pub shader_storage_image_read_without_format: bool,
    pub shader_storage_image_write_without_format: bool,
    pub shader_uniform_buffer_array_dynamic_indexing: bool,
    pub shader_sampled_image_array_dynamic_indexing: bool,
    pub shader_storage_buffer_array_dynamic_indexing: bool,
    pub shader_storage_image_array_dynamic_indexing: bool,
    pub shader_clip_distance: bool,
    pub shader_cull_distance: bool,
    pub shader_float64: bool,
    pub shader_int64: bool,
    pub shader_int16: bool,
    pub shader_resource_residency: bool,
    pub shader_resource_min_lod: bool,
    pub sparse_binding: bool,
    pub sparse_residency_buffer: bool,
    pub sparse_residency_image2_d: bool,
    pub sparse_residency_image3_d: bool,
    pub sparse_residency2_samples: bool,
    pub sparse_residency4_samples: bool,
    pub sparse_residency8_samples: bool,
    pub sparse_residency16_samples: bool,
    pub sparse_residency_aliased: bool,
    pub variable_multisample_rate: bool,
    pub inherited_queries: bool,
}

impl DeviceFeatures {
    pub fn get_vulkan13features(&self) -> vk::PhysicalDeviceVulkan13Features {
        vk::PhysicalDeviceVulkan13Features {
            robust_image_access: self.robust_image_access as u32,
            inline_uniform_block: self.inline_uniform_block as u32,
            descriptor_binding_inline_uniform_block_update_after_bind: self.descriptor_binding_inline_uniform_block_update_after_bind as u32,
            pipeline_creation_cache_control: self.pipeline_creation_cache_control as u32,
            private_data: self.private_data as u32,
            shader_demote_to_helper_invocation: self.shader_demote_to_helper_invocation as u32,
            shader_terminate_invocation: self.shader_terminate_invocation as u32,
            subgroup_size_control: self.subgroup_size_control as u32,
            compute_full_subgroups: self.compute_full_subgroups as u32,
            synchronization2: self.synchronization2 as u32,
            texture_compression_astc_hdr: self.texture_compression_astc_hdr as u32,
            shader_zero_initialize_workgroup_memory: self.shader_zero_initialize_workgroup_memory as u32,
            dynamic_rendering: self.dynamic_rendering as u32,
            shader_integer_dot_product: self.shader_integer_dot_product as u32,
            maintenance4: self.maintenance4 as u32,
            ..Default::default()
        }
    }
    pub fn get_vulkan12features(&self) -> vk::PhysicalDeviceVulkan12Features {
        vk::PhysicalDeviceVulkan12Features {
            sampler_mirror_clamp_to_edge: self.sampler_mirror_clamp_to_edge as u32,
            draw_indirect_count: self.draw_indirect_count as u32,
            storage_buffer8_bit_access: self.storage_buffer8_bit_access as u32,
            uniform_and_storage_buffer8_bit_access: self.uniform_and_storage_buffer8_bit_access as u32,
            storage_push_constant8: self.storage_push_constant8 as u32,
            shader_buffer_int64_atomics: self.shader_buffer_int64_atomics as u32,
            shader_shared_int64_atomics: self.shader_shared_int64_atomics as u32,
            shader_float16: self.shader_float16 as u32,
            shader_int8: self.shader_int8 as u32,
            descriptor_indexing: self.descriptor_indexing as u32,
            shader_input_attachment_array_dynamic_indexing: self.shader_input_attachment_array_dynamic_indexing as u32,
            shader_uniform_texel_buffer_array_dynamic_indexing: self.shader_uniform_texel_buffer_array_dynamic_indexing as u32,
            shader_storage_texel_buffer_array_dynamic_indexing: self.shader_storage_texel_buffer_array_dynamic_indexing as u32,
            shader_uniform_buffer_array_non_uniform_indexing: self.shader_uniform_buffer_array_non_uniform_indexing as u32,
            shader_sampled_image_array_non_uniform_indexing: self.shader_sampled_image_array_non_uniform_indexing as u32,
            shader_storage_buffer_array_non_uniform_indexing: self.shader_storage_buffer_array_non_uniform_indexing as u32,
            shader_storage_image_array_non_uniform_indexing: self.shader_storage_image_array_non_uniform_indexing as u32,
            shader_input_attachment_array_non_uniform_indexing: self.shader_input_attachment_array_non_uniform_indexing as u32,
            shader_uniform_texel_buffer_array_non_uniform_indexing: self.shader_uniform_texel_buffer_array_non_uniform_indexing as u32,
            shader_storage_texel_buffer_array_non_uniform_indexing: self.shader_storage_texel_buffer_array_non_uniform_indexing as u32,
            descriptor_binding_uniform_buffer_update_after_bind: self.descriptor_binding_uniform_buffer_update_after_bind as u32,
            descriptor_binding_sampled_image_update_after_bind: self.descriptor_binding_sampled_image_update_after_bind as u32,
            descriptor_binding_storage_image_update_after_bind: self.descriptor_binding_storage_image_update_after_bind as u32,
            descriptor_binding_storage_buffer_update_after_bind: self.descriptor_binding_storage_buffer_update_after_bind as u32,
            descriptor_binding_uniform_texel_buffer_update_after_bind: self.descriptor_binding_uniform_texel_buffer_update_after_bind as u32,
            descriptor_binding_storage_texel_buffer_update_after_bind: self.descriptor_binding_storage_texel_buffer_update_after_bind as u32,
            descriptor_binding_update_unused_while_pending: self.descriptor_binding_update_unused_while_pending as u32,
            descriptor_binding_partially_bound: self.descriptor_binding_partially_bound as u32,
            descriptor_binding_variable_descriptor_count: self.descriptor_binding_variable_descriptor_count as u32,
            runtime_descriptor_array: self.runtime_descriptor_array as u32,
            sampler_filter_minmax: self.sampler_filter_minmax as u32,
            scalar_block_layout: self.scalar_block_layout as u32,
            imageless_framebuffer: self.imageless_framebuffer as u32,
            uniform_buffer_standard_layout: self.uniform_buffer_standard_layout as u32,
            shader_subgroup_extended_types: self.shader_subgroup_extended_types as u32,
            separate_depth_stencil_layouts: self.separate_depth_stencil_layouts as u32,
            host_query_reset: self.host_query_reset as u32,
            timeline_semaphore: self.timeline_semaphore as u32,
            buffer_device_address: self.buffer_device_address as u32,
            buffer_device_address_capture_replay: self.buffer_device_address_capture_replay as u32,
            buffer_device_address_multi_device: self.buffer_device_address_multi_device as u32,
            vulkan_memory_model: self.vulkan_memory_model as u32,
            vulkan_memory_model_device_scope: self.vulkan_memory_model_device_scope as u32,
            vulkan_memory_model_availability_visibility_chains: self.vulkan_memory_model_availability_visibility_chains as u32,
            shader_output_viewport_index: self.shader_output_viewport_index as u32,
            shader_output_layer: self.shader_output_layer as u32,
            subgroup_broadcast_dynamic_id: self.subgroup_broadcast_dynamic_id as u32,
            ..Default::default()
        }
    }
    pub fn get_vulkan11features(&self) -> vk::PhysicalDeviceVulkan11Features {
        vk::PhysicalDeviceVulkan11Features {
            storage_buffer16_bit_access: self.storage_buffer16_bit_access as u32,
            uniform_and_storage_buffer16_bit_access: self.uniform_and_storage_buffer16_bit_access as u32,
            storage_push_constant16: self.storage_push_constant16 as u32,
            storage_input_output16: self.storage_input_output16 as u32,
            multiview: self.multiview as u32,
            multiview_geometry_shader: self.multiview_geometry_shader as u32,
            multiview_tessellation_shader: self.multiview_tessellation_shader as u32,
            variable_pointers_storage_buffer: self.variable_pointers_storage_buffer as u32,
            variable_pointers: self.variable_pointers as u32,
            protected_memory: self.protected_memory as u32,
            sampler_ycbcr_conversion: self.sampler_ycbcr_conversion as u32,
            shader_draw_parameters: self.shader_draw_parameters as u32,
            ..Default::default()
        }
    }
    
    pub fn get_features(&self) -> vk::PhysicalDeviceFeatures {
        vk::PhysicalDeviceFeatures {
            robust_buffer_access: self.robust_buffer_access as u32,
            full_draw_index_uint32: self.full_draw_index_uint32 as u32,
            image_cube_array: self.image_cube_array as u32,
            independent_blend: self.independent_blend as u32,
            geometry_shader: self.geometry_shader as u32,
            tessellation_shader: self.tessellation_shader as u32,
            sample_rate_shading: self.sample_rate_shading as u32,
            dual_src_blend: self.dual_src_blend as u32,
            logic_op: self.logic_op as u32,
            multi_draw_indirect: self.multi_draw_indirect as u32,
            draw_indirect_first_instance: self.draw_indirect_first_instance as u32,
            depth_clamp: self.depth_clamp as u32,
            depth_bias_clamp: self.depth_bias_clamp as u32,
            fill_mode_non_solid: self.fill_mode_non_solid as u32,
            depth_bounds: self.depth_bounds as u32,
            wide_lines: self.wide_lines as u32,
            large_points: self.large_points as u32,
            alpha_to_one: self.alpha_to_one as u32,
            multi_viewport: self.multi_viewport as u32,
            sampler_anisotropy: self.sampler_anisotropy as u32,
            texture_compression_etc2: self.texture_compression_etc2 as u32,
            texture_compression_astc_ldr: self.texture_compression_astc_ldr as u32,
            texture_compression_bc: self.texture_compression_bc as u32,
            occlusion_query_precise: self.occlusion_query_precise as u32,
            pipeline_statistics_query: self.pipeline_statistics_query as u32,
            vertex_pipeline_stores_and_atomics: self.vertex_pipeline_stores_and_atomics as u32,
            fragment_stores_and_atomics: self.fragment_stores_and_atomics as u32,
            shader_tessellation_and_geometry_point_size: self.shader_tessellation_and_geometry_point_size as u32,
            shader_image_gather_extended: self.shader_image_gather_extended as u32,
            shader_storage_image_extended_formats: self.shader_storage_image_extended_formats as u32,
            shader_storage_image_multisample: self.shader_storage_image_multisample as u32,
            shader_storage_image_read_without_format: self.shader_storage_image_read_without_format as u32,
            shader_storage_image_write_without_format: self.shader_storage_image_write_without_format as u32,
            shader_uniform_buffer_array_dynamic_indexing: self.shader_uniform_buffer_array_dynamic_indexing as u32,
            shader_sampled_image_array_dynamic_indexing: self.shader_sampled_image_array_dynamic_indexing as u32,
            shader_storage_buffer_array_dynamic_indexing: self.shader_storage_buffer_array_dynamic_indexing as u32,
            shader_storage_image_array_dynamic_indexing: self.shader_storage_image_array_dynamic_indexing as u32,
            shader_clip_distance: self.shader_clip_distance as u32,
            shader_cull_distance: self.shader_cull_distance as u32,
            shader_float64: self.shader_float64 as u32,
            shader_int64: self.shader_int64 as u32,
            shader_int16: self.shader_int16 as u32,
            shader_resource_residency: self.shader_resource_residency as u32,
            shader_resource_min_lod: self.shader_resource_min_lod as u32,
            sparse_binding: self.sparse_binding as u32,
            sparse_residency_buffer: self.sparse_residency_buffer as u32,
            sparse_residency_image2_d: self.sparse_residency_image2_d as u32,
            sparse_residency_image3_d: self.sparse_residency_image3_d as u32,
            sparse_residency2_samples: self.sparse_residency2_samples as u32,
            sparse_residency4_samples: self.sparse_residency4_samples as u32,
            sparse_residency8_samples: self.sparse_residency8_samples as u32,
            sparse_residency16_samples: self.sparse_residency16_samples as u32,
            sparse_residency_aliased: self.sparse_residency_aliased as u32,
            variable_multisample_rate: self.variable_multisample_rate as u32,
            inherited_queries: self.inherited_queries as u32,
            ..Default::default()
        }
    }
    pub fn set_features_13(&mut self, features: &vk::PhysicalDeviceVulkan13Features) {
            self.robust_image_access = features.robust_image_access != 0;
            self.inline_uniform_block = features.inline_uniform_block != 0;
            self.descriptor_binding_inline_uniform_block_update_after_bind = features.descriptor_binding_inline_uniform_block_update_after_bind != 0;
            self.pipeline_creation_cache_control = features.pipeline_creation_cache_control != 0;
            self.private_data = features.private_data != 0;
            self.shader_demote_to_helper_invocation = features.shader_demote_to_helper_invocation != 0;
            self.shader_terminate_invocation = features.shader_terminate_invocation != 0;
            self.subgroup_size_control = features.subgroup_size_control != 0;
            self.compute_full_subgroups = features.compute_full_subgroups != 0;
            self.synchronization2 = features.synchronization2 != 0;
            self.texture_compression_astc_hdr = features.texture_compression_astc_hdr != 0;
            self.shader_zero_initialize_workgroup_memory = features.shader_zero_initialize_workgroup_memory != 0;
            self.dynamic_rendering = features.dynamic_rendering != 0;
            self.shader_integer_dot_product = features.shader_integer_dot_product != 0;
            self.maintenance4 = features.maintenance4 != 0;
    }
    pub fn set_features_12(&mut self, features: &vk::PhysicalDeviceVulkan12Features) {
        self.sampler_mirror_clamp_to_edge = features.sampler_mirror_clamp_to_edge != 0;
        self.draw_indirect_count = features.draw_indirect_count != 0;
        self.storage_buffer8_bit_access = features.storage_buffer8_bit_access != 0;
        self.uniform_and_storage_buffer8_bit_access = features.uniform_and_storage_buffer8_bit_access != 0;
        self.storage_push_constant8 = features.storage_push_constant8 != 0;
        self.shader_buffer_int64_atomics = features.shader_buffer_int64_atomics != 0;
        self.shader_shared_int64_atomics = features.shader_shared_int64_atomics != 0;
        self.shader_float16 = features.shader_float16 != 0;
        self.shader_int8 = features.shader_int8 != 0;
        self.descriptor_indexing = features.descriptor_indexing != 0;
        self.shader_input_attachment_array_dynamic_indexing = features.shader_input_attachment_array_dynamic_indexing != 0;
        self.shader_uniform_texel_buffer_array_dynamic_indexing = features.shader_uniform_texel_buffer_array_dynamic_indexing != 0;
        self.shader_storage_texel_buffer_array_dynamic_indexing = features.shader_storage_texel_buffer_array_dynamic_indexing != 0;
        self.shader_uniform_buffer_array_non_uniform_indexing = features.shader_uniform_buffer_array_non_uniform_indexing != 0;
        self.shader_sampled_image_array_non_uniform_indexing = features.shader_sampled_image_array_non_uniform_indexing != 0;
        self.shader_storage_buffer_array_non_uniform_indexing = features.shader_storage_buffer_array_non_uniform_indexing != 0;
        self.shader_storage_image_array_non_uniform_indexing = features.shader_storage_image_array_non_uniform_indexing != 0;
        self.shader_input_attachment_array_non_uniform_indexing = features.shader_input_attachment_array_non_uniform_indexing != 0;
        self.shader_uniform_texel_buffer_array_non_uniform_indexing = features.shader_uniform_texel_buffer_array_non_uniform_indexing != 0;
        self.shader_storage_texel_buffer_array_non_uniform_indexing = features.shader_storage_texel_buffer_array_non_uniform_indexing != 0;
        self.descriptor_binding_uniform_buffer_update_after_bind = features.descriptor_binding_uniform_buffer_update_after_bind != 0;
        self.descriptor_binding_sampled_image_update_after_bind = features.descriptor_binding_sampled_image_update_after_bind != 0;
        self.descriptor_binding_storage_image_update_after_bind = features.descriptor_binding_storage_image_update_after_bind != 0;
        self.descriptor_binding_storage_buffer_update_after_bind = features.descriptor_binding_storage_buffer_update_after_bind != 0;
        self.descriptor_binding_uniform_texel_buffer_update_after_bind = features.descriptor_binding_uniform_texel_buffer_update_after_bind != 0;
        self.descriptor_binding_storage_texel_buffer_update_after_bind = features.descriptor_binding_storage_texel_buffer_update_after_bind != 0;
        self.descriptor_binding_update_unused_while_pending = features.descriptor_binding_update_unused_while_pending != 0;
        self.descriptor_binding_partially_bound = features.descriptor_binding_partially_bound != 0;
        self.descriptor_binding_variable_descriptor_count = features.descriptor_binding_variable_descriptor_count != 0;
        self.runtime_descriptor_array = features.runtime_descriptor_array != 0;
        self.sampler_filter_minmax = features.sampler_filter_minmax != 0;
        self.scalar_block_layout = features.scalar_block_layout != 0;
        self.imageless_framebuffer = features.imageless_framebuffer != 0;
        self.uniform_buffer_standard_layout = features.uniform_buffer_standard_layout != 0;
        self.shader_subgroup_extended_types = features.shader_subgroup_extended_types != 0;
        self.separate_depth_stencil_layouts = features.separate_depth_stencil_layouts != 0;
        self.host_query_reset = features.host_query_reset != 0;
        self.timeline_semaphore = features.timeline_semaphore != 0;
        self.buffer_device_address = features.buffer_device_address != 0;
        self.buffer_device_address_capture_replay = features.buffer_device_address_capture_replay != 0;
        self.buffer_device_address_multi_device = features.buffer_device_address_multi_device != 0;
        self.vulkan_memory_model = features.vulkan_memory_model != 0;
        self.vulkan_memory_model_device_scope = features.vulkan_memory_model_device_scope != 0;
        self.vulkan_memory_model_availability_visibility_chains = features.vulkan_memory_model_availability_visibility_chains != 0;
        self.shader_output_viewport_index = features.shader_output_viewport_index != 0;
        self.shader_output_layer = features.shader_output_layer != 0;
        self.subgroup_broadcast_dynamic_id = features.subgroup_broadcast_dynamic_id != 0;
    }
    pub fn set_features_11(&mut self, features: &vk::PhysicalDeviceVulkan11Features) {
        self.storage_buffer16_bit_access = features.storage_buffer16_bit_access != 0;
        self.uniform_and_storage_buffer16_bit_access = features.uniform_and_storage_buffer16_bit_access != 0;
        self.storage_push_constant16 = features.storage_push_constant16 != 0;
        self.storage_input_output16 = features.storage_input_output16 != 0;
        self.multiview = features.multiview != 0;
        self.multiview_geometry_shader = features.multiview_geometry_shader != 0;
        self.multiview_tessellation_shader = features.multiview_tessellation_shader != 0;
        self.variable_pointers_storage_buffer = features.variable_pointers_storage_buffer != 0;
        self.variable_pointers = features.variable_pointers != 0;
        self.protected_memory = features.protected_memory != 0;
        self.sampler_ycbcr_conversion = features.sampler_ycbcr_conversion != 0;
        self.shader_draw_parameters = features.shader_draw_parameters != 0;
    }
    pub fn set_features(&mut self, features: &vk::PhysicalDeviceFeatures) {
        self.robust_buffer_access = features.robust_buffer_access != 0;
        self.full_draw_index_uint32 = features.full_draw_index_uint32 != 0;
        self.image_cube_array = features.image_cube_array != 0;
        self.independent_blend = features.independent_blend != 0;
        self.geometry_shader = features.geometry_shader != 0;
        self.tessellation_shader = features.tessellation_shader != 0;
        self.sample_rate_shading = features.sample_rate_shading != 0;
        self.dual_src_blend = features.dual_src_blend != 0;
        self.logic_op = features.logic_op != 0;
        self.multi_draw_indirect = features.multi_draw_indirect != 0;
        self.draw_indirect_first_instance = features.draw_indirect_first_instance != 0;
        self.depth_clamp = features.depth_clamp != 0;
        self.depth_bias_clamp = features.depth_bias_clamp != 0;
        self.fill_mode_non_solid = features.fill_mode_non_solid != 0;
        self.depth_bounds = features.depth_bounds != 0;
        self.wide_lines = features.wide_lines != 0;
        self.large_points = features.large_points != 0;
        self.alpha_to_one = features.alpha_to_one != 0;
        self.multi_viewport = features.multi_viewport != 0;
        self.sampler_anisotropy = features.sampler_anisotropy != 0;
        self.texture_compression_etc2 = features.texture_compression_etc2 != 0;
        self.texture_compression_astc_ldr = features.texture_compression_astc_ldr != 0;
        self.texture_compression_bc = features.texture_compression_bc != 0;
        self.occlusion_query_precise = features.occlusion_query_precise != 0;
        self.pipeline_statistics_query = features.pipeline_statistics_query != 0;
        self.vertex_pipeline_stores_and_atomics = features.vertex_pipeline_stores_and_atomics != 0;
        self.fragment_stores_and_atomics = features.fragment_stores_and_atomics != 0;
        self.shader_tessellation_and_geometry_point_size = features.shader_tessellation_and_geometry_point_size != 0;
        self.shader_image_gather_extended = features.shader_image_gather_extended != 0;
        self.shader_storage_image_extended_formats = features.shader_storage_image_extended_formats != 0;
        self.shader_storage_image_multisample = features.shader_storage_image_multisample != 0;
        self.shader_storage_image_read_without_format = features.shader_storage_image_read_without_format != 0;
        self.shader_storage_image_write_without_format = features.shader_storage_image_write_without_format != 0;
        self.shader_uniform_buffer_array_dynamic_indexing = features.shader_uniform_buffer_array_dynamic_indexing != 0;
        self.shader_sampled_image_array_dynamic_indexing = features.shader_sampled_image_array_dynamic_indexing != 0;
        self.shader_storage_buffer_array_dynamic_indexing = features.shader_storage_buffer_array_dynamic_indexing != 0;
        self.shader_storage_image_array_dynamic_indexing = features.shader_storage_image_array_dynamic_indexing != 0;
        self.shader_clip_distance = features.shader_clip_distance != 0;
        self.shader_cull_distance = features.shader_cull_distance != 0;
        self.shader_float64 = features.shader_float64 != 0;
        self.shader_int64 = features.shader_int64 != 0;
        self.shader_int16 = features.shader_int16 != 0;
        self.shader_resource_residency = features.shader_resource_residency != 0;
        self.shader_resource_min_lod = features.shader_resource_min_lod != 0;
        self.sparse_binding = features.sparse_binding != 0;
        self.sparse_residency_buffer = features.sparse_residency_buffer != 0;
        self.sparse_residency_image2_d = features.sparse_residency_image2_d != 0;
        self.sparse_residency_image3_d = features.sparse_residency_image3_d != 0;
        self.sparse_residency2_samples = features.sparse_residency2_samples != 0;
        self.sparse_residency4_samples = features.sparse_residency4_samples != 0;
        self.sparse_residency8_samples = features.sparse_residency8_samples != 0;
        self.sparse_residency16_samples = features.sparse_residency16_samples != 0;
        self.sparse_residency_aliased = features.sparse_residency_aliased != 0;
        self.variable_multisample_rate = features.variable_multisample_rate != 0;
        self.inherited_queries = features.inherited_queries != 0;
    }
    pub fn get_all_features(instance: &Arc<Instance>, physical_device: vk::PhysicalDevice) -> Self {
        let mut this = Self::default();
        match instance.api_version().minor {
            0 => {
                let features = unsafe { instance.instance.get_physical_device_features(physical_device) };
                this.set_features(&features);
            }
            1 => {
                let mut vk11 = vk::PhysicalDeviceVulkan11Features::default();
                let mut features = vk::PhysicalDeviceFeatures2 {
                    p_next: &mut vk11 as *mut _ as _,
                    ..Default::default()
                };
                unsafe { instance.instance.get_physical_device_features2(physical_device, &mut features) }
                this.set_features(&features.features);
                this.set_features_11(&vk11);
            }
            2 => {
                let mut vk11 = vk::PhysicalDeviceVulkan11Features::default();
                let mut vk12 = vk::PhysicalDeviceVulkan12Features {
                    p_next: &mut vk11 as *mut _ as _,
                    ..Default::default()
                };
                let mut features = vk::PhysicalDeviceFeatures2 {
                    p_next: &mut vk12 as *mut _ as _,
                    ..Default::default()
                };
                unsafe { instance.instance.get_physical_device_features2(physical_device, &mut features) }
                this.set_features(&features.features);
                this.set_features_11(&vk11);
                this.set_features_12(&vk12);
                
            }
            3 => {
                let mut vk11 = vk::PhysicalDeviceVulkan11Features::default();
                let mut vk12 = vk::PhysicalDeviceVulkan12Features {
                    p_next: &mut vk11 as *mut _ as _,
                    ..Default::default()
                };
                let mut vk13 = vk::PhysicalDeviceVulkan13Features {
                    p_next: &mut vk12 as *mut _ as _,
                    ..Default::default()
                };
                let mut features = vk::PhysicalDeviceFeatures2 {
                    p_next: &mut vk13 as *mut _ as _,
                    ..Default::default()
                };
                unsafe { instance.instance.get_physical_device_features2(physical_device, &mut features) }
                this.set_features(&features.features);
                this.set_features_11(&vk11);
                this.set_features_12(&vk12);
                this.set_features_13(&vk13);
            }
            _ => {}
        }
        this
    }
    pub fn validate_features11(mut self, instance: Arc<ash::Instance>, physical_device: vk::PhysicalDevice) -> Self {
        let mut vk11 = vk::PhysicalDeviceVulkan11Features::default();
        let mut features = vk::PhysicalDeviceFeatures2 {
            p_next: &mut vk11 as *mut _ as _,
            ..Default::default()
        };
        unsafe { instance.get_physical_device_features2(physical_device, &mut features) }
        self.storage_buffer16_bit_access &= vk11.storage_buffer16_bit_access != 0;
        self.uniform_and_storage_buffer16_bit_access &= vk11.uniform_and_storage_buffer16_bit_access != 0;
        self.storage_push_constant16 &= vk11.storage_push_constant16 != 0;
        self.storage_input_output16 &= vk11.storage_input_output16 != 0;
        self.multiview &= vk11.multiview != 0;
        self.multiview_geometry_shader &= vk11.multiview_geometry_shader != 0;
        self.multiview_tessellation_shader &= vk11.multiview_tessellation_shader != 0;
        self.variable_pointers_storage_buffer &= vk11.variable_pointers_storage_buffer != 0;
        self.variable_pointers &= vk11.variable_pointers != 0;
        self.protected_memory &= vk11.protected_memory != 0;
        self.sampler_ycbcr_conversion &= vk11.sampler_ycbcr_conversion != 0;
        self.shader_draw_parameters &= vk11.shader_draw_parameters != 0;
        self
    }
    pub fn features10_active(&self) -> bool {
        self.robust_buffer_access ||
        self.full_draw_index_uint32 ||
        self.image_cube_array ||
        self.independent_blend ||
        self.geometry_shader ||
        self.tessellation_shader ||
        self.sample_rate_shading ||
        self.dual_src_blend ||
        self.logic_op ||
        self.multi_draw_indirect ||
        self.draw_indirect_first_instance ||
        self.depth_clamp ||
        self.depth_bias_clamp ||
        self.fill_mode_non_solid ||
        self.depth_bounds ||
        self.wide_lines ||
        self.large_points ||
        self.alpha_to_one ||
        self.multi_viewport ||
        self.sampler_anisotropy ||
        self.texture_compression_etc2 ||
        self.texture_compression_astc_ldr ||
        self.texture_compression_bc ||
        self.occlusion_query_precise ||
        self.pipeline_statistics_query ||
        self.vertex_pipeline_stores_and_atomics ||
        self.fragment_stores_and_atomics ||
        self.shader_tessellation_and_geometry_point_size ||
        self.shader_image_gather_extended ||
        self.shader_storage_image_extended_formats ||
        self.shader_storage_image_multisample ||
        self.shader_storage_image_read_without_format ||
        self.shader_storage_image_write_without_format ||
        self.shader_uniform_buffer_array_dynamic_indexing ||
        self.shader_sampled_image_array_dynamic_indexing ||
        self.shader_storage_buffer_array_dynamic_indexing ||
        self.shader_storage_image_array_dynamic_indexing ||
        self.shader_clip_distance ||
        self.shader_cull_distance ||
        self.shader_float64 ||
        self.shader_int64 ||
        self.shader_int16 ||
        self.shader_resource_residency ||
        self.shader_resource_min_lod ||
        self.sparse_binding ||
        self.sparse_residency_buffer ||
        self.sparse_residency_image2_d ||
        self.sparse_residency_image3_d ||
        self.sparse_residency2_samples ||
        self.sparse_residency4_samples ||
        self.sparse_residency8_samples ||
        self.sparse_residency16_samples ||
        self.sparse_residency_aliased ||
        self.variable_multisample_rate ||
        self.inherited_queries
    }
    pub fn features11_active(&self) -> bool {
        self.storage_buffer16_bit_access ||
        self.uniform_and_storage_buffer16_bit_access ||
        self.storage_push_constant16 ||
        self.storage_input_output16 ||
        self.multiview ||
        self.multiview_geometry_shader ||
        self.multiview_tessellation_shader ||
        self.variable_pointers_storage_buffer ||
        self.variable_pointers ||
        self.protected_memory ||
        self.sampler_ycbcr_conversion ||
        self.shader_draw_parameters
    }
    pub fn features12_active(&self) -> bool {
        self.sampler_mirror_clamp_to_edge ||
        self.draw_indirect_count ||
        self.storage_buffer8_bit_access ||
        self.uniform_and_storage_buffer8_bit_access ||
        self.storage_push_constant8 ||
        self.shader_buffer_int64_atomics ||
        self.shader_shared_int64_atomics ||
        self.shader_float16 ||
        self.shader_int8 ||
        self.descriptor_indexing ||
        self.shader_input_attachment_array_dynamic_indexing ||
        self.shader_uniform_texel_buffer_array_dynamic_indexing ||
        self.shader_storage_texel_buffer_array_dynamic_indexing ||
        self.shader_uniform_buffer_array_non_uniform_indexing ||
        self.shader_sampled_image_array_non_uniform_indexing ||
        self.shader_storage_buffer_array_non_uniform_indexing ||
        self.shader_storage_image_array_non_uniform_indexing ||
        self.shader_input_attachment_array_non_uniform_indexing ||
        self.shader_uniform_texel_buffer_array_non_uniform_indexing ||
        self.shader_storage_texel_buffer_array_non_uniform_indexing ||
        self.descriptor_binding_uniform_buffer_update_after_bind ||
        self.descriptor_binding_sampled_image_update_after_bind ||
        self.descriptor_binding_storage_image_update_after_bind ||
        self.descriptor_binding_storage_buffer_update_after_bind ||
        self.descriptor_binding_uniform_texel_buffer_update_after_bind ||
        self.descriptor_binding_storage_texel_buffer_update_after_bind ||
        self.descriptor_binding_update_unused_while_pending ||
        self.descriptor_binding_partially_bound ||
        self.descriptor_binding_variable_descriptor_count ||
        self.runtime_descriptor_array ||
        self.sampler_filter_minmax ||
        self.scalar_block_layout ||
        self.imageless_framebuffer ||
        self.uniform_buffer_standard_layout ||
        self.shader_subgroup_extended_types ||
        self.separate_depth_stencil_layouts ||
        self.host_query_reset ||
        self.timeline_semaphore ||
        self.buffer_device_address ||
        self.buffer_device_address_capture_replay ||
        self.buffer_device_address_multi_device ||
        self.vulkan_memory_model ||
        self.vulkan_memory_model_device_scope ||
        self.vulkan_memory_model_availability_visibility_chains ||
        self.shader_output_viewport_index ||
        self.shader_output_layer ||
        self.subgroup_broadcast_dynamic_id
    }
    pub fn features13_active(&self) -> bool {
        self.robust_image_access ||
        self.inline_uniform_block ||
        self.descriptor_binding_inline_uniform_block_update_after_bind ||
        self.pipeline_creation_cache_control ||
        self.private_data ||
        self.shader_demote_to_helper_invocation ||
        self.shader_terminate_invocation ||
        self.subgroup_size_control ||
        self.compute_full_subgroups ||
        self.synchronization2 ||
        self.texture_compression_astc_hdr ||
        self.shader_zero_initialize_workgroup_memory ||
        self.dynamic_rendering ||
        self.shader_integer_dot_product ||
        self.maintenance4
    }
}