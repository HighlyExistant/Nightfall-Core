use std::{cell::RefCell, rc::Rc, sync::Arc};

use ash::{vk, vk_bitflags_wrapped};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html>"]
pub struct ShaderStageFlags(pub(crate) u32);
vk_bitflags_wrapped!(ShaderStageFlags, u32);
impl ShaderStageFlags {
    pub const VERTEX: Self = Self(0b1);
    pub const TESSELLATION_CONTROL: Self = Self(0b10);
    pub const TESSELLATION_EVALUATION: Self = Self(0b100);
    pub const GEOMETRY: Self = Self(0b1000);
    pub const FRAGMENT: Self = Self(0b1_0000);
    pub const COMPUTE: Self = Self(0b10_0000);
    pub const ALL_GRAPHICS: Self = Self(0x0000_001F);
    pub const ALL: Self = Self(0x7FFF_FFFF);
}

use crate::{device::LogicalDevice, error::VulkanError};
pub trait HasShaderStages {
    fn stages(&self) -> impl ExactSizeIterator<Item = &vk::PipelineShaderStageCreateInfo>;
}
pub struct VulkanShadersIter<'a> {
    shaders: &'a Vec<vk::PipelineShaderStageCreateInfo>,
    len: usize,
}
impl<'a> VulkanShadersIter<'a>  {
    pub fn new(shaders: &'a Vec<vk::PipelineShaderStageCreateInfo>) -> Self {
        Self { shaders, len: shaders.len() }
    }
}
impl<'a> Iterator for VulkanShadersIter<'a> {
    type Item = &'a vk::PipelineShaderStageCreateInfo;
    fn next(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        Some(&self.shaders[self.len])
    }
}
impl<'a> ExactSizeIterator for VulkanShadersIter<'a> {
    fn len(&self) -> usize {
        self.len
    }
}
pub struct VulkanShaderIter<'a> {
    shader: Option<&'a vk::PipelineShaderStageCreateInfo>,
}
impl<'a> VulkanShaderIter<'a>  {
    pub fn new(shader: &'a vk::PipelineShaderStageCreateInfo) -> Self {
        Self { shader: Some(shader) }
    }
}
impl<'a> ExactSizeIterator for VulkanShaderIter<'a> {
    fn len(&self) -> usize {
        self.shader.is_none() as usize
    }
}
impl<'a> Iterator for VulkanShaderIter<'a> {
    type Item = &'a vk::PipelineShaderStageCreateInfo;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.shader?;
        self.shader = None;
        Some(next)
    }
}
#[derive(Clone, Debug)]
pub struct ShaderCreateInfo<'a> {
    pub entry: &'a str, 
    pub stage: ShaderStageFlags, 
    pub data: &'a [u8],
}
impl<'a> Default for ShaderCreateInfo<'a> {
    fn default() -> Self {
        Self {
            entry: "main",
            data: &[],
            stage: ShaderStageFlags::empty()
        }
    }
}
pub struct Shader {
    pub(crate) device: Arc<LogicalDevice>,
    pub(crate) stage: vk::PipelineShaderStageCreateInfo,
}
impl Shader {
    pub fn new(device: Arc<LogicalDevice>, ci: ShaderCreateInfo) -> Result<Arc<Self>, VulkanError> {
        let create_info = vk::ShaderModuleCreateInfo {
            code_size: ci.data.len(),
            p_code: ci.data.as_ptr() as *const u32,
            ..Default::default()
        };
        let module = unsafe { device.device.create_shader_module(&create_info, None).map_err(|err|{VulkanError::from(err)})? };
        let stage = vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::from_raw(ci.stage.0),
            p_name: ci.entry.as_bytes().as_ptr() as *const i8,
            module,
            ..Default::default()
        };
        Ok(Arc::new(Self { device, stage }))
    }
    pub fn stage(&self) -> &vk::PipelineShaderStageCreateInfo {
        &self.stage
    }
}
impl HasShaderStages for Shader {
    fn stages(&self) -> impl ExactSizeIterator<Item = &vk::PipelineShaderStageCreateInfo> {
        VulkanShaderIter::new(&self.stage)
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_shader_module(self.stage.module, None) };
    }
}
// pub struct Shaders {
//     pub(crate) device: Arc<LogicalDevice>,
//     pub(crate) stages: Vec<vk::PipelineShaderStageCreateInfo>,
// }

// impl Shaders {
//     pub fn new(device: Arc<LogicalDevice>, ci: Vec<ShaderCreateInfo>) -> Result<Arc<Self>, VulkanErrors> {
//         let mut stages = Vec::with_capacity(ci.len());
//         for value in ci {
//             let create_info = vk::ShaderModuleCreateInfo {
//                 code_size: value.data.len(),
//                 p_code: value.data.as_ptr() as *const u32,
//                 ..Default::default()
//             };
//             let module = unsafe { device.device.create_shader_module(&create_info, None).map_err(|err|{VulkanErrors::from(err)})? };
//             stages.push(vk::PipelineShaderStageCreateInfo {
//                 stage: value.stage,
//                 p_name: value.entry.as_bytes().as_ptr() as *const i8,
//                 module,
//                 ..Default::default()
//             });
//         }
//         Ok(Arc::new(Self { device, stages }))
//     }
// }
// impl HasShaderStages for Shaders {
//     fn stages(&self) -> impl ExactSizeIterator<Item = &vk::PipelineShaderStageCreateInfo> {
//         VulkanShadersIter::new(&self.stages)
//     }
// }
// impl Drop for Shaders {
//     fn drop(&mut self) {
//         for shader in self.stages.iter() {
//             unsafe { self.device.device.destroy_shader_module(shader.module, None) };
//         }
//     }
// }