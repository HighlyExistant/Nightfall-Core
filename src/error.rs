use ash::vk;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, PartialOrd)]
pub enum VulkanError {
    #[error("Out of host memory")]
    OutOfHostMemory,
    #[error("Out of device memory")]
    OutOfDeviceMemory,
    #[error("Out of memory")]
    OutOfMemory,
    #[error("failed to initialize")]
    InitializationFailed,
    #[error("Layer not present")]
    LayerNotPresent,
    #[error("Extensions not present")]
    ExtensionNotPresent,
    #[error("Features not present")]
    FeatureNotPresent,
    #[error("Too many objects")]
    TooManyObjects,
    #[error("Lost device")]
    DeviceLost,
    #[error("Selected driver is incompatible")]
    IncompatibleDriver,
    #[error("Invalid shader")]
    InvalidShader,
    #[error("Fragmented pool")]
    FragmentedPool,
    #[error("Out of pool memory")]
    OutOfPoolMemory,
    #[error("Invalid video parameters")]
    InvalidVideoStdParameters,
    #[error("Failed to map memory")]
    MemoryMapFailed,
    #[error("Buffer device addressing is disabled")]
    BufferDeviceAddressingDisabled,
    #[error("Surface lost")]
    SurfaceLost,
    #[error("Full screen exclusive mode lost")]
    FullScreenExclusiveModeLost,
    #[error("Warning: Suboptimal")]
    Suboptimal,
    #[error("Out of date")]
    OutOfDate,
    #[error("Can not be accessed by the host")]
    NotAccessibleInHostMemory,
    #[error("The prefix header of the safe pipeline cache is invalid, build the pipeline without the cache")]
    InvalidPipelineCachePrefixHeader,
}
impl From<ash::vk::Result> for VulkanError {
    fn from(value: vk::Result) -> Self {
        match value {
            vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
                VulkanError::OutOfHostMemory
            }
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                VulkanError::OutOfDeviceMemory
            }
            vk::Result::ERROR_INITIALIZATION_FAILED => {
                VulkanError::InitializationFailed
            }
            vk::Result::ERROR_LAYER_NOT_PRESENT => {
                VulkanError::LayerNotPresent
            }
            vk::Result::ERROR_EXTENSION_NOT_PRESENT => {
                VulkanError::ExtensionNotPresent
            }
            vk::Result::ERROR_INCOMPATIBLE_DRIVER => {
                VulkanError::IncompatibleDriver
            }
            vk::Result::ERROR_FEATURE_NOT_PRESENT => {
                VulkanError::FeatureNotPresent
            }
            vk::Result::ERROR_TOO_MANY_OBJECTS => {
                VulkanError::TooManyObjects
            }
            vk::Result::ERROR_DEVICE_LOST => {
                VulkanError::DeviceLost
            }
            vk::Result::ERROR_INVALID_SHADER_NV => {
                VulkanError::InvalidShader
            }
            vk::Result::ERROR_FRAGMENTED_POOL => {
                VulkanError::FragmentedPool
            }
            vk::Result::ERROR_OUT_OF_POOL_MEMORY => {
                VulkanError::OutOfPoolMemory
            }
            vk::Result::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR => {
                VulkanError::InvalidVideoStdParameters
            }
            vk::Result::ERROR_MEMORY_MAP_FAILED => {
                VulkanError::MemoryMapFailed
            }
            vk::Result::ERROR_SURFACE_LOST_KHR => {
                VulkanError::SurfaceLost
            }
            vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
                VulkanError::FullScreenExclusiveModeLost
            }
            vk::Result::SUBOPTIMAL_KHR => {
                VulkanError::Suboptimal
            }
            vk::Result::ERROR_OUT_OF_DATE_KHR => {
                VulkanError::OutOfDate
            }
            _ => panic!("Error Unknown")
        }
    }
}

/// NightfallError is a more robust error handler for  
/// Nightfall specific operations, not only vulkan
#[derive(Error, Debug, PartialEq, PartialOrd)]
pub enum NightfallError {
    #[error("{0}")]
    VulkanError(VulkanError),
    #[error("{0}")]
    InternalError(String),
    #[error("Failed to allocate memory of size {0}, there is {1} remaining memory")]
    OutOfMemory(usize, usize),
    #[error("Zero Sized Types are Invalid")]
    NoZeroSizedTypes,
    #[error("Failed to deallocate memory which is not marked as in use by the allocator")]
    InvalidFree,
    #[error("The memory you are planning on using can not be addressed by device")]
    NotDeviceAddressable,
    #[error("The memory you are planning on using can not be mapped in host memory.")]
    NotHostMappable,
    #[error("The amount of descriptor sets necessary for this operation is {1} but {0} were provided")]
    NotEnoughDescriptorSets(usize, usize),
    #[error("No Input was provided for this action")]
    NoInputWasGiven,
}

impl From<VulkanError> for NightfallError {
    fn from(value: VulkanError) -> Self {
        NightfallError::VulkanError(value)
    }
}