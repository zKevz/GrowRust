use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct InventoryItem {
    pub count: u8,
    pub flags: u8,
}
