use std::{mem::MaybeUninit, ops::Sub, ptr::NonNull, sync::Arc};

use ash::{vk::{self, CopyBufferInfo2}, vk_bitflags_wrapped};
mod definitions;
pub use definitions::*;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferLevel.html>"]
pub struct CommandBufferLevel(pub(crate) i32);
impl CommandBufferLevel {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CommandBufferLevel {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html>"]
pub struct CommandBufferResetFlags(pub(crate) u32);
vk_bitflags_wrapped!(CommandBufferResetFlags, u32);
impl CommandBufferResetFlags {
    #[doc = "Release resources owned by the buffer"]
    pub const RELEASE_RESOURCES: Self = Self(0b1);
}
use crate::{barriers::{BufferMemoryBarrier, ImageMemoryBarrier, MemoryBarrier}, device::LogicalDevice, error::VulkanError, image::{ImageLayout, PipelineStageFlags}, memory::DependencyFlags, pipeline::shader::ShaderStageFlags, NfPtr};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html>"]
pub struct CommandPoolCreateFlags(pub(crate) u32);
vk_bitflags_wrapped!(CommandPoolCreateFlags, u32);
impl CommandPoolCreateFlags {
    #[doc = "Command buffers have a short lifetime"]
    pub const TRANSIENT: Self = Self(0b1);
    #[doc = "Command buffers may release their memory individually"]
    pub const RESET_COMMAND_BUFFER: Self = Self(0b10);
}
pub struct CommandBufferIter {
    start: *mut vk::CommandBuffer,
    end: *mut vk::CommandBuffer,
}
impl CommandBufferIter {
    pub unsafe fn new(start: *mut vk::CommandBuffer, len: usize) -> Self {
        Self { start, end: start.add(len) }
    }
}
impl Iterator for CommandBufferIter {
    type Item = vk::CommandBuffer;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.start = unsafe { self.start.add(1) };
            unsafe { Some(*self.start) }
        }
    }
}
impl ExactSizeIterator for CommandBufferIter {
    fn len(&self) -> usize {
        unsafe { (self.end.offset_from(self.start)) as usize/std::mem::size_of::<vk::CommandBuffer>() } 
    }
}
pub struct CommandPoolAllocation {
    pub(crate) command_buffer: vk::CommandBuffer,
    pub(crate) level: CommandBufferLevel,
    pub(crate) device: Arc<LogicalDevice>,
}
impl CommandPoolAllocation {
    #[inline]
    pub fn level(&self) -> CommandBufferLevel { self.level }
    #[inline]
    pub fn device(&self) -> Arc<LogicalDevice> { self.device.clone() }
    #[inline]
    pub fn get_command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }
    #[inline]
    pub fn empty(device: Arc<LogicalDevice>) -> Self {
        Self { command_buffer: vk::CommandBuffer::null(), level: CommandBufferLevel::PRIMARY, device }
    }
    pub fn begin(&self, info: CommandBufferBeginInfo) -> Result<(), VulkanError> {
        let inheritence: Option<vk::CommandBufferInheritanceInfo> = info.p_inheritance_info.map(Into::into);
        let p_inheritance_info = if let Some(inheritence) = &inheritence {
            inheritence as *const _
        } else {
            std::ptr::null()
        };
        let begin_info = vk::CommandBufferBeginInfo {
            p_inheritance_info,
            flags: vk::CommandBufferUsageFlags::from_raw(info.flags.0),
            p_next: info.p_next.use_p_next(),
            ..Default::default()
        };
        unsafe {
            (self.device.fns.v1_0.begin_command_buffer)(
                self.get_command_buffer(), 
                &begin_info
            ).result().map_err(|err|{ VulkanError::from(err) })
        }
    }
    #[inline]
    pub fn end(&self) -> Result<(), VulkanError> {
        unsafe { (self.device.fns.v1_0.end_command_buffer)(
                    self.get_command_buffer()
                ).result().map_err(VulkanError::from)
            }
    }
    #[inline]
    pub fn reset(&self, flags: CommandBufferResetFlags) -> Result<(), VulkanError> {
        unsafe { self.device.device.reset_command_buffer(self.command_buffer, vk::CommandBufferResetFlags::from_raw(flags.0)).map_err(VulkanError::from) }
    }
    #[inline]
    pub fn push_constants<P>(&self, layout: vk::PipelineLayout, stage_flags: ShaderStageFlags, offset: u32, constant: &P) {
        self.device.push_constants(self.command_buffer, layout, stage_flags, offset, constant)
    }
    pub fn set_viewport(&self, first_viewport: u32, viewports: &[vk::Viewport]) {
        unsafe { self.device.device.cmd_set_viewport(self.command_buffer, first_viewport, viewports) }
    }
    pub fn set_scissor(&self, first_scissor: u32, scissors: &[vk::Rect2D]) {
        unsafe { self.device.device.cmd_set_scissor(self.command_buffer, first_scissor, scissors) }
    }
    pub fn copy_buffer(&self, src: vk::Buffer, dst: vk::Buffer, regions: &[BufferCopy]) {
        unsafe { 
            self.device.device.cmd_copy_buffer(
                self.command_buffer, 
                src, 
                dst, 
                std::mem::transmute_copy::<&[BufferCopy], &[vk::BufferCopy]>(&regions)
            ) 
        };
    }
    pub fn copy_image(&self, src: vk::Image, dst: vk::Image, src_layout: ImageLayout, dst_layout: ImageLayout, regions: &[ImageCopy]) {
        unsafe {
            self.device.device.cmd_copy_image(
                self.command_buffer, 
                src, 
                vk::ImageLayout::from_raw(src_layout.0), 
                dst, 
                vk::ImageLayout::from_raw(dst_layout.0), 
                std::mem::transmute_copy::<&[ImageCopy], &[vk::ImageCopy]>(&regions),
            );
        }
    }
    pub fn buffer_to_image(&self, src: vk::Buffer, dst: vk::Image, layout: ImageLayout, regions: &[BufferImageCopy]) {
        unsafe {
            self.device.device.cmd_copy_buffer_to_image(
                self.command_buffer, 
                src, 
                dst, 
                vk::ImageLayout::from_raw(layout.0), 
                std::mem::transmute_copy::<&[BufferImageCopy], &[vk::BufferImageCopy]>(&regions)
            );
        }
    }
    // vkCmdCopyBufferToImage, 
    // vkCmdCopyBufferToImage2, 
    // vkCmdCopyBufferToImage2KHR, 
    // vkCmdCopyImageToBuffer, 
    // vkCmdCopyImageToBuffer2, 
    // vkCmdCopyImageToBuffer2KHR, 
    pub fn begin_render_pass(&self, info: vk::RenderPassBeginInfo, subpass_contents: SubpassContents) -> Result<(), VulkanError> {
        // let begin_info: vk::RenderPassBeginInfo = info.into();
        if subpass_contents == SubpassContents::INLINE_AND_SECONDARY_COMMAND_BUFFERS && !self.device.enabled_extensions.ext_nested_command_buffer {
            return Err(VulkanError::ExtensionNotPresent);
        }
        let contents: vk::SubpassContents = subpass_contents.into();
        unsafe {
            (self.device.fns.v1_0.cmd_begin_render_pass)(
                self.get_command_buffer(),
                &info,
                contents
            );
        }
        Ok(())
    }
    pub fn end_render_pass(&self) {
        unsafe { self.device.device.cmd_end_render_pass(self.command_buffer) };
    }
    pub fn pipeline_barrier(&self, 
        src_stage_mask: PipelineStageFlags, 
        dst_stage_mask: PipelineStageFlags, 
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier]) {
        let memory_barriers = memory_barriers.iter().map(|val|{ vk::MemoryBarrier::from(val) }).collect::<smallvec::SmallVec<[_; 4]>>();
        let buffer_memory_barriers = buffer_memory_barriers.iter().map(|val|{ vk::BufferMemoryBarrier::from(val) }).collect::<smallvec::SmallVec<[_; 4]>>();
        let image_memory_barriers = image_memory_barriers.iter().map(|val|{ vk::ImageMemoryBarrier::from(val) }).collect::<smallvec::SmallVec<[_; 4]>>();
        unsafe {
            self.device.device.cmd_pipeline_barrier(
                self.command_buffer, 
                vk::PipelineStageFlags::from_raw(src_stage_mask.0), 
                vk::PipelineStageFlags::from_raw(dst_stage_mask.0), 
                vk::DependencyFlags::from_raw(dependency_flags.0), 
                &memory_barriers, 
                &buffer_memory_barriers, 
                &image_memory_barriers
            )
        }
    }
    pub unsafe fn bind_vertex_nfptrs(&self, first_binding: u32, nfptr: &[NfPtr]) {
        let mut buffers = vec![];
        let mut offsets = vec![];
        for ptr in nfptr {
            buffers.push(ptr.buffer());
            offsets.push(ptr.offset() as u64);
        }
        self.bind_vertex_buffers(0, &buffers, &offsets);
    }
    pub unsafe fn bind_index_nfptr(&self, first_binding: u32, nfptr: NfPtr, index_type: vk::IndexType) {
        self.bind_index_buffers(nfptr.buffer(), nfptr.offset() as u64, index_type)
    }
    pub fn bind_vertex_buffers(&self, first_binding: u32, buffers: &[vk::Buffer], offsets: &[vk::DeviceSize]) {
        unsafe { self.device.device.cmd_bind_vertex_buffers(self.command_buffer, first_binding, buffers, offsets) }
    }
    pub fn bind_index_buffers(&self, buffer: vk::Buffer, offset: vk::DeviceSize, index_type: vk::IndexType) {
        unsafe { self.device.device.cmd_bind_index_buffer(self.command_buffer, buffer, offset, index_type); }
    }
    pub fn draw(&self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
        unsafe { self.device.device.cmd_draw(self.command_buffer, vertex_count, instance_count, first_vertex, first_instance) };
    }
    pub fn draw_indexed(&self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
        unsafe { self.device.device.cmd_draw_indexed(self.command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) };
        
    }
}
pub struct CommandPool {
    device: Arc<LogicalDevice>,
    pub(crate) command_pool: vk::CommandPool
}
impl CommandPool {
    pub fn new(device: Arc<LogicalDevice>, flags: CommandPoolCreateFlags, family_index: u32) -> Result<Self, VulkanError> {
        let create_info = vk::CommandPoolCreateInfo {
            queue_family_index: family_index,
            flags: vk::CommandPoolCreateFlags::from_raw(flags.0),
            ..Default::default()
        };
        let mut output = MaybeUninit::uninit();
        unsafe { (device.fns.v1_0.create_command_pool)(
            device.handle(),
            &create_info,
            std::ptr::null(),
            output.as_mut_ptr(),
        ).result().map_err(|err|{VulkanError::from(err)})?};
        unsafe {
            Ok(Self { 
                device,
                command_pool: output.assume_init() 
            })
        }
    }
    pub unsafe fn allocate_command_buffers(&self, level: CommandBufferLevel, count: u32) -> Result<impl ExactSizeIterator<Item = CommandPoolAllocation>, VulkanError> {
        let out = if count == 0 {
            vec![]
        } else {
            let allocate_info = ash::vk::CommandBufferAllocateInfo {
                command_pool: self.command_pool,
                level: vk::CommandBufferLevel::from_raw(level.0),
                command_buffer_count: count,
                ..Default::default()
            };

            unsafe {
                let mut out = Vec::with_capacity(count as usize);
                (self.device.fns.v1_0.allocate_command_buffers)(
                    self.device.handle(),
                    &allocate_info,
                    out.as_mut_ptr(),
                )
                .result()
                .map_err(VulkanError::from)?;
                out.set_len(count as usize);
                out
            }
        };
        let device = self.device.clone();

        Ok(out.into_iter().map(move |command_buffer| CommandPoolAllocation {
            command_buffer,
            device: device.clone(),
            level,
        }))
    }
    pub unsafe fn free_command_buffers(&self, command_buffers: &[vk::CommandBuffer]) {
        (self.device.fns.v1_0.free_command_buffers)(
            self.device.handle(),
            self.command_pool,
            command_buffers.len() as u32,
            command_buffers.as_ptr()
        );
    }
}
impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_command_pool(self.command_pool, None) };
    }
}
    // fn begin_render_pass2(&self) {
// 
    // } 
    // vkCmdBeginRendering, 
    // vkCmdBeginRenderingKHR, 
    // vkCmdBeginTransformFeedbackEXT, 
    // vkCmdBeginVideoCodingKHR, 
    // vkCmdBindDescriptorBufferEmbeddedSamplers2EXT, 
    // vkCmdBindDescriptorBufferEmbeddedSamplersEXT, 
    // vkCmdBindDescriptorBuffersEXT, 
    // vkCmdBindDescriptorSets, 
    // vkCmdBindDescriptorSets2KHR, 
    // vkCmdBindIndexBuffer, 
    // vkCmdBindIndexBuffer2KHR, 
    // vkCmdBindInvocationMaskHUAWEI, 
    // vkCmdBindPipeline, 
    // vkCmdBindPipelineShaderGroupNV, 
    // vkCmdBindShadersEXT, 
    // vkCmdBindShadingRateImageNV, 
    // vkCmdBindTransformFeedbackBuffersEXT, 
    // vkCmdBindVertexBuffers, 
    // vkCmdBindVertexBuffers2, 
    // vkCmdBindVertexBuffers2EXT, 
    // vkCmdBlitImage, 
    // vkCmdBlitImage2, 
    // vkCmdBlitImage2KHR, 
    // vkCmdBuildAccelerationStructureNV, 
    // vkCmdBuildAccelerationStructuresIndirectKHR, 
    // vkCmdBuildAccelerationStructuresKHR, 
    // vkCmdBuildMicromapsEXT, 
    // vkCmdClearAttachments, 
    // vkCmdClearColorImage, 
    // vkCmdClearDepthStencilImage, 
    // vkCmdControlVideoCodingKHR, 
    // vkCmdCopyAccelerationStructureKHR, 
    // vkCmdCopyAccelerationStructureNV, 
    // vkCmdCopyAccelerationStructureToMemoryKHR, 
    // vkCmdCopyBuffer, 
    // vkCmdCopyBuffer2, 
    // vkCmdCopyBuffer2KHR, 
    // vkCmdCopyBufferToImage, 
    // vkCmdCopyBufferToImage2, 
    // vkCmdCopyBufferToImage2KHR, 
    // vkCmdCopyImage, 
    // vkCmdCopyImage2, 
    // vkCmdCopyImage2KHR, 
    // vkCmdCopyImageToBuffer, 
    // vkCmdCopyImageToBuffer2, 
    // vkCmdCopyImageToBuffer2KHR, 
    // vkCmdCopyMemoryIndirectNV, 
    // vkCmdCopyMemoryToAccelerationStructureKHR, 
    // vkCmdCopyMemoryToImageIndirectNV, 
    // vkCmdCopyMemoryToMicromapEXT, 
    // vkCmdCopyMicromapEXT, 
    // vkCmdCopyMicromapToMemoryEXT, 
    // vkCmdCopyQueryPoolResults, 
    // vkCmdCuLaunchKernelNVX, 
    // vkCmdCudaLaunchKernelNV, 
    // vkCmdDebugMarkerBeginEXT, 
    // vkCmdDebugMarkerEndEXT, 
    // vkCmdDebugMarkerInsertEXT, 
    // vkCmdDecodeVideoKHR, 
    // vkCmdDecompressMemoryIndirectCountNV, 
    // vkCmdDecompressMemoryNV, 
    // vkCmdDispatch, 
    // vkCmdDispatchBase, 
    // vkCmdDispatchBaseKHR, 
    // vkCmdDispatchGraphAMDX, 
    // vkCmdDispatchGraphIndirectAMDX, 
    // vkCmdDispatchGraphIndirectCountAMDX, 
    // vkCmdDispatchIndirect, 
    // vkCmdDraw, 
    // vkCmdDrawClusterHUAWEI, 
    // vkCmdDrawClusterIndirectHUAWEI, 
    // vkCmdDrawIndexed, 
    // vkCmdDrawIndexedIndirect, 
    // vkCmdDrawIndexedIndirectCount, 
    // vkCmdDrawIndexedIndirectCountAMD, 
    // vkCmdDrawIndexedIndirectCountKHR, 
    // vkCmdDrawIndirect, 
    // vkCmdDrawIndirectByteCountEXT, 
    // vkCmdDrawIndirectCount, 
    // vkCmdDrawIndirectCountAMD, 
    // vkCmdDrawIndirectCountKHR, 
    // vkCmdDrawMeshTasksEXT, 
    // vkCmdDrawMeshTasksIndirectCountEXT, 
    // vkCmdDrawMeshTasksIndirectCountNV, 
    // vkCmdDrawMeshTasksIndirectEXT, 
    // vkCmdDrawMeshTasksIndirectNV, 
    // vkCmdDrawMeshTasksNV, 
    // vkCmdDrawMultiEXT, 
    // vkCmdDrawMultiIndexedEXT, 
    // vkCmdEncodeVideoKHR, 
    // vkCmdEndConditionalRenderingEXT, 
    // vkCmdEndDebugUtilsLabelEXT, 
    // vkCmdEndQuery, 
    // vkCmdEndQueryIndexedEXT, 
    // vkCmdEndRenderPass, 
    // vkCmdEndRenderPass2, 
    // vkCmdEndRenderPass2KHR, 
    // vkCmdEndRendering, 
    // vkCmdEndRenderingKHR, 
    // vkCmdEndTransformFeedbackEXT, 
    // vkCmdEndVideoCodingKHR, 
    // vkCmdExecuteCommands, 
    // vkCmdExecuteGeneratedCommandsNV, 
    // vkCmdFillBuffer, 
    // vkCmdInitializeGraphScratchMemoryAMDX, 
    // vkCmdInsertDebugUtilsLabelEXT, 
    // vkCmdNextSubpass, 
    // vkCmdNextSubpass2, 
    // vkCmdNextSubpass2KHR, 
    // vkCmdOpticalFlowExecuteNV, 
    // vkCmdPipelineBarrier, 
    // vkCmdPipelineBarrier2, 
    // vkCmdPipelineBarrier2KHR, 
    // vkCmdPreprocessGeneratedCommandsNV, 
    // vkCmdPushConstants, 
    // vkCmdPushConstants2KHR, 
    // vkCmdPushDescriptorSet2KHR, 
    // vkCmdPushDescriptorSetKHR, 
    // vkCmdPushDescriptorSetWithTemplate2KHR, 
    // vkCmdPushDescriptorSetWithTemplateKHR, 
    // vkCmdResetEvent, 
    // vkCmdResetEvent2, 
    // vkCmdResetEvent2KHR, 
    // vkCmdResetQueryPool, 
    // vkCmdResolveImage, 
    // vkCmdResolveImage2, 
    // vkCmdResolveImage2KHR, 
    // vkCmdSetAlphaToCoverageEnableEXT, 
    // vkCmdSetAlphaToOneEnableEXT, 
    // vkCmdSetAttachmentFeedbackLoopEnableEXT, 
    // vkCmdSetBlendConstants, 
    // vkCmdSetCheckpointNV, 
    // vkCmdSetCoarseSampleOrderNV, 
    // vkCmdSetColorBlendAdvancedEXT, 
    // vkCmdSetColorBlendEnableEXT, 
    // vkCmdSetColorBlendEquationEXT, 
    // vkCmdSetColorWriteEnableEXT, 
    // vkCmdSetColorWriteMaskEXT, 
    // vkCmdSetConservativeRasterizationModeEXT, 
    // vkCmdSetCoverageModulationModeNV, 
    // vkCmdSetCoverageModulationTableEnableNV, 
    // vkCmdSetCoverageModulationTableNV, 
    // vkCmdSetCoverageReductionModeNV, 
    // vkCmdSetCoverageToColorEnableNV, 
    // vkCmdSetCoverageToColorLocationNV, 
    // vkCmdSetCullMode, 
    // vkCmdSetCullModeEXT, 
    // vkCmdSetDepthBias, 
    // vkCmdSetDepthBias2EXT, 
    // vkCmdSetDepthBiasEnable, 
    // vkCmdSetDepthBiasEnableEXT, 
    // vkCmdSetDepthBounds, 
    // vkCmdSetDepthBoundsTestEnable, 
    // vkCmdSetDepthBoundsTestEnableEXT, 
    // vkCmdSetDepthClampEnableEXT, 
    // vkCmdSetDepthClipEnableEXT, 
    // vkCmdSetDepthClipNegativeOneToOneEXT, 
    // vkCmdSetDepthCompareOp, 
    // vkCmdSetDepthCompareOpEXT, 
    // vkCmdSetDepthTestEnable, 
    // vkCmdSetDepthTestEnableEXT, 
    // vkCmdSetDepthWriteEnable, 
    // vkCmdSetDepthWriteEnableEXT, 
    // vkCmdSetDescriptorBufferOffsets2EXT, 
    // vkCmdSetDescriptorBufferOffsetsEXT, 
    // vkCmdSetDeviceMask, 
    // vkCmdSetDeviceMaskKHR, 
    // vkCmdSetDiscardRectangleEXT, 
    // vkCmdSetDiscardRectangleEnableEXT, 
    // vkCmdSetDiscardRectangleModeEXT, 
    // vkCmdSetEvent, 
    // vkCmdSetEvent2, 
    // vkCmdSetEvent2KHR, 
    // vkCmdSetExclusiveScissorEnableNV, 
    // vkCmdSetExclusiveScissorNV, 
    // vkCmdSetExtraPrimitiveOverestimationSizeEXT, 
    // vkCmdSetFragmentShadingRateEnumNV, 
    // vkCmdSetFragmentShadingRateKHR, 
    // vkCmdSetFrontFace, 
    // vkCmdSetFrontFaceEXT, 
    // vkCmdSetLineRasterizationModeEXT, 
    // vkCmdSetLineStippleEXT, 
    // vkCmdSetLineStippleEnableEXT, 
    // vkCmdSetLineStippleKHR, 
    // vkCmdSetLineWidth, 
    // vkCmdSetLogicOpEXT, 
    // vkCmdSetLogicOpEnableEXT, 
    // vkCmdSetPatchControlPointsEXT, 
    // vkCmdSetPerformanceMarkerINTEL, 
    // vkCmdSetPerformanceOverrideINTEL, 
    // vkCmdSetPerformanceStreamMarkerINTEL, 
    // vkCmdSetPolygonModeEXT, 
    // vkCmdSetPrimitiveRestartEnable, 
    // vkCmdSetPrimitiveRestartEnableEXT, 
    // vkCmdSetPrimitiveTopology, 
    // vkCmdSetPrimitiveTopologyEXT, 
    // vkCmdSetProvokingVertexModeEXT, 
    // vkCmdSetRasterizationSamplesEXT, 
    // vkCmdSetRasterizationStreamEXT, 
    // vkCmdSetRasterizerDiscardEnable, 
    // vkCmdSetRasterizerDiscardEnableEXT, 
    // vkCmdSetRayTracingPipelineStackSizeKHR, 
    // vkCmdSetRenderingAttachmentLocationsKHR, 
    // vkCmdSetRenderingInputAttachmentIndicesKHR, 
    // vkCmdSetRepresentativeFragmentTestEnableNV, 
    // vkCmdSetSampleLocationsEXT, 
    // vkCmdSetSampleLocationsEnableEXT, 
    // vkCmdSetSampleMaskEXT, 
    // vkCmdSetScissor, 
    // vkCmdSetScissorWithCount, 
    // vkCmdSetScissorWithCountEXT, 
    // vkCmdSetShadingRateImageEnableNV, 
    // vkCmdSetStencilCompareMask, 
    // vkCmdSetStencilOp, 
    // vkCmdSetStencilOpEXT, 
    // vkCmdSetStencilReference, 
    // vkCmdSetStencilTestEnable, 
    // vkCmdSetStencilTestEnableEXT, 
    // vkCmdSetStencilWriteMask, 
    // vkCmdSetTessellationDomainOriginEXT, 
    // vkCmdSetVertexInputEXT, 
    // vkCmdSetViewport, 
    // vkCmdSetViewportShadingRatePaletteNV, 
    // vkCmdSetViewportSwizzleNV, 
    // vkCmdSetViewportWScalingEnableNV, 
    // vkCmdSetViewportWScalingNV, 
    // vkCmdSetViewportWithCount, 
    // vkCmdSetViewportWithCountEXT, 
    // vkCmdSubpassShadingHUAWEI, 
    // vkCmdTraceRaysIndirect2KHR, 
    // vkCmdTraceRaysIndirectKHR, 
    // vkCmdTraceRaysKHR, 
    // vkCmdTraceRaysNV, 
    // vkCmdUpdateBuffer, 
    // vkCmdUpdatePipelineIndirectBufferNV, 
    // vkCmdWaitEvents, 
    // vkCmdWaitEvents2, 
    // vkCmdWaitEvents2KHR, 
    // vkCmdWriteAccelerationStructuresPropertiesKHR, 
    // vkCmdWriteAccelerationStructuresPropertiesNV, 
    // vkCmdWriteBufferMarker2AMD, 
    // vkCmdWriteBufferMarkerAMD, 
    // vkCmdWriteMicromapsPropertiesEXT, 
    // vkCmdWriteTimestamp, 
    // vkCmdWriteTimestamp2, 
    // vkCmdWriteTimestamp2KHR, 
    // vkEndCommandBuffer, 
    // vkFreeCommandBuffers, 
    // vkResetCommandBuffer
