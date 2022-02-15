use enet_sys::{_ENetPacket, enet_packet_destroy};

pub struct ENetPacket {
    _inner_packet: *mut _ENetPacket,
}

impl ENetPacket {
    pub fn new(packet: *mut _ENetPacket) -> Self {
        Self {
            _inner_packet: packet,
        }
    }

    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(
                (*self._inner_packet).data,
                (*self._inner_packet).dataLength as usize,
            )
        }
    }

    // pub fn get_data<'a>(&'a self) -> Vec<u8> {
    //     unsafe { std::slice::from_raw_parts((*self._inner_packet).data, (*self._inner_packet).dataLength as usize).to_vec() }
    // }
}

impl Drop for ENetPacket {
    fn drop(&mut self) {
        unsafe {
            enet_packet_destroy(self._inner_packet);
        }
    }
}
