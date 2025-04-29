use std::{
    ffi::{c_char, c_void},
    str::FromStr,
};

use ash::ext::debug_utils;

fn main() {
    let entry = ash::Entry::linked();
    let instance = unsafe {
        let appinfo = ash::vk::ApplicationInfo::default()
            .application_version(0)
            .engine_version(0)
            .api_version(ash::vk::make_api_version(0, 1, 0, 0));
        let mut extension_names = Vec::default();
        extension_names.append(&mut vec![
            ash::ext::debug_utils::NAME.as_ptr(),
            ash::khr::get_physical_device_properties2::NAME.as_ptr(),
            ash::khr::portability_enumeration::NAME.as_ptr(),
	    // std::ffi::CString::from_str("VK_EXT_metal_objects").unwrap().as_ptr()
        ]);
        extension_names.push(debug_utils::NAME.as_ptr());

        let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
            ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            ash::vk::InstanceCreateFlags::default()
        };

        let layer_names = [c"VK_LAYER_KHRONOS_validation"];
        let layers_names_raw: Vec<*const c_char> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        let mut a = ash::vk::ExportMetalObjectCreateInfoEXT {
            s_type: ash::vk::StructureType::EXPORT_METAL_OBJECT_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            export_object_type: ash::vk::ExportMetalObjectTypeFlagsEXT::METAL_DEVICE,
            _marker: std::marker::PhantomData,
        };
        let instance_create_info = ash::vk::InstanceCreateInfo::default()
            .application_info(&appinfo)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names)
            .push_next(&mut a)
            .flags(create_flags);
        entry.create_instance(&instance_create_info, None)
    }
    .unwrap();
    println!("SAAAAA");

    let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    let physical_device = physical_devices[0];
    println!("{}", physical_devices.len());

    let device = unsafe {
        let features = ash::vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];
        let queue_family_index = 0;
        let queue_info = ash::vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index as u32)
            .queue_priorities(&priorities);
	let ext = std::ffi::CString::from_str("VK_EXT_metal_objects").unwrap();
        let device_extension_names_raw = [
            //ash::khr::swapchain::NAME.as_ptr(),
	    ext.as_ptr(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            ash::khr::portability_subset::NAME.as_ptr(),
        ];

        let device_create_info = ash::vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);
        ash::vk::DeviceCreateFlags::default();

        instance.create_device(physical_device, &device_create_info, None)
    }
    .unwrap();

    let metal_device_ptr: *mut c_void = std::ptr::null_mut();
    let metal_objects_device = ash::ext::metal_objects::Device::new(&instance, &device);
    let export_metal_device_info_ext = ash::vk::ExportMetalDeviceInfoEXT {
        s_type: ash::vk::StructureType::EXPORT_METAL_DEVICE_INFO_EXT,
        p_next: std::ptr::null(),
        // mtl_device: metal_device_ptr,
	mtl_device: std::ptr::null_mut(),
        _marker: std::marker::PhantomData {},
    };
    let mut export_metal_objects_info_ext = ash::vk::ExportMetalObjectsInfoEXT {
        s_type: ash::vk::StructureType::EXPORT_METAL_OBJECTS_INFO_EXT,
        p_next: (&export_metal_device_info_ext as *const _) as *const c_void,
        _marker: std::marker::PhantomData {},
    };
    unsafe {
        (metal_objects_device.fp().export_metal_objects_ext)(
            device.handle(),
            &mut export_metal_objects_info_ext,
        )
    };

    println!("{:?}", export_metal_objects_info_ext);

    let device = metal::Device::system_default().unwrap();
    if device.supports_raytracing() {
        println!("RayTracing Supported");
    } else {
        println!("RayTracing NOT Supported...");
    }

    println!("Hello, world!");
}
