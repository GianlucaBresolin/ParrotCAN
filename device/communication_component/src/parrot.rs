use cortex_m_semihosting::hprintln;

use crate::CANFrame;
use crate::COMMUNICATION_COMPONENT;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use core::sync::atomic::{AtomicU8, AtomicBool, Ordering};
use cortex_m_rt::exception;

const ND: u8 = 100;

pub static PENDING_ENGAGE_FRAME: Mutex<RefCell<Option<CANFrame>>> = Mutex::new(RefCell::new(None));
pub static SUSPECT_FOUND: AtomicBool = AtomicBool::new(false);
pub static COLLISION_COUNT: AtomicU8 = AtomicU8::new(0);
pub static COLLISION_DETECTED: AtomicBool = AtomicBool::new(false);
pub static TX_D_SIGNAL: AtomicBool = AtomicBool::new(false);
pub static D_MESSAGE: Mutex<RefCell<Option<CANFrame>>> = Mutex::new(RefCell::new(None));

#[exception]
fn PendSV() {
    let frame = cortex_m::interrupt::free(|cs| {
        PENDING_ENGAGE_FRAME.borrow(cs).replace(None)
    });
    if let Some(frame) = frame {
        engage(frame);
    }
}

fn engage(attacker_frame: CANFrame) {
    hprintln!("DEFENSE MODE ON");
    while SUSPECT_FOUND.load(Ordering::Acquire) && !COLLISION_DETECTED.load(Ordering::Acquire) {
        transmit_nd_message(attacker_frame);
    }
}

fn transmit_nd_message(attacker_frame: CANFrame) {
    let mut bound = ND;
    let mut i = 0;
    while i < bound {
        transmit_d_message(attacker_frame);

        while TX_D_SIGNAL.load(Ordering::Acquire) {
            // wait for incoming frame interrupt before checking 
            // (avoid busy wait)
            cortex_m::asm::wfi(); 
        }

        if COLLISION_DETECTED.load(Ordering::Acquire) {
            COLLISION_DETECTED.store(false, Ordering::Release);
            SUSPECT_FOUND.store(false, Ordering::Release);
            bound = i + 16;
        }
        i += 1;
    }
}

fn transmit_d_message(attacker_frame: CANFrame) {
    unsafe {
        if let Some(ref mut communication_component) = COMMUNICATION_COMPONENT {
            // defense message
            communication_component.send(
                attacker_frame.id, // same spoofed id
                attacker_frame.dlc, // same dlc    
                &[0u8; 64][..attacker_frame.dlc as usize], // data of all 0s
            );
        }
    }

    cortex_m::interrupt::free(|cs| {
        D_MESSAGE.borrow(cs).replace(Some(CANFrame {
            id: attacker_frame.id,
            rtr: attacker_frame.rtr,
            dlc: attacker_frame.dlc,
            data_low: 0,
            data_high: 0,
        }));
    });

    TX_D_SIGNAL.store(true, Ordering::Release);
}

// Public functions to update shared state within the Parrot defense algorithm
pub fn defense_mode() -> bool{
    SUSPECT_FOUND.load(Ordering::Acquire)
}

pub fn activate_defense_mode() {
    SUSPECT_FOUND.store(true, Ordering::Release);
    COLLISION_COUNT.store(0, Ordering::Release);
}

pub fn set_attacker_frame(frame: CANFrame) {
    cortex_m::interrupt::free(|cs| {
        PENDING_ENGAGE_FRAME.borrow(cs).replace(Some(frame));
    });
}

pub fn collision_detected() {
    let count = COLLISION_COUNT.fetch_add(1, Ordering::AcqRel) + 1;
    if count == 16 {
        COLLISION_DETECTED.store(true, Ordering::Release);
    }
}

pub fn is_d_message(received_frame: CANFrame) -> bool {
    cortex_m::interrupt::free(|cs| {
        match &*D_MESSAGE.borrow(cs).borrow() {
            Some(d_msg) => {
                d_msg.id == received_frame.id
                && d_msg.dlc == received_frame.dlc
                && d_msg.data_low == received_frame.data_low
                && d_msg.data_high == received_frame.data_high
            }
            None => false,
        }
    })
}

pub fn notify_d_message_received() {
    TX_D_SIGNAL.store(false, Ordering::Release);
}