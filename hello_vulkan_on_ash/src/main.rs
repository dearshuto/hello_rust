use std::{
    borrow::Cow,
    ffi::{c_char, CStr},
};

use ash::{
    extensions::ext::DebugUtils,
    vk::{self, KhrGetPhysicalDeviceProperties2Fn},
};

fn main() {
    let entry = unsafe { ash::Entry::new() }.unwrap();
    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };
    let layer_names = unsafe {
        [CStr::from_bytes_with_nul_unchecked(
            b"VK_LAYER_KHRONOS_validation\0",
        )]
    };

    // for i in entry.enumerate_instance_layer_properties().unwrap() {
    //     println!("{:?}", i);
    // }
    let layers_names_raw: Vec<*const c_char> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();
    let binding = [
        DebugUtils::name().as_ptr(),
        KhrGetPhysicalDeviceProperties2Fn::name().as_ptr(),
    ];
    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layers_names_raw)
        .enabled_extension_names(&binding);

    let instance = unsafe { entry.create_instance(&create_info, None) }.unwrap();

    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING, // | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(vulkan_debug_callback))
        .build();
    let debug_utils_loader = DebugUtils::new(&entry, &instance);

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

    let queue_create_info = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index as u32)
        .queue_priorities(&[1.0])
        .build();

    // 物理デバイスに VK_KHR_portability_subset が含まれていたらデバイス拡張に含める必要がある
    let device_extensions = ["VK_KHR_portability_subset".as_ptr() as *const i8];
    let required_device_extensions: Vec<*const i8> = device_extensions
        .into_iter()
        .filter(|extension| {
            for physical_device in &physical_devices {
                let properties =
                    unsafe { instance.enumerate_device_extension_properties(*physical_device) }
                        .unwrap();
                for property in properties {
                    let extension_name =
                        unsafe { CStr::from_ptr(property.extension_name.as_ptr()) };
                    let target = unsafe { CStr::from_ptr(*extension) };
                    if extension_name == target {
                        return true;
                    } else {
                        continue;
                    }
                }
            }

            false
        })
        .collect();
    let device_create_info = vk::DeviceCreateInfo::builder()
        .enabled_extension_names(&required_device_extensions)
        .queue_create_infos(std::slice::from_ref(&queue_create_info));
    let device =
        unsafe { instance.create_device(physical_device, &device_create_info, None) }.unwrap();
    let _queue = unsafe { device.get_device_queue(queue_family_index as u32, 0) };

    // let shader_module_create_info = vk::ShaderModuleCreateInfo::builder().code(&[]);
    // let shader_module =
    //     unsafe { device.create_shader_module(&shader_module_create_info, None) }.unwrap();

    let descriptor_set_layout_binding = [vk::DescriptorSetLayoutBinding::builder().build()];
    let descriptor_set_create_info =
        vk::DescriptorSetLayoutCreateInfo::builder().bindings(&descriptor_set_layout_binding);
    let descriptor_set_layout =
        unsafe { device.create_descriptor_set_layout(&descriptor_set_create_info, None) }.unwrap();
    // let pipeline_create_info = vk::PipelineLayoutCreateInfo::builder()
    //     .set_layouts(&[])
    //     .build();
    // let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_create_info, None) };

    let command_pool_crate_info = vk::CommandPoolCreateInfo::builder()
        .queue_family_index(queue_family_index as u32)
        .build();
    let command_pool =
        unsafe { device.create_command_pool(&command_pool_crate_info, None) }.unwrap();

    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .command_buffer_count(1);
    let command_buffers =
        unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }.unwrap();
    // let command_buffer = command_buffers[0];

    let _spv = include_bytes!("compute.spv");
    // let shader_object = ash::extensions::ext::ShaderObject::new(&instance, &device);
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
