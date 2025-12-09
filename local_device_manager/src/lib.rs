#![no_std]

pub mod device_drivers;

use crate::device_drivers::DeviceDriver;

pub struct LocalDeviceManager<D: DeviceDriver> {
    led_driver: D,
}

impl<D: DeviceDriver> LocalDeviceManager<D> {
    pub fn new(led_driver: D) -> Self {
        Self {
            led_driver
        }
    }

    pub fn turn_on_light(&mut self) {
        self.led_driver.turn_on();
    }
}