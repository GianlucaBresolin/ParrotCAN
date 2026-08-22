use communication_component::frame_listener::FrameListener as FrameListenerTrait;
use local_device_manager::LOCAL_DEVICE_MANAGER;

pub struct AppFrameListener;

impl FrameListenerTrait for AppFrameListener {
    fn on_interesting_frame(&mut self, _payload: &[u8]) {
        if let Some(local_device_manager) = unsafe { &mut *core::ptr::addr_of_mut!(LOCAL_DEVICE_MANAGER) }.as_mut() {
            local_device_manager.turn_on_light();
        }
    }
}
