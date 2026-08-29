#![no_std]

pub trait DeviceDriver {
    fn turn_on(&mut self);
}
