use enet_sys::_ENetPeer;

pub struct ENetPeer {
    pub inner_peer: *mut _ENetPeer,
}

impl ENetPeer {
    pub fn new(peer: *mut _ENetPeer) -> Self {
        Self { inner_peer: peer }
    }

    pub fn get_inner(&mut self) -> *mut _ENetPeer {
        self.inner_peer
    }

    pub fn get_data<T>(&mut self) -> Option<&mut T> {
        unsafe {
            let raw_data = (*self.inner_peer).data as *mut T;

            if raw_data.is_null() {
                None
            } else {
                Some(&mut (*raw_data))
            }
        }
    }

    pub fn set_data<T>(&mut self, data: Option<T>) {
        unsafe {
            let raw_data = (*self.inner_peer).data as *mut T;

            if !raw_data.is_null() {
                // free old data
                let _: Box<T> = Box::from_raw(raw_data);
            }

            let new_data = match data {
                Some(data) => Box::into_raw(Box::new(data)) as *mut _,
                None => std::ptr::null_mut(),
            };

            (*self.inner_peer).data = new_data;
        }
    }
}
