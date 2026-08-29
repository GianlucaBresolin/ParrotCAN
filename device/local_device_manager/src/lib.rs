#![no_std]

use device_driver::DeviceDriver;
use simulated_led_driver::SIMULATED_LED_DRIVER;

pub static mut LOCAL_DEVICE_MANAGER: Option<LocalDeviceManager> = None;

pub struct LocalDeviceManager {
    led_driver: *mut dyn DeviceDriver,
}

impl LocalDeviceManager {
    pub fn new(led_driver: *mut dyn DeviceDriver) -> Self {
        Self {
            led_driver
        }
    }

    pub fn turn_on_light(&mut self) {
        unsafe {
            (&mut *self.led_driver).turn_on();
        }
    }
}

pub fn init() {
    let led_driver: *mut dyn DeviceDriver = {
        core::ptr::addr_of_mut!(SIMULATED_LED_DRIVER)
    };

    unsafe {
        LOCAL_DEVICE_MANAGER = Some(LocalDeviceManager::new(led_driver));
    }
}