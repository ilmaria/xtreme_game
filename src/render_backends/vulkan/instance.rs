use ash::vk;
use ash::version::EntryV1_0;
use ash::extensions::{XlibSurface, Surface, DebugReport, Win32Surface};

use std::ptr;
use std::error::Error;
use std::ffi::CString;

use super::VulkanRenderer;
use super::RendererError;

impl VulkanRenderer {
    pub fn create_instance(&mut self) -> Result<&mut VulkanRenderer, Box<Error>> {
        let entry = self.entry.ok_or(RendererError::NoEntry)?;

        let app_name = CString::new("Xtreme Game")?.as_ptr();

        let appinfo = vk::ApplicationInfo {
            p_application_name: app_name,
            s_type: vk::StructureType::ApplicationInfo,
            p_next: ptr::null(),
            application_version: 0,
            p_engine_name: app_name,
            engine_version: 0,
            api_version: vk_make_version!(1, 0, 36),
        };

        let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation")?];
        let layers_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let os_surface = if cfg!(windows) {
            Win32Surface::name()
        } else
        /*if cfg!(all(unix, not(target_os = "android")))*/
        {
            XlibSurface::name()
        };

        let extension_names = vec![
            Surface::name().as_ptr(),
            os_surface.as_ptr(),
            DebugReport::name().as_ptr(),
        ];

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::InstanceCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            p_application_info: &appinfo,
            pp_enabled_layer_names: layers_names_raw.as_ptr(),
            enabled_layer_count: layers_names_raw.len() as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        self.instance = Some(entry.create_instance(&create_info, None)?);

        Ok(self)
    }
}