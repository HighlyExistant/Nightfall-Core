use std::{ops::{Add, Index, IndexMut, Range}, os::raw::c_void, ptr::NonNull, sync::Arc};

use ash::vk;

use crate::{device::LogicalDevice, error::{NightfallError, VulkanError}, memory::{DevicePointer, MappingState}};

use super::Buffer;

pub struct MappedMemory<T> {
    ptr: NonNull<T>,
    range: Range<usize>,
    buffer: Arc<Buffer>,
}

impl<T> MappedMemory<T> {
    /// map memory with a range. The [`range`] is in units of T and offseted to where the buffer is allocated; e.g., a range of 0..3 
    /// represents a pointer offset of 0..(3 * size_of::<T>() bytes).
    ///
    /// [`MappedMemory`] is made to be used with buffers, if you want to work with [`DeviceMemory`] look into [`MappingState`].
    pub fn new(buffer: Arc<Buffer>, range: Range<usize>) -> Result<Self, VulkanError> {
        // check if memory has already been mapped on this buffers memory
        if let Some(mapped) = unsafe { buffer.memory.mapping_state.as_ptr().as_mut().unwrap() } {
            // If the memory has already been mapped then try to see if its within range
            if mapped.end() <= mapped.offset() + range.end as u64 {
                unsafe {
                    return Ok(
                        MappedMemory {
                            ptr: NonNull::new_unchecked(mapped.ptr().as_ptr().add(range.start) as *mut _ as _), 
                            range,
                            buffer,
                    })
                }
            } else { // If it's not within range return error
                return Err(VulkanError::MemoryMapFailed);
            }
        } else {
            // If it's not mapped then proceed like normal
            unsafe {
                let ptr = buffer.device.device.map_memory(buffer.memory.memory(), 
                range.start as u64, 
                range.end as u64, 
                vk::MemoryMapFlags::empty())
                .map_err(VulkanError::from)?;
                let mapped = NonNull::new_unchecked(ptr).cast::<T>();
                buffer.memory.mapping_state.set(Some(MappingState::new(mapped.cast::<_>(), range.start as u64..range.end as u64)));
                Ok(MappedMemory {
                    ptr: mapped, 
                    range,
                    buffer,
                })
            }
        }
    }
    pub unsafe fn offset_from_start(&self, ptr: usize) -> usize {
        let selfptr = self.ptr.as_ptr() as *mut _ as *mut T as usize;
        // The pointer must be larger than the start as we are assuming the data is in ascending order.
        assert!(ptr > selfptr);
        ptr - selfptr
    }
    pub fn to_device_ptr(&self, ptr: *const c_void) -> Result<DevicePointer, VulkanError> {
        let start = self.ptr.as_ptr() as *const usize as usize;
        let end = start+(self.range.end*std::mem::size_of::<T>());

        assert!((ptr as usize) >= start && (ptr as usize) <= end, "the host pointer: {ptr:?}, must lie between the mapped range {start} - {end}. ");

        let offset = unsafe { self.offset_from_start(ptr as usize) };
        Ok(self.buffer.get_address().ok_or(VulkanError::BufferDeviceAddressingDisabled)?.add(offset as usize))
    }
    pub fn ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }
    pub fn mut_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T> Index<usize> for MappedMemory<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.ptr.as_ptr().add(index).as_ref().unwrap() }
    }
}
impl<T> IndexMut<usize> for MappedMemory<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.ptr.as_ptr().add(index).as_mut().unwrap() }
    }
}