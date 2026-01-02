#![no_std]
#![no_main]

use cortex_m_rt::entry;
use local_device_manager::LocalDeviceManager;
use simulated_led_driver::SimulatedLedDriver;
use communication_component::CommunicationComponent;

#[entry]
fn main() -> !{
    let mut local_device_manager = LocalDeviceManager::new(SimulatedLedDriver);
    let communication_component = CommunicationComponent::new();

    local_device_manager.turn_on_light();
    communication_component.send();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}