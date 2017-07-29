use ash::vk;
use ash::extensions::{XlibSurface, Surface};
use winit;

use std::ptr;
use std::u32;
use std::error::Error;

use super::Renderer;
use super::RendererError;

impl Renderer {
    pub fn create_surface_loader(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let entry = self.entry.ok_or(RendererError::NoEntry)?;
        let instance = self.instance.ok_or(RendererError::NoInstance)?;

        let loader = Surface::new(&entry, &instance).map_err(
            |_| "Couldn't create surface loader",
        )?;

        self.surface_loader = Some(loader);

        Ok(self)
    }

    #[cfg(all(unix, not(target_os = "android")))]
    pub fn create_surface(&mut self) -> Result<&mut Renderer, Box<Error>> {
        use winit::os::unix::WindowExt;

        let entry = self.entry.ok_or(RendererError::NoEntry)?;
        let instance = self.instance.ok_or(RendererError::NoInstance)?;
        let window = self.window.ok_or(RendererError::NoWindow)?;

        let x11_display = window.get_xlib_display().ok_or("Couldn't get xlib display")?;
        let x11_window = window.get_xlib_window().ok_or("Couldn't get xlib window")?;

        let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
            s_type: vk::StructureType::XlibSurfaceCreateInfoKhr,
            p_next: ptr::null(),
            flags: Default::default(),
            window: x11_window as vk::Window,
            dpy: x11_display as *mut vk::Display,
        };

        let xlib_surface_loader = XlibSurface::new(&entry, &instance).map_err(
            |_| "Unable to load xlib surface",
        )?;

        let surface = unsafe {
            xlib_surface_loader.create_xlib_surface_khr(
                &x11_create_info,
                None,
            )?
        };

        self.surface = Some(surface);

        Ok(self)
    }

    #[cfg(windows)]
    pub fn create_surface(&mut self) -> Result<&mut Renderer, Box<Error>> {
        use winit::os::windows::WindowExt;

        let entry = self.entry.ok_or(RendererError::NoEntry)?;
        let instance = self.instance.ok_or(RendererError::NoInstance)?;
        let window = self.window.ok_or(RendererError::NoWindow)?;

        let hwnd = window.get_hwnd().ok_or("Couldn't get window hwnd")?;

        let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::StructureType::Win32SurfaceCreateInfoKhr,
            p_next: ptr::null(),
            flags: Default::default(),
            hinstance: ptr::null(),
            hwnd: hwnd as *const (),
        };

        let win32_surface_loader = Win32Surface::new(&entry, &instance).map_err(
            |_| "Unable to load xlib surface",
        )?;

        let surface = unsafe {
            win32_surface_loader.create_win32_surface_khr(
                &win32_create_info,
                None,
            )?
        };

        self.surface = Some(surface);

        Ok(self)
    }

    pub fn choose_surface_format(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let physical_device = self.physical_device.ok_or(RendererError::NoPhysicalDevice)?;
        let surface = self.surface.ok_or(RendererError::NoSurface)?;
        let surface_loader = self.surface_loader.ok_or(RendererError::NoSurfaceLoader)?;

        let surface_format = surface_loader
            .get_physical_device_surface_formats_khr(physical_device, surface.clone())?
            .iter()
            .map(|sfmt| match sfmt.format {
                vk::Format::Undefined => {
                    vk::SurfaceFormatKHR {
                        format: vk::Format::B8g8r8Unorm,
                        color_space: sfmt.color_space,
                    }
                }
                _ => sfmt.clone(),
            })
            .nth(0)
            .ok_or("Couldn't get physical device surface formats")?;

        self.surface_format = Some(surface_format);

        Ok(self)
    }
}