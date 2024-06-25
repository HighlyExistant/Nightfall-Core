use std::ops::Add;

use ash::vk::{self, Handle};

use crate::{barriers::BufferMemoryBarrier, buffers::BufferOffset, descriptors::DescriptorBufferInfo, memory::{AccessFlags, DevicePointer}};

#[derive(Clone, Copy, Debug)]
pub struct NfPtr {
    id: u64,
    offset: usize,
    size: usize,
    address: Option<DevicePointer>,
}
impl NfPtr {
    pub const fn new(alloc_id: u64, offset: usize, ptr: Option<DevicePointer>, size: usize) -> Self { Self { id: alloc_id, offset, size, address: ptr } }
    #[inline]
    pub const unsafe fn dangling() -> Self { Self { id: 0, offset: 0, size: 0, address: None } }
    #[inline]
    pub const fn id(&self) -> u64 { self.id }
    #[inline]
    pub const fn offset(&self) -> usize { self.offset }
    #[inline]
    pub const fn size(&self) -> usize { self.size }
    #[inline]
    pub const fn device_address(&self) -> Option<DevicePointer> { self.address }
    #[inline]
    pub fn is_same_buffer(&self, other: &Self) -> bool {
        self.id == other.id
    }
    pub fn buffer(&self) -> ash::vk::Buffer {
        ash::vk::Buffer::from_raw(self.id())
    }
    pub fn as_buffer_offset(&self) -> BufferOffset {
        unsafe { BufferOffset::from_raw(ash::vk::Buffer::from_raw(self.id()), self.offset() as u64) }
    }
    pub fn as_descriptor_buffer_info(&self) -> DescriptorBufferInfo {
        DescriptorBufferInfo {
            buffer: self.buffer(),
            offset: self.offset() as u64,
            range: self.size() as u64,
        }
    }
    pub fn fill_buffer_memory_barrier(&self, barrier: &mut vk::BufferMemoryBarrier) {
        barrier.buffer = self.buffer();
        barrier.size = self.size() as u64;
        barrier.offset = self.offset() as u64;
    }
    pub fn as_buffer_memory_barrier(&self, src_access: AccessFlags, dst_access: AccessFlags) -> BufferMemoryBarrier {
        BufferMemoryBarrier {
            buffer: self.buffer(),
            size: self.size() as u64,
            offset: self.offset() as u64,
            src_access_mask: src_access,
            dst_access_mask: dst_access,
            ..Default::default()
        }
    }
}
impl PartialEq for NfPtr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.offset == other.offset
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id || self.offset != other.offset
    }
}

impl Add<usize> for NfPtr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        if let Some(address) = self.address {
            Self::new(self.id(), self.offset()+rhs, Some(DevicePointer::from_raw(address.addr() as u64+rhs as u64)), self.size-rhs)
        } else {
            Self::new(self.id(), self.offset()+rhs, None, self.size-rhs)
        }
    }
}