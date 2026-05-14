#![no_std]
#![no_main]

use cortex_m_rt::entry;
use local_device_manager::LocalDeviceManager;
use simulated_led_driver::SimulatedLedDriver;
use communication_component::CommunicationComponent;

mod parse;

#[entry]
fn main() -> !{
    let my_ids = parse::my_ids();
    let interested_ids = parse::interested_ids();

    let mut local_device_manager = LocalDeviceManager::new(SimulatedLedDriver);
    let communication_component = CommunicationComponent::new(
        my_ids, 
        my_ids.len(), 
        interested_ids,
        interested_ids.len()
    );

    unsafe {
        NVIC::unmask(Interrupt::CAN_CONTROLLER);
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

#[interrupt]
fn CAN_interrupt_handler() {
    // to do
}