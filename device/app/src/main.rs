#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_rt::exception;

use local_device_manager::LocalDeviceManager;
use simulated_led_driver::SimulatedLedDriver;
use communication_component::CommunicationComponent;

mod parse;

static mut COMMUNICATION_COMPONENT: Option<CommunicationComponent> = None;

#[entry]
fn main() -> !{
    let my_ids = parse::my_ids();
    let interested_ids = parse::interested_ids();

    let mut local_device_manager = LocalDeviceManager::new(SimulatedLedDriver);
    unsafe {
        COMMUNICATION_COMPONENT = Some(CommunicationComponent::new(
            my_ids, 
            my_ids.len(), 
            interested_ids,
            interested_ids.len()
       ));
    }

    unsafe {
        NVIC::unmask(cortex_m::interrupt::Nr::from(40u8));
        cortex_m::interrupt::enable();
    }

    local_device_manager.turn_on_light();
    communication_component.send();
    loop {
        wfi(); // Wait for interrupt
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[exception]
fn DefaultHandler(irqn: i16) {
    if irqn == 40 {
        // Received CAN Frame Interrupt
        unsafe {
            if let Some(ref mut communication_component) = COMMUNICATION_COMPONENT {
                communication_component.receive();
            }
        }
    }
}