#![no_std]

const CAN_BASE: *mut u32 = 0x40006400 as *mut u32;

pub struct CommunicationComponent;

impl CommunicationComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn send(&self) {
        // write MMIO registers to send CAN frame
        unsafe {
            core::ptr::write_volatile(CAN_BASE.offset(0), 0x123); // ID
            // core::ptr::write_volatile(CAN_BASE.offset(1), 8);     // DLC
            // core::ptr::write_volatile(CAN_BASE.offset(2), 0xDEADBEEF); // Data Low
            // core::ptr::write_volatile(CAN_BASE.offset(3), 0xCAFEBABE); // Data High
            // core::ptr::write_volatile(CAN_BASE.offset(4), 1);     // Command to send
        }
    }

    pub fn receive(&self) {
        // todo 
    }
}

