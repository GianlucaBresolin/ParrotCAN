#![no_std]
#![no_main]

mod parse;
mod app_listener;

use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m::interrupt::InterruptNumber;
use cortex_m::peripheral::{NVIC};

use inter_component_exchange_manager::INTER_COMPONENT_EXCHANGE_MANAGER;
use app_listener::AppListener;

static mut APP_LISTENER: AppListener = AppListener;

#[derive(Clone, Copy)]
struct CanInterrupt(u8);

unsafe impl InterruptNumber for CanInterrupt {
    fn number(self) -> u16 {
        self.0 as u16
    }
}

#[entry]
fn main() -> !{
    let my_ids = parse::my_ids();
    let interested_ids = parse::interested_ids();
    let role = parse::get_role();
    let mut tx_count = 0;

    // sets priorities and interrupts
    unsafe {
        let mut cp = cortex_m::Peripherals::steal();
        
        cp.SCB.set_priority(cortex_m::peripheral::scb::SystemHandler::PendSV, 0xFF);
        cp.NVIC.set_priority(CanInterrupt(40), 0x00);
        cp.NVIC.set_priority(CanInterrupt(41), 0x00);

        NVIC::unmask(CanInterrupt(40));
        NVIC::unmask(CanInterrupt(41));

        cortex_m::interrupt::enable();
    }   

    
    // passive device application logic
    let app_frame_listener = (&raw mut APP_LISTENER) as *mut dyn inter_component_exchange_manager::app_listener::AppListener;

    inter_component_exchange_manager::init(
        my_ids, 
        interested_ids, 
        role, 
        app_frame_listener
    );

    loop{    
        // active device application logic
        if role == "attacker" && tx_count < 2 {
            if let Some(inter_component_exchange_manager) = unsafe { &mut *core::ptr::addr_of_mut!(INTER_COMPONENT_EXCHANGE_MANAGER) }.as_mut() {
                // default data
                let dlc: u8 = 8;
                let data: [u8; 8] = [0xFF; 8];
                inter_component_exchange_manager.send(my_ids[0], dlc, data);
                tx_count += 1;
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    match irqn {
        40 => {
                // Received CAN Frame Interrupt
                unsafe {
                    if let Some(ref mut inter_component_exchange_manager) = INTER_COMPONENT_EXCHANGE_MANAGER {
                        inter_component_exchange_manager.receive();
                    }
                }
            },
        41 => 
            {
                // Received TX Error Interrupt
                unsafe {
                    if let Some(ref mut inter_component_exchange_manager) = INTER_COMPONENT_EXCHANGE_MANAGER {
                        inter_component_exchange_manager.tx_error_handler();
                    }
                }
            }
        _ => {},
    }
}