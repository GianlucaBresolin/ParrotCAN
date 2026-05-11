#![no_std]

mod parrot;
mod can_frame;

use crate::can_frame::CANFrame;

const CAN_BASE: *mut u32 = 0x40006400 as *mut u32;
const CAN_ID_MASK: u16 = 0x7FF; // 11-bit ID mask (2047)

pub struct CommunicationComponent {
    my_IDs: [u16; 10],
    my_IDs_count: usize,
    interested_IDs: [u16; 10],
    interested_IDs_count: usize,
}

impl CommunicationComponent {
    pub fn new(
        my_IDs: &[u16], 
        interested_IDs: &[u16]
    ) -> Self {
        let mut comm_comp = Self {
            my_IDs: [0; 10],
            my_IDs_count: 0,
            interested_IDs: [0; 10],
            interested_IDs_count: 0,
        };

        for (i, &id) in my_IDs.iter().enumerate().take(10) {
            comm_comp.my_IDs[i] = id & CAN_ID_MASK;
            comm_comp.my_IDs_count = i + 1;
        }

        for (i, &id) in interested_IDs.iter().enumerate().take(10) {
            comm_comp.interested_IDs[i] = id & CAN_ID_MASK;
            comm_comp.interested_IDs_count = i + 1;
        }

        comm_comp
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
        let frame: CANFrame = unsafe {
            let id = core::ptr::read_volatile(CAN_BASE.offset(0)) as u16;
            let rtr = core::ptr::read_volatile(CAN_BASE.offset(1)) != 0;
            let dlc = core::ptr::read_volatile(CAN_BASE.offset(2)) as u8;
            let data_low = core::ptr::read_volatile(CAN_BASE.offset(3)) as u32;
            let data_high = core::ptr::read_volatile(CAN_BASE.offset(4)) as u32;
            CANFrame { id, rtr, dlc, data_low, data_high }
        };

        self.check_frame(frame);

        if self.interested_IDs[..self.interested_IDs_count].contains(&frame.id) {
            // TODO: pass frame data to app component
        }
    }
}
