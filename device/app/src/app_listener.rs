use inter_component_exchange_manager::app_listener::AppListener as AppListenerTrait;
use inter_component_exchange_manager::INTER_COMPONENT_EXCHANGE_MANAGER;

pub struct AppListener;

impl AppListenerTrait for AppListener {
    fn on_receive(&mut self, _payload: &[u8]) {
        if let Some(inter_component_exchange_manager) = unsafe { &mut *core::ptr::addr_of_mut!(INTER_COMPONENT_EXCHANGE_MANAGER) }.as_mut() {
            inter_component_exchange_manager.turn_on_led();
        }
    }
}
