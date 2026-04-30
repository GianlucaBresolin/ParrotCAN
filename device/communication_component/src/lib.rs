#![no_std]

const CAN_BASE: *mut u32 = 0x40006400 as *mut u32;

pub struct CommunicationComponent;

impl CommunicationComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn send_data(&self) {
        // write MMIO registers to send data CAN frame (RTR = 0)
        unsafe {
            core::ptr::write_volatile(CAN_BASE.offset(0), 0x123); // ID
            core::ptr::write_volatile(CAN_BASE.offset(1), 0);     // RTR
            core::ptr::write_volatile(CAN_BASE.offset(2), 8);     // DLC
            core::ptr::write_volatile(CAN_BASE.offset(3), 0xDE);  // Data low
            core::ptr::write_volatile(CAN_BASE.offset(4), 0xAD);  // Data high
            core::ptr::write_volatile(CAN_BASE.offset(5), 1);     // Command: send frame
        }
    }

    pub fn request_data(&self) {
        // write MMIO register to send a request frame (RTR = 1)
        unsafe {
            core::ptr::write_volatile(CAN_BASE.offset(0), 0x123); // ID
            core::ptr::write_volatile(CAN_BASE.offset(1), 1);     // RTR
            core::ptr::write_volatile(CAN_BASE.offset(2), 8);     // DLC (ignored for RTR)
            core::ptr::write_volatile(CAN_BASE.offset(3), 0xDE);  // Data low
            core::ptr::write_volatile(CAN_BASE.offset(4), 0xAD);  // Data high
            core::ptr::write_volatile(CAN_BASE.offset(5), 1);     // Command: send frame
        }
    }

    pub fn receive(&self) {
        // TODO: process incoming packet (ID, DLC, data)
    }
}

