#![no_std]

use core::ffi::c_int;
use rdmacm_sys::{rdma_free_devices, rdma_get_devices};

fn main() {
    let mut num_devices: c_int = 0;
    unsafe {
        let devices = rdma_get_devices(&mut num_devices);
        if devices.is_null() {
            panic!("no device found");
        }
        for i in 0..num_devices {
            let device = *devices.add(i as usize);
            if device.is_null() {
                panic!("device is null");
            }
        }
        rdma_free_devices(devices);
    }
}
