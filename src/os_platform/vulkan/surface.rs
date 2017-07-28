use ash::vk;
use ash::extensions::{XlibSurface, Surface};
use winit;

use std::ptr;
use std::u32;
use std::error::Error;

use super::EntryV10;
use super::InstanceV10;
use super::SurfaceDetails;
use super::physical_device::pick_physical_device;

pub fn create_surface(
    entry: &EntryV10,
    instance: &InstanceV10,
    window: &winit::Window,
    window_width: u32,
    window_height: u32,
) -> Result<SurfaceDetails, Box<Error>> {
    let surface_khr = create_surface_khr(entry, instance, window)?;

    let loader = Surface::new(entry, instance).map_err(
        |_| "Couldn't create surface loader",
    )?;

    let (physical_device, _queue_family_index) =
        pick_physical_device(&instance, &surface_khr, &loader)?;

    let format = choose_surface_format(physical_device, &surface_khr, &loader)?;

    let capabilities = loader.get_physical_device_surface_capabilities_khr(
        physical_device,
        surface_khr,
    )?;

    let resolution = match capabilities.clone().current_extent.width {
        u32::MAX => {
            vk::Extent2D {
                width: window_width,
                height: window_height,
            }
        }
        _ => capabilities.clone().current_extent,
    };

    Ok(SurfaceDetails {
        khr: surface_khr,
        loader: loader,
        format: format,
        capabilities: capabilities,
        resolution: resolution,
    })
}

#[cfg(all(unix, not(target_os = "android")))]
fn create_surface_khr(
    entry: &EntryV10,
    instance: &InstanceV10,
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
fn create_surface_khr(
    entry: &Entry<V1_0>,
    instance: &InstanceV10,
    window: &winit::Window,
) -> Result<vk::SurfaceKHR, Box<Error>> {
    use winit::os::windows::WindowExt;
    let hwnd = window.get_hwnd();
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
        s_type: vk::StructureType::Win32SurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: Default::default(),
        hinstance: ptr::null(),
        hwnd: hwnd as *const (),
    };
    let win32_surface_loader = Win32Surface::new(entry, instance)?;
    win32_surface_loader.create_win32_surface_khr(&win32_create_info, None)
}

fn choose_surface_format(
    physical_device: vk::types::PhysicalDevice,
    surface_khr: &vk::SurfaceKHR,
    surface_loader: &Surface,
) -> Result<vk::SurfaceFormatKHR, Box<Error>> {
    let surface_format = surface_loader
        .get_physical_device_surface_formats_khr(physical_device, surface_khr.clone())?
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