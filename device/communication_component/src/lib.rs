#![no_std]

mod parrot;
mod can_frame;
pub mod frame_listener;

use crate::can_frame::CANFrame;
use crate::frame_listener::FrameListener;

use cortex_m_semihosting::hprintln;

pub const CAN_BASE: *mut u32 = 0x40006400 as *mut u32;
pub const CAN_ID_MASK: u16 = 0x7FF; // 11-bit ID mask (2047)

pub static mut COMMUNICATION_COMPONENT: Option<CommunicationComponent> = None;

pub struct CommunicationComponent {
    my_ids: [u16; 10],
    my_ids_count: usize,
    interested_ids: [u16; 10],
    interested_ids_count: usize,
    listener: *mut dyn FrameListener, 
    tx_flag: bool
}

impl CommunicationComponent {
    pub fn new(
        my_ids: &[u16], 
        interested_ids: &[u16], 
        listener: *mut dyn FrameListener
    ) -> Self {
        let mut comm_comp = Self {
            my_ids: [0; 10],
            my_ids_count: 0,
            interested_ids: [0; 10],
            interested_ids_count: 0,
            listener,
            tx_flag: false
        };

        for (i, &id) in my_ids.iter().enumerate().take(10) {
            comm_comp.my_ids[i] = id & CAN_ID_MASK;
            comm_comp.my_ids_count = i + 1;
        }

        for (i, &id) in interested_ids.iter().enumerate().take(10) {
            comm_comp.interested_ids[i] = id & CAN_ID_MASK;
            comm_comp.interested_ids_count = i + 1;
        }

        comm_comp
    }

    pub fn send(
        &mut self, 
        id: u16,
        dlc: u8, 
        data: &[u8]
    ) {
        if id > CAN_ID_MASK || dlc > 8 || data.len() > 8 {
            // invalid parameters
            hprintln!("wrong parameters");
            return; 
        }

        let data_low: u32 = ((data.get(3).copied().unwrap_or(0) as u32) << 24)
                            | ((data.get(2).copied().unwrap_or(0) as u32) << 16)
                            | ((data.get(1).copied().unwrap_or(0) as u32) <<  8)
                            | ((data.get(0).copied().unwrap_or(0) as u32)      );

        let data_high: u32 = ((data.get(7).copied().unwrap_or(0) as u32) << 24)
                            | ((data.get(6).copied().unwrap_or(0) as u32) << 16)
                            | ((data.get(5).copied().unwrap_or(0) as u32) <<  8)
                            | ((data.get(4).copied().unwrap_or(0) as u32)      );
        
        // write MMIO registers to send data CAN frame (RTR = 0)
        unsafe {
            core::ptr::write_volatile(CAN_BASE.offset(0), id as u32); // ID
            core::ptr::write_volatile(CAN_BASE.offset(1), 0);     // RTR
            core::ptr::write_volatile(CAN_BASE.offset(2), dlc as u32);     // DLC
            core::ptr::write_volatile(CAN_BASE.offset(3), data_low);  // Data low
            core::ptr::write_volatile(CAN_BASE.offset(4), data_high);  // Data high
            core::ptr::write_volatile(CAN_BASE.offset(5), 1);     // Command: send frame
        }

        self.tx_flag = true;
    }

    pub fn receive(&mut self) {
        let frame: CANFrame = unsafe {
            let id = core::ptr::read_volatile(CAN_BASE.offset(0)) as u16;
            let rtr = core::ptr::read_volatile(CAN_BASE.offset(1)) != 0;
            let dlc = core::ptr::read_volatile(CAN_BASE.offset(2)) as u8;
            let data_low = core::ptr::read_volatile(CAN_BASE.offset(3)) as u32;
            let data_high = core::ptr::read_volatile(CAN_BASE.offset(4)) as u32;
            let _cmd = core::ptr::read_volatile(CAN_BASE.offset(5)) as u32; // cmd to remove read CAN frame from rx_queue
            CANFrame { id, rtr, dlc, data_low, data_high }
        };

        // Parrots Defense Algorithm
        self.check_frame(frame);

        if self.interested_ids[..self.interested_ids_count].contains(&frame.id) {
            let payload: [u8; 8] = [
                (frame.data_low  >> 24) as u8,
                (frame.data_low  >> 16) as u8,
                (frame.data_low  >>  8) as u8,
                (frame.data_low        ) as u8,
                (frame.data_high >> 24) as u8,
                (frame.data_high >> 16) as u8,
                (frame.data_high >>  8) as u8,
                (frame.data_high      ) as u8,
            ];
            if payload.iter().any(|&b| b != 0) {
                // not a D message
                unsafe {
                    (&mut *self.listener).on_interesting_frame(&payload[..frame.dlc as usize]);
                }
            }
        }

        // Reset Tx Flag
        self.tx_flag = false;
    }

    pub fn check_frame(
        &mut self, 
        frame: CANFrame
    ) {
    if self.my_ids[..self.my_ids_count].contains(&frame.id) {
            if !self.tx_flag && !parrot::defense_mode() {
                parrot::activate_defense_mode();
                parrot::set_attacker_frame(frame);
                cortex_m::peripheral::SCB::set_pendsv();
                return;
            }
            // else: it was our packet or defense mode already on
            if self.tx_flag && parrot::is_d_message(frame) {
                parrot::notify_d_message_received();
            }
        }
    }

    pub fn tx_error_handler(&mut self) {
        if parrot::defense_mode() { 
            parrot::collision_detected();
        }
        // lower TX error IRQ
        unsafe {
            core::ptr::write_volatile(CAN_BASE.offset(6), 1); 
        }
    }
}

pub fn init(
    my_ids: &'static [u16],
    interested_ids: &'static [u16],
    listener: *mut dyn FrameListener,
) {
    unsafe {
        COMMUNICATION_COMPONENT = Some(CommunicationComponent::new(my_ids, interested_ids, listener));
    }
}