use ash::{vk, vk_bitflags_wrapped};
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html>"]
pub struct PipelineStageFlags(pub(crate) u32);
vk_bitflags_wrapped!(PipelineStageFlags, u32);
impl PipelineStageFlags {
    #[doc = "Before subsequent commands are processed"]
    pub const TOP_OF_PIPE: Self = Self(0b1);
    #[doc = "Draw/DispatchIndirect command fetch"]
    pub const DRAW_INDIRECT: Self = Self(0b10);
    #[doc = "Vertex/index fetch"]
    pub const VERTEX_INPUT: Self = Self(0b100);
    #[doc = "Vertex shading"]
    pub const VERTEX_SHADER: Self = Self(0b1000);
    #[doc = "Tessellation control shading"]
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0b1_0000);
    #[doc = "Tessellation evaluation shading"]
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0b10_0000);
    #[doc = "Geometry shading"]
    pub const GEOMETRY_SHADER: Self = Self(0b100_0000);
    #[doc = "Fragment shading"]
    pub const FRAGMENT_SHADER: Self = Self(0b1000_0000);
    #[doc = "Early fragment (depth and stencil) tests"]
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0b1_0000_0000);
    #[doc = "Late fragment (depth and stencil) tests"]
    pub const LATE_FRAGMENT_TESTS: Self = Self(0b10_0000_0000);
    #[doc = "Color attachment writes"]
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(0b100_0000_0000);
    #[doc = "Compute shading"]
    pub const COMPUTE_SHADER: Self = Self(0b1000_0000_0000);
    #[doc = "Transfer/copy operations"]
    pub const TRANSFER: Self = Self(0b1_0000_0000_0000);
    #[doc = "After previous commands have completed"]
    pub const BOTTOM_OF_PIPE: Self = Self(0b10_0000_0000_0000);
    #[doc = "Indicates host (CPU) is a source/sink of the dependency"]
    pub const HOST: Self = Self(0b100_0000_0000_0000);
    #[doc = "All stages of the graphics pipeline"]
    pub const ALL_GRAPHICS: Self = Self(0b1000_0000_0000_0000);
    #[doc = "All stages supported on the queue"]
    pub const ALL_COMMANDS: Self = Self(0b1_0000_0000_0000_0000);
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html>"]
pub struct ImageUsageFlags(pub(crate) u32);
vk_bitflags_wrapped!(ImageUsageFlags, u32);
impl ImageUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0b10);
    #[doc = "Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED: Self = Self(0b100);
    #[doc = "Can be used as storage image (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE: Self = Self(0b1000);
    #[doc = "Can be used as framebuffer color attachment"]
    pub const COLOR_ATTACHMENT: Self = Self(0b1_0000);
    #[doc = "Can be used as framebuffer depth/stencil attachment"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0b10_0000);
    #[doc = "Image data not needed outside of rendering"]
    pub const TRANSIENT_ATTACHMENT: Self = Self(0b100_0000);
    #[doc = "Can be used as framebuffer input attachment"]
    pub const INPUT_ATTACHMENT: Self = Self(0b1000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html>"]
pub struct ImageAspectFlags(pub(crate) u32);
vk_bitflags_wrapped!(ImageAspectFlags, u32);
impl ImageAspectFlags {
    pub const COLOR: Self = Self(0b1);
    pub const DEPTH: Self = Self(0b10);
    pub const STENCIL: Self = Self(0b100);
    pub const METADATA: Self = Self(0b1000);
}
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html>"]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html>"]
pub struct ImageLayout(pub(crate) i32);
impl ImageLayout {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageLayout {
    #[doc = "Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)"]
    pub const UNDEFINED: Self = Self(0);
    #[doc = "General layout when image can be used for any kind of access"]
    pub const GENERAL: Self = Self(1);
    #[doc = "Optimal layout when image is only used for color attachment read/write"]
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    #[doc = "Optimal layout when image is only used for depth/stencil attachment read/write"]
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    #[doc = "Optimal layout when image is used for read only depth/stencil attachment and shader access"]
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    #[doc = "Optimal layout when image is used for read only shader access"]
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    #[doc = "Optimal layout when image is used only as source of transfer operations"]
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    #[doc = "Optimal layout when image is used only as destination of transfer operations"]
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    #[doc = "Initial layout used when the data is populated by the CPU"]
    pub const PREINITIALIZED: Self = Self(8);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(i32)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageType.html>"]
pub enum ImageType {
    #[default]
    Type1D = 0,
    Type2D = 1,
    Type3D = 2,
}
impl ImageType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        match x {
            0 => ImageType::Type1D,
            1 => ImageType::Type2D,
            2 => ImageType::Type3D,
            _ => panic!("ImageType must be either 0, 1 or 2")
        }
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html>"]
pub struct ImageCreateFlags(pub(crate) u32);
vk_bitflags_wrapped!(ImageCreateFlags, u32);
impl ImageCreateFlags {
    #[doc = "Image should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0b1);
    #[doc = "Image should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0b10);
    #[doc = "Image should support constant data access to physical memory ranges mapped into multiple locations of sparse images"]
    pub const SPARSE_ALIASED: Self = Self(0b100);
    #[doc = "Allows image views to have different format than the base image"]
    pub const MUTABLE_FORMAT: Self = Self(0b1000);
    #[doc = "Allows creating image views with cube type from the created image"]
    pub const CUBE_COMPATIBLE: Self = Self(0b1_0000);
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html>"]
pub struct SampleCountFlags(pub(crate) u32);
vk_bitflags_wrapped!(SampleCountFlags, u32);
impl SampleCountFlags {
    #[doc = "Sample count 1 supported"]
    pub const TYPE_1: Self = Self(0b1);
    #[doc = "Sample count 2 supported"]
    pub const TYPE_2: Self = Self(0b10);
    #[doc = "Sample count 4 supported"]
    pub const TYPE_4: Self = Self(0b100);
    #[doc = "Sample count 8 supported"]
    pub const TYPE_8: Self = Self(0b1000);
    #[doc = "Sample count 16 supported"]
    pub const TYPE_16: Self = Self(0b1_0000);
    #[doc = "Sample count 32 supported"]
    pub const TYPE_32: Self = Self(0b10_0000);
    #[doc = "Sample count 64 supported"]
    pub const TYPE_64: Self = Self(0b100_0000);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html>"]
pub struct SharingMode(pub(crate) i32);
impl SharingMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SharingMode {
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html>"]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl From<ImageSubresourceRange> for vk::ImageSubresourceRange {
    fn from(value: ImageSubresourceRange) -> Self {
        Self {
            aspect_mask: vk::ImageAspectFlags::from_raw(value.aspect_mask.0),
            base_mip_level: value.base_mip_level,
            level_count: value.level_count,
            base_array_layer: value.base_array_layer,
            layer_count: value.layer_count,
            ..Default::default()
        }
    }
}