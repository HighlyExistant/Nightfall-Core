
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct InstanceExtensions {
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_android_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_android_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group_creation.html)\n- Promoted to Vulkan 1.1"]
    pub khr_device_group_creation: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_display: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_capabilities.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_external_fence_capabilities: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_capabilities.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_external_memory_capabilities: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_capabilities.html)\n- Promoted to Vulkan 1.1\n- Requires all of:\n  - Vulkan API version 1.1 or instance extension [`khr_get_physical_device_properties2`](crate::instance::InstanceExtensions::khr_get_physical_device_properties2)"]
    pub khr_external_semaphore_capabilities: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_display_properties2.html)\n- Requires all of:\n  - instance extension [`khr_display`](crate::instance::InstanceExtensions::khr_display)"]
    pub khr_get_display_properties2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_physical_device_properties2.html)\n- Promoted to Vulkan 1.1"]
    pub khr_get_physical_device_properties2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_surface_capabilities2.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_get_surface_capabilities2: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_portability_enumeration.html)"]
    pub khr_portability_enumeration: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html)"]
    pub khr_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface_protected_capabilities.html)\n- Requires all of:\n  - Vulkan API version 1.1\n  - instance extension [`khr_get_surface_capabilities2`](crate::instance::InstanceExtensions::khr_get_surface_capabilities2)"]
    pub khr_surface_protected_capabilities: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_wayland_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_wayland_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_win32_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_xcb_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_xcb_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_xlib_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub khr_xlib_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_drm_display.html)\n- Requires all of:\n  - instance extension [`ext_direct_mode_display`](crate::instance::InstanceExtensions::ext_direct_mode_display)"]
    pub ext_acquire_drm_display: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_xlib_display.html)\n- Requires all of:\n  - instance extension [`ext_direct_mode_display`](crate::instance::InstanceExtensions::ext_direct_mode_display)"]
    pub ext_acquire_xlib_display: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_report.html)\n- Deprecated by [`ext_debug_utils`](crate::instance::InstanceExtensions::ext_debug_utils)"]
    pub ext_debug_report: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
    pub ext_debug_utils: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_direct_mode_display.html)\n- Requires all of:\n  - instance extension [`khr_display`](crate::instance::InstanceExtensions::khr_display)"]
    pub ext_direct_mode_display: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_directfb_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub ext_directfb_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_display_surface_counter.html)\n- Requires all of:\n  - instance extension [`khr_display`](crate::instance::InstanceExtensions::khr_display)"]
    pub ext_display_surface_counter: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_headless_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub ext_headless_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub ext_metal_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_surface_maintenance1.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)\n  - instance extension [`khr_get_surface_capabilities2`](crate::instance::InstanceExtensions::khr_get_surface_capabilities2)"]
    pub ext_surface_maintenance1: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_swapchain_colorspace.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub ext_swapchain_colorspace: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_features.html)"]
    pub ext_validation_features: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_flags.html)\n- Deprecated by [`ext_validation_features`](crate::instance::InstanceExtensions::ext_validation_features)"]
    pub ext_validation_flags: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_imagepipe_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub fuchsia_imagepipe_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GGP_stream_descriptor_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub ggp_stream_descriptor_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_surfaceless_query.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub google_surfaceless_query: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_LUNARG_direct_driver_loading.html)"]
    pub lunarg_direct_driver_loading: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_MVK_ios_surface.html)\n- Deprecated by [`ext_metal_surface`](crate::instance::InstanceExtensions::ext_metal_surface)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub mvk_ios_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_MVK_macos_surface.html)\n- Deprecated by [`ext_metal_surface`](crate::instance::InstanceExtensions::ext_metal_surface)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub mvk_macos_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NN_vi_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub nn_vi_surface: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_capabilities.html)\n- Deprecated by [`khr_external_memory_capabilities`](crate::instance::InstanceExtensions::khr_external_memory_capabilities)"]
    pub nv_external_memory_capabilities: bool,
    #[doc = "- [Vulkan documentation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_QNX_screen_surface.html)\n- Requires all of:\n  - instance extension [`khr_surface`](crate::instance::InstanceExtensions::khr_surface)"]
    pub qnx_screen_surface: bool,
}


impl<'a> FromIterator<&'a str> for InstanceExtensions {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut extensions = Self::default();
        for name in iter {
            match name {
                "VK_KHR_android_surface" => {
                    extensions.khr_android_surface = true;
                }
                "VK_KHR_device_group_creation" => {
                    extensions.khr_device_group_creation = true;
                }
                "VK_KHR_display" => {
                    extensions.khr_display = true;
                }
                "VK_KHR_external_fence_capabilities" => {
                    extensions.khr_external_fence_capabilities = true;
                }
                "VK_KHR_external_memory_capabilities" => {
                    extensions.khr_external_memory_capabilities = true;
                }
                "VK_KHR_external_semaphore_capabilities" => {
                    extensions.khr_external_semaphore_capabilities = true;
                }
                "VK_KHR_get_display_properties2" => {
                    extensions.khr_get_display_properties2 = true;
                }
                "VK_KHR_get_physical_device_properties2" => {
                    extensions.khr_get_physical_device_properties2 = true;
                }
                "VK_KHR_get_surface_capabilities2" => {
                    extensions.khr_get_surface_capabilities2 = true;
                }
                "VK_KHR_portability_enumeration" => {
                    extensions.khr_portability_enumeration = true;
                }
                "VK_KHR_surface" => {
                    extensions.khr_surface = true;
                }
                "VK_KHR_surface_protected_capabilities" => {
                    extensions.khr_surface_protected_capabilities = true;
                }
                "VK_KHR_wayland_surface" => {
                    extensions.khr_wayland_surface = true;
                }
                "VK_KHR_win32_surface" => {
                    extensions.khr_win32_surface = true;
                }
                "VK_KHR_xcb_surface" => {
                    extensions.khr_xcb_surface = true;
                }
                "VK_KHR_xlib_surface" => {
                    extensions.khr_xlib_surface = true;
                }
                "VK_EXT_acquire_drm_display" => {
                    extensions.ext_acquire_drm_display = true;
                }
                "VK_EXT_acquire_xlib_display" => {
                    extensions.ext_acquire_xlib_display = true;
                }
                "VK_EXT_debug_report" => {
                    extensions.ext_debug_report = true;
                }
                "VK_EXT_debug_utils" => {
                    extensions.ext_debug_utils = true;
                }
                "VK_EXT_direct_mode_display" => {
                    extensions.ext_direct_mode_display = true;
                }
                "VK_EXT_directfb_surface" => {
                    extensions.ext_directfb_surface = true;
                }
                "VK_EXT_display_surface_counter" => {
                    extensions.ext_display_surface_counter = true;
                }
                "VK_EXT_headless_surface" => {
                    extensions.ext_headless_surface = true;
                }
                "VK_EXT_metal_surface" => {
                    extensions.ext_metal_surface = true;
                }
                "VK_EXT_surface_maintenance1" => {
                    extensions.ext_surface_maintenance1 = true;
                }
                "VK_EXT_swapchain_colorspace" => {
                    extensions.ext_swapchain_colorspace = true;
                }
                "VK_EXT_validation_features" => {
                    extensions.ext_validation_features = true;
                }
                "VK_EXT_validation_flags" => {
                    extensions.ext_validation_flags = true;
                }
                "VK_FUCHSIA_imagepipe_surface" => {
                    extensions.fuchsia_imagepipe_surface = true;
                }
                "VK_GGP_stream_descriptor_surface" => {
                    extensions.ggp_stream_descriptor_surface = true;
                }
                "VK_GOOGLE_surfaceless_query" => {
                    extensions.google_surfaceless_query = true;
                }
                "VK_LUNARG_direct_driver_loading" => {
                    extensions.lunarg_direct_driver_loading = true;
                }
                "VK_MVK_ios_surface" => {
                    extensions.mvk_ios_surface = true;
                }
                "VK_MVK_macos_surface" => {
                    extensions.mvk_macos_surface = true;
                }
                "VK_NN_vi_surface" => {
                    extensions.nn_vi_surface = true;
                }
                "VK_NV_external_memory_capabilities" => {
                    extensions.nv_external_memory_capabilities = true;
                }
                "VK_QNX_screen_surface" => {
                    extensions.qnx_screen_surface = true;
                }
                _ => (),
            }
        }
        extensions
    }
}

impl<'a> From<&'a InstanceExtensions> for Vec<std::ffi::CString> {
    fn from(x: &'a InstanceExtensions) -> Self {
        let mut data = Self::new();
        if x.khr_android_surface {
            data.push(std::ffi::CString::new("VK_KHR_android_surface").unwrap());
        }
        if x.khr_device_group_creation {
            data.push(std::ffi::CString::new("VK_KHR_device_group_creation").unwrap());
        }
        if x.khr_display {
            data.push(std::ffi::CString::new("VK_KHR_display").unwrap());
        }
        if x.khr_external_fence_capabilities {
            data.push(std::ffi::CString::new("VK_KHR_external_fence_capabilities").unwrap());
        }
        if x.khr_external_memory_capabilities {
            data.push(std::ffi::CString::new("VK_KHR_external_memory_capabilities").unwrap());
        }
        if x.khr_external_semaphore_capabilities {
            data.push(std::ffi::CString::new("VK_KHR_external_semaphore_capabilities").unwrap());
        }
        if x.khr_get_display_properties2 {
            data.push(std::ffi::CString::new("VK_KHR_get_display_properties2").unwrap());
        }
        if x.khr_get_physical_device_properties2 {
            data.push(std::ffi::CString::new("VK_KHR_get_physical_device_properties2").unwrap());
        }
        if x.khr_get_surface_capabilities2 {
            data.push(std::ffi::CString::new("VK_KHR_get_surface_capabilities2").unwrap());
        }
        if x.khr_portability_enumeration {
            data.push(std::ffi::CString::new("VK_KHR_portability_enumeration").unwrap());
        }
        if x.khr_surface {
            data.push(std::ffi::CString::new("VK_KHR_surface").unwrap());
        }
        if x.khr_surface_protected_capabilities {
            data.push(std::ffi::CString::new("VK_KHR_surface_protected_capabilities").unwrap());
        }
        if x.khr_wayland_surface {
            data.push(std::ffi::CString::new("VK_KHR_wayland_surface").unwrap());
        }
        if x.khr_win32_surface {
            data.push(std::ffi::CString::new("VK_KHR_win32_surface").unwrap());
        }
        if x.khr_xcb_surface {
            data.push(std::ffi::CString::new("VK_KHR_xcb_surface").unwrap());
        }
        if x.khr_xlib_surface {
            data.push(std::ffi::CString::new("VK_KHR_xlib_surface").unwrap());
        }
        if x.ext_acquire_drm_display {
            data.push(std::ffi::CString::new("VK_EXT_acquire_drm_display").unwrap());
        }
        if x.ext_acquire_xlib_display {
            data.push(std::ffi::CString::new("VK_EXT_acquire_xlib_display").unwrap());
        }
        if x.ext_debug_report {
            data.push(std::ffi::CString::new("VK_EXT_debug_report").unwrap());
        }
        if x.ext_debug_utils {
            data.push(std::ffi::CString::new("VK_EXT_debug_utils").unwrap());
        }
        if x.ext_direct_mode_display {
            data.push(std::ffi::CString::new("VK_EXT_direct_mode_display").unwrap());
        }
        if x.ext_directfb_surface {
            data.push(std::ffi::CString::new("VK_EXT_directfb_surface").unwrap());
        }
        if x.ext_display_surface_counter {
            data.push(std::ffi::CString::new("VK_EXT_display_surface_counter").unwrap());
        }
        if x.ext_headless_surface {
            data.push(std::ffi::CString::new("VK_EXT_headless_surface").unwrap());
        }
        if x.ext_metal_surface {
            data.push(std::ffi::CString::new("VK_EXT_metal_surface").unwrap());
        }
        if x.ext_surface_maintenance1 {
            data.push(std::ffi::CString::new("VK_EXT_surface_maintenance1").unwrap());
        }
        if x.ext_swapchain_colorspace {
            data.push(std::ffi::CString::new("VK_EXT_swapchain_colorspace").unwrap());
        }
        if x.ext_validation_features {
            data.push(std::ffi::CString::new("VK_EXT_validation_features").unwrap());
        }
        if x.ext_validation_flags {
            data.push(std::ffi::CString::new("VK_EXT_validation_flags").unwrap());
        }
        if x.fuchsia_imagepipe_surface {
            data.push(std::ffi::CString::new("VK_FUCHSIA_imagepipe_surface").unwrap());
        }
        if x.ggp_stream_descriptor_surface {
            data.push(std::ffi::CString::new("VK_GGP_stream_descriptor_surface").unwrap());
        }
        if x.google_surfaceless_query {
            data.push(std::ffi::CString::new("VK_GOOGLE_surfaceless_query").unwrap());
        }
        if x.lunarg_direct_driver_loading {
            data.push(std::ffi::CString::new("VK_LUNARG_direct_driver_loading").unwrap());
        }
        if x.mvk_ios_surface {
            data.push(std::ffi::CString::new("VK_MVK_ios_surface").unwrap());
        }
        if x.mvk_macos_surface {
            data.push(std::ffi::CString::new("VK_MVK_macos_surface").unwrap());
        }
        if x.nn_vi_surface {
            data.push(std::ffi::CString::new("VK_NN_vi_surface").unwrap());
        }
        if x.nv_external_memory_capabilities {
            data.push(std::ffi::CString::new("VK_NV_external_memory_capabilities").unwrap());
        }
        if x.qnx_screen_surface {
            data.push(std::ffi::CString::new("VK_QNX_screen_surface").unwrap());
        }
        data
    }
}

macro_rules! for_each_instance_extension {
    ($op:ident, $this:ident, $val:ident) => {
    $this.khr_android_surface=$this.khr_android_surface.$op($val.khr_android_surface);
    $this.khr_device_group_creation=$this.khr_device_group_creation.$op($val.khr_device_group_creation);
    $this.khr_display=$this.khr_display.$op($val.khr_display);
    $this.khr_external_fence_capabilities=$this.khr_external_fence_capabilities.$op($val.khr_external_fence_capabilities);
    $this.khr_external_memory_capabilities=$this.khr_external_memory_capabilities.$op($val.khr_external_memory_capabilities);
    $this.khr_external_semaphore_capabilities=$this.khr_external_semaphore_capabilities.$op($val.khr_external_semaphore_capabilities);
    $this.khr_get_display_properties2=$this.khr_get_display_properties2.$op($val.khr_get_display_properties2);
    $this.khr_get_physical_device_properties2=$this.khr_get_physical_device_properties2.$op($val.khr_get_physical_device_properties2);
    $this.khr_get_surface_capabilities2=$this.khr_get_surface_capabilities2.$op($val.khr_get_surface_capabilities2);
    $this.khr_portability_enumeration=$this.khr_portability_enumeration.$op($val.khr_portability_enumeration);
    $this.khr_surface=$this.khr_surface.$op($val.khr_surface);
    $this.khr_surface_protected_capabilities=$this.khr_surface_protected_capabilities.$op($val.khr_surface_protected_capabilities);
    $this.khr_wayland_surface=$this.khr_wayland_surface.$op($val.khr_wayland_surface);
    $this.khr_win32_surface=$this.khr_win32_surface.$op($val.khr_win32_surface);
    $this.khr_xcb_surface=$this.khr_xcb_surface.$op($val.khr_xcb_surface);
    $this.khr_xlib_surface=$this.khr_xlib_surface.$op($val.khr_xlib_surface);
    $this.ext_acquire_drm_display=$this.ext_acquire_drm_display.$op($val.ext_acquire_drm_display);
    $this.ext_acquire_xlib_display=$this.ext_acquire_xlib_display.$op($val.ext_acquire_xlib_display);
    $this.ext_debug_report=$this.ext_debug_report.$op($val.ext_debug_report);
    $this.ext_debug_utils=$this.ext_debug_utils.$op($val.ext_debug_utils);
    $this.ext_direct_mode_display=$this.ext_direct_mode_display.$op($val.ext_direct_mode_display);
    $this.ext_directfb_surface=$this.ext_directfb_surface.$op($val.ext_directfb_surface);
    $this.ext_display_surface_counter=$this.ext_display_surface_counter.$op($val.ext_display_surface_counter);
    $this.ext_headless_surface=$this.ext_headless_surface.$op($val.ext_headless_surface);
    $this.ext_metal_surface=$this.ext_metal_surface.$op($val.ext_metal_surface);
    $this.ext_surface_maintenance1=$this.ext_surface_maintenance1.$op($val.ext_surface_maintenance1);
    $this.ext_swapchain_colorspace=$this.ext_swapchain_colorspace.$op($val.ext_swapchain_colorspace);
    $this.ext_validation_features=$this.ext_validation_features.$op($val.ext_validation_features);
    $this.ext_validation_flags=$this.ext_validation_flags.$op($val.ext_validation_flags);
    $this.fuchsia_imagepipe_surface=$this.fuchsia_imagepipe_surface.$op($val.fuchsia_imagepipe_surface);
    $this.ggp_stream_descriptor_surface=$this.ggp_stream_descriptor_surface.$op($val.ggp_stream_descriptor_surface);
    $this.google_surfaceless_query=$this.google_surfaceless_query.$op($val.google_surfaceless_query);
    $this.lunarg_direct_driver_loading=$this.lunarg_direct_driver_loading.$op($val.lunarg_direct_driver_loading);
    $this.mvk_ios_surface=$this.mvk_ios_surface.$op($val.mvk_ios_surface);
    $this.mvk_macos_surface=$this.mvk_macos_surface.$op($val.mvk_macos_surface);
    $this.nn_vi_surface=$this.nn_vi_surface.$op($val.nn_vi_surface);
    $this.nv_external_memory_capabilities=$this.nv_external_memory_capabilities.$op($val.nv_external_memory_capabilities);
    $this.qnx_screen_surface=$this.qnx_screen_surface.$op($val.qnx_screen_surface);
    };
}
impl std::ops::BitOr for InstanceExtensions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut this = self;
        for_each_instance_extension!(bitor, this, rhs);
        this
    }
}
impl std::ops::BitAnd for InstanceExtensions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut this = self;
        for_each_instance_extension!(bitand, this, rhs);
        this
    }
}
impl std::ops::BitXor for InstanceExtensions {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut this = self;
        for_each_instance_extension!(bitxor, this, rhs);
        this
    }
}