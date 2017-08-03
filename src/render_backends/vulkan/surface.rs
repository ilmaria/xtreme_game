use ash::vk;
use ash::Entry;
use ash::Instance;
use ash::version::V1_0;
use ash::extensions::{XlibSurface, Surface};
use winit;

use std::ptr;
use std::u32;
use std::error::Error;

pub fn new_loader(entry: &Entry<V1_0>, instance: &Instance<V1_0>) -> Result<Surface, Box<Error>> {
    let loader = Surface::new(entry, instance).map_err(
        |_| "Couldn't create surface loader",
    )?;

    Ok(loader)
}

#[cfg(all(unix, not(target_os = "android")))]
pub fn new(
    entry: &Entry<V1_0>,
    instance: &Instance<V1_0>,
    window: &winit::Window,
) -> Result<vk::SurfaceKHR, Box<Error>> {
    use winit::os::unix::WindowExt;

    let x11_display = window.get_xlib_display().ok_or("Couldn't get xlib display")?;
    let x11_window = window.get_xlib_window().ok_or("Couldn't get xlib window")?;

    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        s_type: vk::StructureType::XlibSurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: Default::default(),
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display,
    };

    let xlib_surface_loader = XlibSurface::new(entry, instance).map_err(
        |_| "Unable to load xlib surface",
    )?;

    let surface = unsafe {
        xlib_surface_loader.create_xlib_surface_khr(
            &x11_create_info,
            None,
        )?
    };

    Ok(surface)
}

#[cfg(windows)]
pub fn new(
    entry: &Entry<V1_0>,
    instance: &Instance<V1_0>,
    window: winit::Window,
) -> Result<vk::SurfaceKHR, Box<Error>> {
    use winit::os::windows::WindowExt;

    let hwnd = window.get_hwnd().ok_or("Couldn't get window hwnd")?;

    let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
        s_type: vk::StructureType::Win32SurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: Default::default(),
        hinstance: ptr::null(),
        hwnd: hwnd as *const (),
    };

    let win32_surface_loader = Win32Surface::new(entry, instance).map_err(
        |_| "Unable to load xlib surface",
    )?;

    let surface = unsafe {
        win32_surface_loader.create_win32_surface_khr(
            &win32_create_info,
            None,
        )?
    };

    Ok(surface)
}

pub fn new_format(
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_loader: &Surface,
) -> Result<vk::SurfaceFormatKHR, Box<Error>> {
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

    Ok(surface_format)
}

pub fn new_resolution(
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_loader: &Surface,
    width: u32,
    height: u32,
) -> Result<vk::Extent2D, Box<Error>> {
    let surface_capabilities =
        surface_loader
            .get_physical_device_surface_capabilities_khr(physical_device, surface)?;

    let surface_resolution = match surface_capabilities.current_extent.width {
        u32::MAX => vk::Extent2D { width, height },
        _ => surface_capabilities.current_extent,
    };

    Ok(surface_resolution)
}