#![cfg_attr(not(target_os = "windows"), deny(unsafe_code))]

use std::ffi::CStr;

use ash::vk::{ApplicationInfo, InstanceCreateInfo, make_api_version, QueueFlags};
use ash::Entry;

pub fn main() {
    let entry = Entry::linked();

    let application_name = unsafe {
        CStr::from_bytes_with_nul_unchecked(b"OMSI 2 (Rust Edition)\0")
    };
    let engine_name = unsafe {
        CStr::from_bytes_with_nul_unchecked(b"Rust 1.60.0-nightly (5d8767cb2 2022-02-12)\0")
    };
    let application_info = ApplicationInfo::builder()
        .api_version(make_api_version(0, 1, 3, 0))
        .application_name(application_name)
        .application_version(000) // version 0.0.0
        .engine_name(engine_name)
        .engine_version(1600); // version 1.60.0
    let instance_create_info = InstanceCreateInfo::builder()
        .application_info(&application_info);

    let instance = unsafe {
        entry.create_instance(&instance_create_info, None)
    }
    .expect("could not create Vulkan instance");

    let enumerate_physical_devices = unsafe {
        instance.enumerate_physical_devices()
    }
    .expect("could not enumerate of Vulkan physical devices");
    let suitable_physical_device = enumerate_physical_devices
        .iter()
        .map(|physical_device| {
            unsafe {
                instance
                    .get_physical_device_queue_family_properties(*physical_device)
                    .iter()
                    .filter_map(|queue_family_properties| {
                        let supports_graphics = queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS);

                        if supports_graphics {
                            Some(*physical_device)
                        } else {
                            None
                        }
                    })
                    .next()
            }
        })
        .flatten()
        .next()
        .expect("could not find suitable device");
    let physical_device_properties = unsafe {
        instance.get_physical_device_properties(suitable_physical_device.clone())
    };
    let device_name_bytes = physical_device_properties.device_name.map(|byte| byte as u8);
    let device_name = unsafe {
        CStr::from_bytes_with_nul_unchecked(device_name_bytes.as_slice())
    };
    println!("{}", device_name.to_str().unwrap());
}
