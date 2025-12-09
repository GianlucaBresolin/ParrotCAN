#![no_std]

use local_device_manager::device_drivers::DeviceDriver;
use cortex_m_semihosting::hprintln;

pub struct SimulatedLedDriver;

impl DeviceDriver for SimulatedLedDriver {
    fn turn_on(&mut self) { 
        hprintln!("LED is turned ON");
    }
} 