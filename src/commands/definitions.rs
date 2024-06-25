use std::marker::PhantomData;

use ash::{vk::{self, ClearValue, Framebuffer, Rect2D, RenderPass}, vk_bitflags_wrapped};

use crate::{image::ImageSubresourceLayers, PNext};

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearDepthStencilValue.html>"]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ClearColorValue {
    F32([f32; 4]),
    I32([i32; 4]),
    U32([u32; 4])
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ClearValues {
    Color(ClearColorValue),
    Depth(ClearDepthStencilValue),
}
impl From<&ClearValues> for vk::ClearValue {
    fn from(value: &ClearValues) -> Self {
        match value {
            ClearValues::Color(color) => {
                vk::ClearValue {
                    color: match color {
                        ClearColorValue::F32(float32) => vk::ClearColorValue { float32: *float32 },
                        ClearColorValue::I32(int32) => vk::ClearColorValue { int32: *int32 },
                        ClearColorValue::U32(uint32) => vk::ClearColorValue { uint32: *uint32 },
                    }
                }
            }
            ClearValues::Depth(depth) => {
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: depth.depth,
                        stencil: depth.stencil,
                    }
                }
            }
        }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassBeginInfo.html>"]
pub struct RenderPassBeginInfo {
    pub p_next: Option<PNext>,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub offset: [i32; 2],
    pub extent: [u32; 2],
    pub clear_value_count: u32,
    pub p_clear_values: Vec<ClearValues>,
}
impl From<RenderPassBeginInfo> for vk::RenderPassBeginInfo {
    fn from(value: RenderPassBeginInfo) -> Self {
        let clear_values = value.p_clear_values.as_ptr();
        vk::RenderPassBeginInfo {
            framebuffer: value.framebuffer,
            clear_value_count: value.clear_value_count,
            p_clear_values: clear_values.cast(),
            render_area: Rect2D {
                offset: vk::Offset2D { x: value.offset[0], y: value.offset[1] },
                extent: vk::Extent2D { width: value.extent[0], height: value.extent[1] },
            },
            render_pass: value.render_pass,
            p_next: value.p_next.map(|v|v.use_p_next()).unwrap_or(std::ptr::null_mut()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassContents.html>"]
pub struct SubpassContents(pub(crate) i32);
impl SubpassContents {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SubpassContents {
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS: Self = Self(1000451000);

}
impl From<SubpassContents> for vk::SubpassContents {
    fn from(value: SubpassContents) -> Self {
        vk::SubpassContents::from_raw(value.as_raw())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html>"]
pub struct CommandBufferUsageFlags(pub(crate) u32);
vk_bitflags_wrapped!(CommandBufferUsageFlags, u32);
impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT: Self = Self(0b1);
    pub const RENDER_PASS_CONTINUE: Self = Self(0b10);
    #[doc = "Command buffer may be submitted/executed more than once simultaneously"]
    pub const SIMULTANEOUS_USE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html>"]
pub struct QueryPipelineStatisticFlags(pub(crate) u32);
vk_bitflags_wrapped!(QueryPipelineStatisticFlags, u32);
impl QueryPipelineStatisticFlags {
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(0b1);
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(0b10);
    #[doc = "Optional"]
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(0b100);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(0b1000);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(0b1_0000);
    #[doc = "Optional"]
    pub const CLIPPING_INVOCATIONS: Self = Self(0b10_0000);
    #[doc = "Optional"]
    pub const CLIPPING_PRIMITIVES: Self = Self(0b100_0000);
    #[doc = "Optional"]
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(0b1000_0000);
    #[doc = "Optional"]
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(0b1_0000_0000);
    #[doc = "Optional"]
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(0b10_0000_0000);
    #[doc = "Optional"]
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(0b100_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html>"]
pub struct QueryControlFlags(pub(crate) u32);
vk_bitflags_wrapped!(QueryControlFlags, u32);
impl QueryControlFlags {
    #[doc = "Require precise results to be collected by the query"]
    pub const PRECISE: Self = Self(0b1);
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceInfo.html>"]
pub struct CommandBufferInheritanceInfo {
    pub p_next: PNext,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: bool,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
impl From<&CommandBufferInheritanceInfo> for vk::CommandBufferInheritanceInfo {
    fn from(value: &CommandBufferInheritanceInfo) -> Self {
        vk::CommandBufferInheritanceInfo {
            p_next: value.p_next.use_p_next(),
            render_pass: value.render_pass,
            subpass: value.subpass,
            framebuffer: value.framebuffer,
            occlusion_query_enable: value.occlusion_query_enable as u32,
            pipeline_statistics: vk::QueryPipelineStatisticFlags::from_raw(value.pipeline_statistics.0),
            query_flags: vk::QueryControlFlags::from_raw(value.query_flags.0),
            ..Default::default()
        }
    }
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferBeginInfo.html>"]
pub struct CommandBufferBeginInfo<'a> {
    pub p_next: PNext,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: Option<&'a CommandBufferInheritanceInfo>,
}
impl<'a> CommandBufferBeginInfo<'a> {
    pub const SINGLE_SUBMIT: Self =  CommandBufferBeginInfo { p_next: PNext::new(), flags: CommandBufferUsageFlags::ONE_TIME_SUBMIT, p_inheritance_info: None };
    pub const RENDER_PASS_CONTINUE: Self =  CommandBufferBeginInfo { p_next: PNext::new(), flags: CommandBufferUsageFlags::RENDER_PASS_CONTINUE, p_inheritance_info: None };
    pub const SIMULTANEOUS: Self =  CommandBufferBeginInfo { p_next: PNext::new(), flags: CommandBufferUsageFlags::SIMULTANEOUS_USE, p_inheritance_info: None };
}
impl<'a> Default for CommandBufferBeginInfo<'a> {
    fn default() -> Self {
        Self {
            p_next: PNext::default(),
            flags: CommandBufferUsageFlags::empty(),
            p_inheritance_info: None
        }
    }
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCopy.html>"]
pub struct BufferCopy {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}
impl From<BufferCopy> for vk::BufferCopy {
    fn from(value: BufferCopy) -> Self {
        unsafe { std::mem::transmute::<BufferCopy, vk::BufferCopy>(value) }
    }
}
impl BufferCopy {
    pub fn new(src_offset: u64, dst_offset: u64, size: u64) -> Self {
        Self { src_offset, dst_offset, size }
    }
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCopy.html>"]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: [i32; 3],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: [i32; 3],
    pub extent: [u32; 3],
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy.html>"]
pub struct BufferImageCopy {
    pub buffer_offset: u64,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: [i32; 3],
    pub image_extent: [u32; 3],
}