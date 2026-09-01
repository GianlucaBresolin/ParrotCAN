#![no_std]

pub mod app_listener;

use communication_component::COMMUNICATION_COMPONENT;
use communication_component::frame_listener::FrameListener;
use local_device_manager::LOCAL_DEVICE_MANAGER;

use crate::app_listener::AppListener;

pub static mut INTER_COMPONENT_EXCHANGE_MANAGER: Option<InterComponentExchangeManager> = None;

pub struct InterComponentExchangeManager {
    app_listener: *mut dyn AppListener, 
}

impl InterComponentExchangeManager {
    pub fn new(
        app_listener: *mut dyn AppListener
    ) -> Self {
        let inter_comp_exchange_manager = Self {
            app_listener
        };

        inter_comp_exchange_manager
    }

    pub fn send(
        &self,
        id: u16,
        dlc: u8, 
        data: [u8; 8] 
    ) {
        if let Some(communication_component) = unsafe { &mut *core::ptr::addr_of_mut!(COMMUNICATION_COMPONENT) }.as_mut() {    
            communication_component.send(id, dlc, &data);
        }
    }

    pub fn receive(&self) {
        unsafe {
            if let Some(ref mut communication_component) = COMMUNICATION_COMPONENT {
                communication_component.receive();
            }
        }
    }

    pub fn tx_error_handler(&self) {
        unsafe {
            if let Some(ref mut communication_component) = COMMUNICATION_COMPONENT {
                communication_component.tx_error_handler();
            }
        }
    }

    pub fn turn_on_led(&self) {
        if let Some(local_device_manager) = unsafe { &mut *core::ptr::addr_of_mut!(LOCAL_DEVICE_MANAGER) }.as_mut() {
            local_device_manager.turn_on_light();
        }    
    }
}

impl FrameListener for InterComponentExchangeManager {
    fn on_interesting_frame(&mut self, payload: &[u8]) {
        unsafe {
            (&mut *self.app_listener).on_receive(payload);
        }
    }
}

pub fn init(
    my_ids: &'static [u16],
    interested_ids: &'static [u16],
    role: &'static str,
    listener: *mut dyn AppListener
) {
    unsafe {
        INTER_COMPONENT_EXCHANGE_MANAGER = Some(InterComponentExchangeManager::new(listener));
    }

    let inter_component_exchange_manager_ptr: *mut InterComponentExchangeManager = unsafe {
        let opt_ptr = core::ptr::addr_of_mut!(INTER_COMPONENT_EXCHANGE_MANAGER);
        (*opt_ptr).as_mut().unwrap() as *mut _
    };
    let frame_listener: *mut dyn FrameListener = inter_component_exchange_manager_ptr as *mut dyn FrameListener;


    communication_component::init(my_ids, interested_ids, role, frame_listener);

    local_device_manager::init();
}