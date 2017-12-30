use ash::vk;
use ash::version::V1_0;
use ash::version::DeviceV1_0;
use ash::extensions as ext;
use winit;

use std::ptr;
use std::u32;
use std::error::Error;

use super::{VK_ENTRY, VK_INSTANCE};

pub struct Surface {
    pub width: u32,
    pub height: u32,
    loader: ext::Surface,
    handle: vk::SurfaceKHR,
    format: vk::SurfaceFormatKHR,
}

impl Surface {
    pub fn new(window: &winit::Window) -> Result<Surface, Box<Error>> {
        let handle = Surface::create_surface_khr(window)?;
        let loader = ext::Surface::new(VK_ENTRY, VK_INSTANCE).map_err(|errors| errors.join("\n"))?;
        let (width, height) = window.get_inner_size().unwrap_or((0, 0));
        let format = vk::SurfaceFormatKHR {
            format: vk::Format::B8g8r8Unorm,
            color_space: vk::ColorSpaceKHR::SrgbNonlinear,
        };

        Ok(Surface {
            width,
            height,
            loader,
            handle,
            format,
        })
    }

    #[cfg(all(unix, not(target_os = "android")))]
    fn create_surface_khr(window: &winit::Window) -> Result<vk::SurfaceKHR, Box<Error>> {
        use winit::os::unix::WindowExt;
        use ash::extensions::XlibSurface;

        let x11_display = window.get_xlib_display().ok_or("Couldn't get xlib display")?;
        let x11_window = window.get_xlib_window().ok_or("Couldn't get xlib window")?;

        let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
            s_type: vk::StructureType::XlibSurfaceCreateInfoKhr,
            p_next: ptr::null(),
            flags: Default::default(),
            window: x11_window as vk::Window,
            dpy: x11_display as *mut vk::Display,
        };

        let xlib_surface_loader =
            XlibSurface::new(VK_ENTRY, VK_INSTANCE).map_err(|errors| errors.join("\n"))?;

        let surface =
            unsafe { xlib_surface_loader.create_xlib_surface_khr(&x11_create_info, None)? };

        Ok(surface)
    }

    #[cfg(windows)]
    fn create_surface_khr(window: &winit::Window) -> Result<vk::SurfaceKHR, Box<Error>> {
        use winit::os::windows::WindowExt;
        use ash::extensions::Win32Surface;

        let hwnd = window.get_hwnd();

        let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::StructureType::Win32SurfaceCreateInfoKhr,
            p_next: ptr::null(),
            flags: Default::default(),
            hinstance: ptr::null(),
            hwnd: hwnd,
        };

        let win32_surface_loader =
            Win32Surface::new(VK_ENTRY, VK_INSTANCE).map_err(|errors| errors.join("\n"))?;

        let surface =
            unsafe { win32_surface_loader.create_win32_surface_khr(&win32_create_info, None)? };

        Ok(surface)
    }

    pub fn loader(&self) -> ext::Surface {
        self.loader
    }

    pub fn handle(&self) -> vk::SurfaceKHR {
        self.handle
    }

    fn set_format(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<vk::SurfaceFormatKHR, Box<Error>> {
        let surface_format = self.loader
            .get_physical_device_surface_formats_khr(physical_device, self.handle)?
            .iter()
            .map(|sfmt| match sfmt.format {
                vk::Format::Undefined => vk::SurfaceFormatKHR {
                    format: vk::Format::B8g8r8Unorm,
                    color_space: sfmt.color_space,
                },
                _ => sfmt.clone(),
            })
            .nth(0)
            .ok_or("Couldn't get physical device surface formats")?;

        Ok(surface_format)
    }

    fn set_extent(&self, physical_device: vk::PhysicalDevice) -> Result<vk::Extent2D, Box<Error>> {
        let surface_capabilities = self.loader
            .get_physical_device_surface_capabilities_khr(physical_device, self.handle)?;

        let surface_resolution = match surface_capabilities.current_extent.width {
            u32::MAX => vk::Extent2D {
                width: self.width,
                height: self.height,
            },
            _ => surface_capabilities.current_extent,
        };

        Ok(surface_resolution)
    }
}
