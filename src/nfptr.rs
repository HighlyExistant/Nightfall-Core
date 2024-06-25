use std::{marker::PhantomData, ops::{Add, Deref, DerefMut}};

use ash::vk::{self, Handle};

use crate::{barriers::BufferMemoryBarrier, buffers::BufferOffset, descriptors::DescriptorBufferInfo, error::{NightfallError, PointerError}, memory::{AccessFlags, DevicePointer}};

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
    pub unsafe fn add(self, rhs: usize) -> Self {
        if let Some(address) = self.address {
            Self::new(self.id(), self.offset()+rhs, Some(DevicePointer::from_raw(address.addr() as u64+rhs as u64)), self.size-rhs)
        } else {
            Self::new(self.id(), self.offset()+rhs, None, self.size-rhs)
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct NfPtrType<T>(NfPtr, PhantomData<T>);

impl<T> NfPtrType<T> {
    #[inline]
    pub const fn id(&self) -> u64 { self.0.id() }
    #[inline]
    pub const fn offset(&self) -> usize { self.0.offset() }
    #[inline]
    pub const fn size(&self) -> usize { self.0.size() }
    #[inline]
    pub const fn device_address(&self) -> Option<DevicePointer> { self.0.device_address() }
    #[inline]
    pub fn is_same_buffer(&self, other: &Self) -> bool {
        self.0.is_same_buffer(&other.0)
    }
    pub fn buffer(&self) -> ash::vk::Buffer {
        self.0.buffer()
    }
    pub fn as_buffer_offset(&self) -> BufferOffset {
        self.0.as_buffer_offset()
    }
    pub fn as_descriptor_buffer_info(&self) -> DescriptorBufferInfo {
        self.0.as_descriptor_buffer_info()
    }
    pub fn fill_buffer_memory_barrier(&self, barrier: &mut vk::BufferMemoryBarrier) {
        self.0.fill_buffer_memory_barrier(barrier)
    }
    pub fn as_buffer_memory_barrier(&self, src_access: AccessFlags, dst_access: AccessFlags) -> BufferMemoryBarrier {
        self.0.as_buffer_memory_barrier(src_access, dst_access)
    }
    pub unsafe fn add(self, rhs: usize) -> Self {
        let rhs = rhs*std::mem::size_of::<T>();
        if let Some(address) = self.device_address() {
            Self(NfPtr::new(self.id(), self.offset()+rhs, Some(DevicePointer::from_raw(address.addr() as u64+rhs as u64)), self.size()-rhs), PhantomData)
        } else {
            Self(NfPtr::new(self.id(), self.offset()+rhs, None, self.size()-rhs), PhantomData)
        }
    }
}

impl<T: Sized> From<NfPtrType<T>> for NfPtr {
    fn from(value: NfPtrType<T>) -> Self {
        value.0
    }
}
impl<'a, T: Sized> From<&'a NfPtrType<T>> for NfPtr {
    fn from(value: &'a NfPtrType<T>) -> Self {
        value.0
    }
}

impl<T: Sized> From<NfPtr> for NfPtrType<T> {
    fn from(value: NfPtr) -> Self {
        assert!(std::mem::align_of::<T>()*(value.size()/std::mem::align_of::<T>()) == value.size(), "Alignment error while casting pointer");
        Self(value, PhantomData)
    }
}
impl<'a, T: Sized> TryFrom<&'a NfPtr> for NfPtrType<T> {
    type Error = NightfallError;
    fn try_from(value: &'a NfPtr) -> Result<Self, Self::Error> {
        // check if division by align has a remainder.
        if (std::mem::align_of::<T>()*(value.size()/std::mem::align_of::<T>())) == value.size() {
            Ok(Self(*value, PhantomData))
        } else { // if it does have a remainder then that is an alignment issue
            Err(NightfallError::PointerError(PointerError::CastAlignmentError))
        }
    }
}