#![no_std]

use cortex_m_semihosting::hprintln;
use device_driver::DeviceDriver;

pub struct SimulatedLedDriver;

impl DeviceDriver for SimulatedLedDriver {
    fn turn_on(&mut self) {
        hprintln!("LED is turned ON");
    }
}

pub static mut SIMULATED_LED_DRIVER: SimulatedLedDriver = SimulatedLedDriver;