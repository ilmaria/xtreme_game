use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;
use std::u64;
use std::error::Error;

pub fn new_pool(
    device: &DeviceV1_0,
    queue_family_index: u32,
) -> Result<vk::CommandPool, Box<Error>> {
    let pool_create_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::CommandPoolCreateInfo,
        p_next: ptr::null(),
        flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
        queue_family_index,
    };

    let command_pool = unsafe { device.create_command_pool(&pool_create_info, None)? };

    Ok(command_pool)
}

pub fn new_buffers(
    device: &DeviceV1_0,
    command_pool: vk::CommandPool,
    count: u32,
) -> Result<Vec<vk::CommandBuffer>, Box<Error>> {

    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::CommandBufferAllocateInfo,
        p_next: ptr::null(),
        command_buffer_count: count,
        command_pool,
        level: vk::CommandBufferLevel::Primary,
    };

    let command_buffers = unsafe {
        device
            .allocate_command_buffers(&command_buffer_allocate_info)?
    };

    Ok(command_buffers)
}

pub fn submit<F: FnOnce(&DeviceV1_0, vk::CommandBuffer)>(
    device: &DeviceV1_0,
    command_buffer: vk::CommandBuffer,
    submit_queue: vk::Queue,
    wait_mask: &[vk::PipelineStageFlags],
    wait_semaphores: &[vk::Semaphore],
    signal_semaphores: &[vk::Semaphore],
    f: F,
) -> Result<(), Box<Error>> {
    unsafe {
        device.reset_command_buffer(
            command_buffer,
            vk::COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT,
        )?;
    }
    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::CommandBufferBeginInfo,
        p_next: ptr::null(),
        p_inheritance_info: ptr::null(),
        flags: vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
    };

    unsafe {
        device
            .begin_command_buffer(command_buffer, &command_buffer_begin_info)?;
    }

    f(device, command_buffer);

    unsafe {
        device.end_command_buffer(command_buffer)?;
    }

    let fence_create_info = vk::FenceCreateInfo {
        s_type: vk::StructureType::FenceCreateInfo,
        p_next: ptr::null(),
        flags: vk::FenceCreateFlags::empty(),
    };
    let submit_fence = unsafe { device.create_fence(&fence_create_info, None)? };
    let submit_info = vk::SubmitInfo {
        s_type: vk::StructureType::SubmitInfo,
        p_next: ptr::null(),
        wait_semaphore_count: wait_semaphores.len() as u32,
        p_wait_semaphores: wait_semaphores.as_ptr(),
        p_wait_dst_stage_mask: wait_mask.as_ptr(),
        command_buffer_count: 1,
        p_command_buffers: &command_buffer,
        signal_semaphore_count: signal_semaphores.len() as u32,
        p_signal_semaphores: signal_semaphores.as_ptr(),
    };
    unsafe {
        device
            .queue_submit(submit_queue, &[submit_info], submit_fence)?;

        device.wait_for_fences(&[submit_fence], true, u64::MAX)?;
        device.destroy_fence(submit_fence, None);
    }

    Ok(())
}
