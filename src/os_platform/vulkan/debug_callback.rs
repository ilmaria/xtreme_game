use ash::vk;
use ash::extensions::DebugReport;

use std::ptr;
use std::error::Error;
use std::ffi::CStr;

use super::InstanceV10;
use super::EntryV10;

pub fn set_debug_callback(
    entry: &EntryV10,
    instance: &InstanceV10,
) -> Result<vk::DebugReportCallbackEXT, Box<Error>> {
    let debug_info = vk::DebugReportCallbackCreateInfoEXT {
        s_type: vk::StructureType::DebugReportCallbackCreateInfoExt,
        p_next: ptr::null(),
        flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT |
            vk::DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
        pfn_callback: vulkan_debug_callback,
        p_user_data: ptr::null_mut(),
    };

    let debug_report_loader = DebugReport::new(entry, instance).map_err(
        |_| "Couldn't create debug repoprt loader",
    )?;

    let callback = unsafe {
        debug_report_loader.create_debug_report_callback_ext(
            &debug_info,
            None,
        )?
    };

    Ok(callback)
}

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: vk::uint64_t,
    _: vk::size_t,
    _: vk::int32_t,
    _: *const vk::c_char,
    p_message: *const vk::c_char,
    _: *mut vk::c_void,
) -> u32 {
    println!("{:?}", CStr::from_ptr(p_message));
    1
}