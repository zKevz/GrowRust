use std::io::Write;

use byteorder::{WriteBytesExt, LE};

use super::math::{Vec2f, Vec3f};

#[derive(Debug, Copy, Clone)]
pub enum Variant<'a> {
    Float(f32),
    Int32(i32),
    UInt32(u32),
    String(&'a str),
    Vector2(f32, f32),
    Vector3(f32, f32, f32),
}

impl Variant<'_> {
    pub fn get_type(&self) -> u8 {
        match self {
            Variant::Float(_) => 1,
            Variant::Int32(_) => 9,
            Variant::UInt32(_) => 5,
            Variant::String(_) => 2,
            Variant::Vector2(_, _) => 3,
            Variant::Vector3(_, _, _) => 4,
        }
    }

    pub fn serialize(&self, vec: &mut Vec<u8>) -> std::io::Result<()> {
        vec.write_u8(self.get_type())?;

        match *self {
            Variant::Float(value) => vec.write_f32::<LE>(value)?,
            Variant::Int32(value) => vec.write_i32::<LE>(value)?,
            Variant::UInt32(value) => vec.write_u32::<LE>(value)?,
            Variant::String(value) => {
                vec.write_i32::<LE>(value.len() as i32)?;
                vec.write(value.as_bytes())?;
            }
            Variant::Vector2(x, y) => {
                vec.write_f32::<LE>(x)?;
                vec.write_f32::<LE>(y)?;
            }
            Variant::Vector3(x, y, z) => {
                vec.write_f32::<LE>(x)?;
                vec.write_f32::<LE>(y)?;
                vec.write_f32::<LE>(z)?;
            }
        }

        Ok(())
    }
}

impl From<f32> for Variant<'_> {
    fn from(value: f32) -> Self {
        Variant::Float(value)
    }
}

impl From<i32> for Variant<'_> {
    fn from(value: i32) -> Self {
        Variant::Int32(value)
    }
}

impl From<u32> for Variant<'_> {
    fn from(value: u32) -> Self {
        Variant::UInt32(value)
    }
}

impl<'a> From<&'a str> for Variant<'a> {
    fn from(value: &'a str) -> Self {
        Variant::String(value)
    }
}

impl<'a> From<&'a String> for Variant<'a> {
    fn from(value: &'a String) -> Self {
        Variant::String(&value)
    }
}

// impl From<&'static str> for Variant<'static> {
//     fn from(value: &'static str) -> Self {
//         Variant::String(value)
//     }
// }

impl From<Vec2f> for Variant<'_> {
    fn from(value: Vec2f) -> Self {
        Variant::Vector2(value.x, value.y)
    }
}

impl From<Vec3f> for Variant<'_> {
    fn from(value: Vec3f) -> Self {
        Variant::Vector3(value.x, value.y, value.z)
    }
}

impl From<&[f32; 2]> for Variant<'_> {
    fn from(value: &[f32; 2]) -> Self {
        Variant::Vector2(value[0], value[1])
    }
}

impl From<&[f32; 3]> for Variant<'_> {
    fn from(value: &[f32; 3]) -> Self {
        Variant::Vector3(value[0], value[1], value[2])
    }
}
