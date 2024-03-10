use std::{
    borrow::Cow,
    ffi::{c_char, CStr},
};

use ash::{ext::debug_utils, vk};

fn main() {
    let entry = ash::Entry::linked();
    let app_info = vk::ApplicationInfo::default()
        .application_version(0)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));
    let layer_names = unsafe {
        [CStr::from_bytes_with_nul_unchecked(
            b"VK_LAYER_KHRONOS_validation\0",
        )]
    };

    let layers_names_raw: Vec<*const c_char> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();
    let binding = [
        ash::ext::debug_utils::NAME.as_ptr(),
        ash::khr::get_physical_device_properties2::NAME.as_ptr(),
        ash::khr::portability_enumeration::NAME.as_ptr(),
    ];

    let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::default()
    };

    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_layer_names(&layers_names_raw)
        .enabled_extension_names(&binding)
        .flags(create_flags);

    let instance = unsafe { entry.create_instance(&create_info, None) }.unwrap();

    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING, // | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(vulkan_debug_callback));
    let debug_utils_loader = debug_utils::Instance::new(&entry, &instance);

    let debug_callback =
        unsafe { debug_utils_loader.create_debug_utils_messenger(&debug_info, None) }.unwrap();

    let physical_devices = unsafe { instance.enumerate_physical_devices() }.unwrap();
    let (physical_device, queue_family_index): (vk::PhysicalDevice, usize) = physical_devices
        .iter()
        .find_map(|physical_device| {
            unsafe { instance.get_physical_device_queue_family_properties(*physical_device) }
                .iter()
                .enumerate()
                .find_map(|(index, info)| {
                    let is_supported = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                    if is_supported {
                        Some((*physical_device, index))
                    } else {
                        None
                    }
                })
        })
        .unwrap();

    let queue_create_info = vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index as u32)
        .queue_priorities(&[1.0]);

    let device_extension_names_raw = [
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        ash::khr::portability_subset::NAME.as_ptr(),
    ];
    let device_create_info = vk::DeviceCreateInfo::default()
        .enabled_extension_names(&device_extension_names_raw)
        .queue_create_infos(std::slice::from_ref(&queue_create_info));
    let device =
        unsafe { instance.create_device(physical_device, &device_create_info, None) }.unwrap();
    let _queue = unsafe { device.get_device_queue(queue_family_index as u32, 0) };

    // let shader_module_create_info = vk::ShaderModuleCreateInfo::default().code(&[]);
    // let _shader_module =
    //     unsafe { device.create_shader_module(&shader_module_create_info, None) }.unwrap();

    let descriptor_set_layout_binding = [vk::DescriptorSetLayoutBinding::default()];
    let descriptor_set_create_info =
        vk::DescriptorSetLayoutCreateInfo::default().bindings(&descriptor_set_layout_binding);
    let descriptor_set_layout =
        unsafe { device.create_descriptor_set_layout(&descriptor_set_create_info, None) }.unwrap();
    // let pipeline_create_info = vk::PipelineLayoutCreateInfo::builder()
    //     .set_layouts(&[])
    //     .build();
    // let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_create_info, None) };

    let command_pool_crate_info =
        vk::CommandPoolCreateInfo::default().queue_family_index(queue_family_index as u32);
    let command_pool =
        unsafe { device.create_command_pool(&command_pool_crate_info, None) }.unwrap();

    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .command_buffer_count(1);
    let command_buffers =
        unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }.unwrap();
    // let command_buffer = command_buffers[0];

    let _spv = include_bytes!("compute.spv");
    // ash::ext::shader_object::Device::new(&instance, &device).cmd_shad
    // unsafe{ device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, , 0, &[/*descriptor_set*/], &[]) }
    // unsafe { device.cmd_dispatch(command_buffer, 8, 8, 8) }

    // let submit_info = vk::SubmitInfo::builder().command_buffers(&[command_buffer]);
    // device.queue_submit(queue, &[submit_info], fence);

    unsafe { device.destroy_descriptor_set_layout(descriptor_set_layout, None) };
    unsafe { device.free_command_buffers(command_pool, &command_buffers) };
    unsafe { device.destroy_command_pool(command_pool, None) };
    unsafe { device.destroy_device(None) };
    unsafe { debug_utils_loader.destroy_debug_utils_messenger(debug_callback, None) };
    unsafe { instance.destroy_instance(None) };
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}
