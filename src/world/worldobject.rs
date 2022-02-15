use crate::utils::math::Vec2f;

pub struct WorldObject {
    pub pos: Vec2f,
    pub count: u8,
    pub flags: u8,
    pub item_id: u16,
}
