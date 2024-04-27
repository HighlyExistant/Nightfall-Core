use std::{ops::Range, os::raw::c_void, ptr::NonNull};

use ash::vk::DeviceSize;

#[derive(Debug)]
pub struct MappingState {
    ptr: NonNull<c_void>,
    range: Range<DeviceSize>,
}

unsafe impl Send for MappingState {}
unsafe impl Sync for MappingState {}


impl MappingState {
    pub(crate) fn new(ptr: NonNull<c_void>, range: Range<DeviceSize>) -> Self {
        Self { ptr, range }
    }
    #[inline]
    pub fn ptr(&self) -> NonNull<c_void> {
        self.ptr
    }
    #[inline]
    pub fn offset(&self) -> DeviceSize {
        self.range.start
    }
    #[inline]
    pub fn end(&self) -> DeviceSize {
        self.range.end
    }
    #[inline]
    pub fn size(&self) -> DeviceSize {
        self.range.end - self.range.start
    }
}