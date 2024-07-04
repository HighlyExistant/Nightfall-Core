use std::{ops::Range, os::raw::c_void, sync::Arc};

use ash::{vk::{self, Handle}, vk_bitflags_wrapped};
mod map;
pub use map::*;

use crate::{device::LogicalDevice, error::VulkanError, memory::{DeviceMemory, DevicePointer}, AsNfptr, NfPtr};
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html>"]
pub struct BufferUsageFlags(pub(crate) u32);
vk_bitflags_wrapped!(BufferUsageFlags, u32);
impl BufferUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0b10);
    #[doc = "Can be used as TBO"]
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b100);
    #[doc = "Can be used as IBO"]
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1000);
    #[doc = "Can be used as UBO"]
    pub const UNIFORM_BUFFER: Self = Self(0b1_0000);
    #[doc = "Can be used as SSBO"]
    pub const STORAGE_BUFFER: Self = Self(0b10_0000);
    #[doc = "Can be used as source of fixed-function index fetch (index buffer)"]
    pub const INDEX_BUFFER: Self = Self(0b100_0000);
    #[doc = "Can be used as source of fixed-function vertex fetch (VBO)"]
    pub const VERTEX_BUFFER: Self = Self(0b1000_0000);
    #[doc = "Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)"]
    pub const INDIRECT_BUFFER: Self = Self(0b1_0000_0000);
    pub const SHADER_DEVICE_ADDRESS: Self = Self(0b10_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html>"]
pub struct MemoryPropertyFlags(pub(crate) u32);
vk_bitflags_wrapped!(MemoryPropertyFlags, u32);
impl MemoryPropertyFlags {
    #[doc = "If otherwise stated, then allocate memory on device"]
    pub const DEVICE_LOCAL: Self = Self(0b1);
    #[doc = "Memory is mappable by host"]
    pub const HOST_VISIBLE: Self = Self(0b10);
    #[doc = "Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache"]
    pub const HOST_COHERENT: Self = Self(0b100);
    #[doc = "Memory is mappable by host & Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache"]
    pub const HOST_VISIBLE_COHERENT: Self = Self(0b110);
    #[doc = "Memory will be cached by the host"]
    pub const HOST_CACHED: Self = Self(0b1000);
    #[doc = "Memory may be allocated by the driver when it is required"]
    pub const LAZILY_ALLOCATED: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html>"]
pub struct BufferCreateFlags(pub(crate) u32);
vk_bitflags_wrapped!(BufferCreateFlags, u32);
impl BufferCreateFlags {
    #[doc = "Buffer should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0b1);
    #[doc = "Buffer should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0b10);
    #[doc = "Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers"]
    pub const SPARSE_ALIASED: Self = Self(0b100);
}
pub struct Buffer {
    pub(crate) handle: vk::Buffer,
    pub(crate) device: Arc<LogicalDevice>,
    pub(crate) usage: BufferUsageFlags,
    pub(crate) properties: MemoryPropertyFlags,
    pub(crate) size: usize,
    pub(crate) alignment: usize,
    pub(crate) memory: Arc<DeviceMemory>,
    pub(crate) device_ptr: Option<DevicePointer>,
}
#[derive(Default)]
pub struct BufferCreateInfo<'a> {
    pub size: usize, 
    pub usage: BufferUsageFlags, 
    pub properties: MemoryPropertyFlags, 
    pub flags: BufferCreateFlags, 
    pub share: Option<&'a[u32]>,
    pub buffer_addressing: bool,
}
impl Buffer {
    pub fn new(device: Arc<LogicalDevice>, info: BufferCreateInfo) -> Result<Self, VulkanError> {
        let (mode, count, indices) = if let Some(share) = info.share {
            (vk::SharingMode::CONCURRENT, share.len() as u32, share.as_ptr())
        } else {
            (vk::SharingMode::EXCLUSIVE, 0, std::ptr::null())
        };
        let mut usage = vk::BufferUsageFlags::from_raw(info.usage.0);
        let device_addressing = if info.buffer_addressing {
            usage |= vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;
            Some(vk::MemoryAllocateFlagsInfo { flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS, ..Default::default() })
        } else {
            None
        };
        let extension = if let Some(addr) = &device_addressing {
            addr as *const _
        } else {
            std::ptr::null()
        };
        let create_info = vk::BufferCreateInfo {
            flags: vk::BufferCreateFlags::from_raw(info.flags.0),
            sharing_mode: mode,
            queue_family_index_count: count,
            p_queue_family_indices: indices,
            size: info.size as u64,
            usage,
            ..Default::default()
        };
        let handle = unsafe { device.device.create_buffer(&create_info, None).map_err(VulkanError::from) }?;
        let requirements = device.get_buffer_memory_requirements(handle);
        let memory_index = DeviceMemory::get_memory_type_index(device.clone(), info.properties, requirements);
        let memory = Arc::new(DeviceMemory::allocate(device.clone(), info.size, memory_index, extension)?);
        memory.bind_memory(handle, 0)?;
        let device_ptr = if info.buffer_addressing {
            device.buffer_device_address(handle).ok()
        } else {
            None
        };
        Ok(Self {
            handle,
            device,
            usage: info.usage,
            properties: info.properties,
            size: info.size,
            alignment: requirements.alignment as usize,
            memory,
            device_ptr: device_ptr
        })
    }
    pub fn memory_requirements(&self) -> vk::MemoryRequirements {
        self.device.get_buffer_memory_requirements(self.handle)
    }
    #[inline(always)]
    pub fn usage(&self) -> BufferUsageFlags {
        self.usage
    }
    #[inline(always)]
    pub fn get_address(&self) -> Option<DevicePointer> {
        self.device_ptr
    }
    pub fn memory(&self) -> Arc<DeviceMemory> {
        self.memory.clone()
    }
    pub fn buffer(&self) -> vk::Buffer {
        self.handle
    }
    pub fn device(&self) -> Arc<LogicalDevice> {
        self.device.clone()
    }
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }
    #[inline(always)]
    pub fn alignment(&self) -> usize {
        self.alignment
    }
    #[inline(always)]
    pub fn handle(&self) -> vk::Buffer {
        self.handle
    }
    pub fn is_mappable(&self) -> bool {
        self.properties.contains(MemoryPropertyFlags::HOST_VISIBLE)
    }
    #[inline(always)]
    pub fn properties(&self) -> MemoryPropertyFlags {
        self.properties
    }
    pub unsafe fn raw_map<T>(&self, size: usize, offset: usize) -> Result<*mut T, VulkanError>  {
         self.device.device.map_memory(self.memory.memory(), offset as u64, size as u64, vk::MemoryMapFlags::empty()).map_err(VulkanError::from).map(|ok|{ ok as *mut T })
    }
    pub unsafe fn raw_unmap(&self) {
        self.device.device.unmap_memory(self.memory.memory());
    }
    pub fn buffer_addressing_enabled(&self) -> bool {
        self.device_ptr.is_some()
    }
    pub fn map_memory<T>(self: Arc<Self>, range: Range<usize>) -> Result<MappedMemory<T>, VulkanError> {
        MappedMemory::new(self.clone(), range)
    }
    pub fn offset(&self, offset: usize) -> BufferOffset {
        unsafe { BufferOffset::from_raw(self.handle, offset as u64) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_buffer(self.handle, None) };
    }
}

pub struct BufferOffset {
    pub(crate) handle: vk::Buffer,
    pub(crate) offset: u64,
}

impl BufferOffset {
    pub unsafe fn from_raw(handle: vk::Buffer, offset: u64) -> Self {
        Self { handle, offset }
    }
}

unsafe impl AsNfptr for Buffer {
    unsafe fn as_nfptr(&self) -> NfPtr {
        NfPtr::new(self.buffer().as_raw(), 0, self.device_ptr, self.size)
    }
}