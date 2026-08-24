#![no_std]
#![no_main]

mod parse;
mod frame_listener;

use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m::interrupt::InterruptNumber;
use cortex_m::peripheral::{NVIC, SCB};

use frame_listener::AppFrameListener;
use communication_component::COMMUNICATION_COMPONENT;
use simulated_led_driver::SIMULATED_LED_DRIVER;
use local_device_manager::device_drivers::DeviceDriver;
static mut APP_FRAME_LISTENER: AppFrameListener = AppFrameListener;
use cortex_m_semihosting::hprintln;

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
    let mut sended = false;

    communication_component::init(
        my_ids,
        interested_ids, 
        (&raw mut APP_FRAME_LISTENER) as *mut _ as *mut dyn communication_component::frame_listener::FrameListener,
    );
    
    let led_driver: *mut dyn DeviceDriver = (&raw mut SIMULATED_LED_DRIVER) as *mut _ as *mut dyn DeviceDriver;
    local_device_manager::init(led_driver);

    unsafe {
        let mut cp = cortex_m::Peripherals::steal();
        
        cp.SCB.set_priority(cortex_m::peripheral::scb::SystemHandler::PendSV, 0xFF);
        cp.NVIC.set_priority(CanInterrupt(40), 0x00);
        cp.NVIC.set_priority(CanInterrupt(41), 0x00);

        NVIC::unmask(CanInterrupt(40));
        NVIC::unmask(CanInterrupt(41));

        cortex_m::interrupt::enable();
    }   

    loop{    
        if !sended{
            if let Some(communication_component) = unsafe { &mut *core::ptr::addr_of_mut!(COMMUNICATION_COMPONENT) }.as_mut() {
                // default data
                sended = true;

                let dlc: u8 = 8;
                if my_ids.get(0) == Some(&0x101) && role == "attacker" {
                    let data: [u8; 8] = [0xFF; 8];
                    communication_component.send(my_ids[0], dlc, &data);

                    communication_component.send(my_ids[0], dlc, &data);
                }
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
                hprintln!("\n INTERRUPT FOR RX \n");
                unsafe {
                    if let Some(ref mut communication_component) = COMMUNICATION_COMPONENT {
                        communication_component.receive();
                    }
                }
            },
        41 => 
            {
                // Received TX Error Interrupt
                unsafe {
                    if let Some(ref mut communication_component) = COMMUNICATION_COMPONENT {
                        communication_component.tx_error_handler();
                    }
                }
            }
        _ => {},
    }
}