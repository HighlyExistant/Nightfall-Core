use std::sync::Arc;

use ash::{vk, vk_bitflags_wrapped};
use smallvec::SmallVec;

use crate::{device::LogicalDevice, image::Sampler, pipeline::shader::ShaderStageFlags};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html>"]
pub struct DescriptorBindingFlags(pub(crate) u32);
vk_bitflags_wrapped!(DescriptorBindingFlags, u32);
impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND: Self = Self(0b1);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(0b10);
    pub const PARTIALLY_BOUND: Self = Self(0b100);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(0b1000);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html>"]
pub struct DescriptorType(pub(crate) i32);
impl DescriptorType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html>"]
pub struct DescriptorSetLayoutCreateFlags(pub(crate) u32);
vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, u32);

impl DescriptorSetLayoutCreateFlags {
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(0b1_0000);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self = Self(0b10_0000);
    pub const HOST_ONLY_POOL_EXT: Self = Self(0b100);
    pub const PUSH_DESCRIPTOR_KHR: Self = Self(0b1);
    pub const RESERVED_3_AMD: Self = Self(0b1000);
    pub const RESERVED_6_EXT: Self = Self(0b100_0000);
    pub const UPDATE_AFTER_BIND_POOL: Self = Self(0b10);
}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBinding.html>"]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub immutable_samplers: Option<Arc<Sampler>>,
}
impl From<&DescriptorSetLayoutBinding> for vk::DescriptorSetLayoutBinding {
    fn from(value: &DescriptorSetLayoutBinding) -> Self {
        Self { 
            binding: value.binding, 
            descriptor_type: vk::DescriptorType::from_raw(value.descriptor_type.0), 
            descriptor_count: value.descriptor_count, 
            stage_flags: vk::ShaderStageFlags::from_raw(value.stage_flags.0), 
            p_immutable_samplers: value.immutable_samplers.as_ref().map(|val|{&val.sampler as *const _}).unwrap_or(std::ptr::null())
        }
    }
}
impl DescriptorType {
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
}
pub struct DescriptorLayout {
    device: Arc<LogicalDevice>,
    bindings: SmallVec<[DescriptorSetLayoutBinding; 4]>,
    layout: vk::DescriptorSetLayout,
}
impl DescriptorLayout {
    #[inline]
    pub fn layout(&self) -> vk::DescriptorSetLayout { self.layout }
    pub fn bindings(&self) -> impl ExactSizeIterator<Item = &DescriptorSetLayoutBinding> {
        self.bindings.iter()
    }
    pub fn builder() -> DescriptorLayoutBuilder {
        DescriptorLayoutBuilder::new()
    }
}
pub struct DescriptorLayoutBuilder {
    flags: vk::DescriptorSetLayoutCreateFlags,
    bindings: SmallVec<[DescriptorSetLayoutBinding; 4]>,
    bindflags: SmallVec<[vk::DescriptorBindingFlags; 4]>,
    uses_bindflags: bool,
}
impl DescriptorLayoutBuilder {
    pub fn new() -> Self {
        Self { flags: vk::DescriptorSetLayoutCreateFlags::empty(), bindings: SmallVec::new(), bindflags: SmallVec::new(), uses_bindflags: true }
    }
    pub fn add_binding(mut self, binding: u32, ty: DescriptorType, descriptor_count: u32, flags: ShaderStageFlags) -> Self {
        let binding = DescriptorSetLayoutBinding {
            binding: binding,
            descriptor_count: descriptor_count,
            descriptor_type: ty,
            stage_flags: flags,
            immutable_samplers: None
        };
        self.bindflags.push(vk::DescriptorBindingFlags::empty());
        self.bindings.push(binding);
        self
    }
    pub fn set_flag(mut self, flags: DescriptorSetLayoutCreateFlags) -> Self {
        self.flags = vk::DescriptorSetLayoutCreateFlags::from_raw(flags.0);
        self
    }
    pub fn uses_binding_flags(mut self) -> Self {
        self.uses_bindflags = true;
        self
    }
    pub fn set_binding_flag(mut self, flags: DescriptorBindingFlags) -> Self {
        self.bindflags[self.bindings.len()-1] = vk::DescriptorBindingFlags::from_raw(flags.0);        
        self
    }
    pub fn fill(device: Arc<LogicalDevice>, bindings: &[DescriptorSetLayoutBinding], bindflags: &[DescriptorBindingFlags], flags: DescriptorSetLayoutCreateFlags) -> Arc<DescriptorLayout> {
        let uses_bindflags = !flags.is_empty();
        let bindings_vec = bindings.iter().map(|val|{val.clone()}).collect::<SmallVec<[DescriptorSetLayoutBinding; 4]>>();
        let bindflags_vec = if uses_bindflags {
            bindflags.iter().map(|val|{
                vk::DescriptorBindingFlags::from_raw(val.0)
            }).collect::<SmallVec<[vk::DescriptorBindingFlags; 4]>>()
        } else {
            SmallVec::<[vk::DescriptorBindingFlags; 4]>::new()
        };
        let this = Self { uses_bindflags, bindflags: bindflags_vec, bindings: bindings_vec, flags: vk::DescriptorSetLayoutCreateFlags::from_raw(flags.0) };
        this.build(device.clone())
    }
    pub fn build(self, device: Arc<LogicalDevice>) -> Arc<DescriptorLayout> {
        let mut bindflags = vk::DescriptorSetLayoutBindingFlagsCreateInfo::default();
        let extend = if self.uses_bindflags {
            bindflags.binding_count = self.bindflags.len() as u32;
            bindflags.p_binding_flags = self.bindflags.as_ptr();
            &bindflags as *const _ as _ 
        } else { 
            std::ptr::null() 
        };
        let bindings = self.bindings.iter().map(|binds|{ vk::DescriptorSetLayoutBinding::from(binds) }).collect::<SmallVec<[vk::DescriptorSetLayoutBinding; 4]>>();
        let create_info = vk::DescriptorSetLayoutCreateInfo {
            p_bindings: bindings.as_ptr(),
            binding_count: self.bindings.len() as u32,
            flags: self.flags,
            p_next: extend,
            ..Default::default()
        };
        
        let layout = unsafe { device.device.create_descriptor_set_layout(&create_info, None).unwrap() };
        Arc::new(DescriptorLayout { layout, device: device.clone(), bindings: self.bindings })
    }
}
impl Drop for DescriptorLayout {
    fn drop(&mut self) {
        unsafe {
            if cfg!(debug_assertions) {
                println!("Descriptor Layout Dropped");
            }
            self.device.device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}