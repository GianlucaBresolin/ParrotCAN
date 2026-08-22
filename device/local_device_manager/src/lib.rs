#![no_std]

pub mod device_drivers;

pub static mut LOCAL_DEVICE_MANAGER: Option<LocalDeviceManager> = None;

use crate::device_drivers::DeviceDriver;

pub struct LocalDeviceManager {
    led_driver: &'static mut dyn DeviceDriver,
}

impl LocalDeviceManager {
    pub fn new(led_driver: &'static mut dyn DeviceDriver) -> Self {
        Self {
            led_driver
        }
    }

    pub fn turn_on_light(&mut self) {
        self.led_driver.turn_on();
    }
}

pub fn init(device_driver: &'static mut dyn DeviceDriver) {
    unsafe {
        LOCAL_DEVICE_MANAGER = Some(LocalDeviceManager::new(device_driver));
    }
}