mod devinfo;

use devinfo::DevInfo;
use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
use windows::Win32::NetworkManagement::IpHelper::{GetAdaptersInfo, IP_ADAPTER_INFO};
use std::alloc::Layout;

extern crate alloc;

fn get_iftable() -> Result<*mut IP_ADAPTER_INFO,&'static str>{ 
    let mut buf_len: u32 = 0;
    let layout = Layout::new::<IP_ADAPTER_INFO>();
    let piftable = unsafe {std::alloc::alloc_zeroed(layout) as *mut IP_ADAPTER_INFO};
    unsafe {
        let buf_len_ptr = &mut buf_len;
        if GetAdaptersInfo(None, buf_len_ptr) == ERROR_BUFFER_OVERFLOW.0 {
            let piftable_sized = alloc::alloc::realloc(piftable as *mut u8, layout, {*buf_len_ptr}.try_into().unwrap());
            GetAdaptersInfo(Some(piftable_sized as *mut IP_ADAPTER_INFO), buf_len_ptr);
            Ok(piftable_sized as *mut IP_ADAPTER_INFO)
        }
        else{
            Err("Unable to get adapters!") 
        }
    }
}

pub fn get_dev_by_name(adpt_name: String) -> Result<DevInfo, &'static str>{
    let devices: Vec<DevInfo> = get_all_devices()?;
    devices.into_iter().find(|device| device.dev_name == adpt_name)
                        .ok_or("Device not found")

}

pub fn get_all_devices() -> Result<Vec<DevInfo>, &'static str>{
    let mut devices: Vec<DevInfo> = Vec::new();
    let mut cur_dev = get_iftable()?;

    while !cur_dev.is_null() {
        let device = DevInfo::from_adapter_info(cur_dev)?; 
        devices.push(device);
        cur_dev = unsafe {*cur_dev}.Next;
    }

    Ok(devices)
}

