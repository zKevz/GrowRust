use enet_sys::enet_initialize;

pub mod event;
pub mod host;
pub mod packet;
pub mod peer;

pub fn initialize() {
    unsafe {
        enet_initialize();
    }
}
