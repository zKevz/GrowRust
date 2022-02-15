use std::{mem::MaybeUninit, net::Ipv4Addr};

use enet_sys::{_ENetAddress, _ENetHost, enet_host_create, enet_host_destroy, enet_host_service};

use super::event::ENetEventType;

pub struct ENetHost {
    _inner_host: *mut _ENetHost,
}

impl ENetHost {
    pub fn new(ip: Ipv4Addr, port: u16, peer_count: usize) -> Self {
        let address = _ENetAddress {
            host: u32::from_be_bytes(ip.octets()).to_be(),
            port,
        };

        let host = unsafe { enet_host_create(&address as *const _, peer_count, 1, 0, 0) };
        unsafe {
            (*host).checksum = Some(enet_sys::enet_crc32);
            enet_sys::enet_host_compress_with_range_coder(host);
        }

        Self { _inner_host: host }
    }

    pub fn service(&mut self, timeout: u32) -> Option<ENetEventType> {
        let mut event = MaybeUninit::uninit();
        let res = unsafe { enet_host_service(self._inner_host, event.as_mut_ptr(), timeout) };

        if res > 0 {
            ENetEventType::new(unsafe { &event.assume_init() })
        } else {
            None
        }
    }

    pub fn online_count(&self) -> usize {
        unsafe { (*self._inner_host).connectedPeers }
    }
}

impl Drop for ENetHost {
    /// Call the corresponding ENet cleanup-function(s).
    fn drop(&mut self) {
        unsafe {
            enet_host_destroy(self._inner_host);
        }
    }
}
