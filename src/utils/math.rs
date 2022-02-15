#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;
pub type Vec2f = Vec2<f32>;

pub type Vec3i = Vec3<i32>;
pub type Vec3u = Vec3<u32>;
pub type Vec3f = Vec3<f32>;
