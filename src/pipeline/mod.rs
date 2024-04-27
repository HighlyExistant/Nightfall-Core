use ash::vk;

pub mod compute;
pub mod graphics;
pub mod shader;
pub mod cache;
pub mod layout;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html>"]
pub struct PipelineBindPoint(pub(crate) i32);
impl PipelineBindPoint {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineBindPoint {
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1_000_369_003);
    pub const RAY_TRACING: Self = Self(1_000_165_000);
}

pub trait VulkanPipeline {
    fn pipeline_handle(&self) -> vk::Pipeline;
}