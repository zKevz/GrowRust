use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::{
    io::{self, Write},
    mem,
};

use crate::consts;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TankUpdatePacket {
    pub packet_type: u8,
    pub field1: u8,
    pub field2: u8,
    pub field3: u8,
    pub net_id: i32,
    pub target_id: i32,
    pub flags: u32,
    pub float_val: f32,
    pub int_val: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_x2: f32,
    pub pos_y2: f32,
    pub float_val2: f32,
    pub tile_x: i32,
    pub tile_y: i32,
    pub extra_data_size: u32,
    pub extra_data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl TankUpdatePacket {
    pub fn from(mut data: &[u8]) -> Result<Self, io::Error> {
        Ok(Self {
            packet_type: data.read_u8()?,
            field1: data.read_u8()?,
            field2: data.read_u8()?,
            field3: data.read_u8()?,
            net_id: data.read_i32::<LE>()?,
            target_id: data.read_i32::<LE>()?,
            flags: data.read_u32::<LE>()?,
            float_val: data.read_f32::<LE>()?,
            int_val: data.read_i32::<LE>()?,
            pos_x: data.read_f32::<LE>()?,
            pos_y: data.read_f32::<LE>()?,
            pos_x2: data.read_f32::<LE>()?,
            pos_y2: data.read_f32::<LE>()?,
            float_val2: data.read_f32::<LE>()?,
            tile_x: data.read_i32::<LE>()?,
            tile_y: data.read_i32::<LE>()?,
            extra_data_size: data.read_u32::<LE>()?,
            extra_data: None,
        })
    }

    pub fn default() -> Self {
        let mut packet: Self = unsafe { mem::zeroed() };
        packet.extra_data = None;
        packet
    }

    pub fn with_extra_data(data: Vec<u8>) -> Self {
        let mut packet = TankUpdatePacket::default();
        packet.flags |= consts::packet_flags::EXTENDED;
        packet.extra_data_size = data.len() as u32;
        packet.extra_data = Some(data);
        packet
    }

    pub fn serialize(&self) -> Result<Vec<u8>, io::Error> {
        let mut vec: Vec<u8> = Vec::with_capacity(60);

        vec.write_u8(self.packet_type.into())?;
        vec.write_u8(self.field1)?;
        vec.write_u8(self.field2)?;
        vec.write_u8(self.field3)?;
        vec.write_i32::<LE>(self.net_id)?;
        vec.write_i32::<LE>(self.target_id)?;
        vec.write_u32::<LE>(self.flags)?;
        vec.write_f32::<LE>(self.float_val)?;
        vec.write_i32::<LE>(self.int_val)?;
        vec.write_f32::<LE>(self.pos_x)?;
        vec.write_f32::<LE>(self.pos_y)?;
        vec.write_f32::<LE>(self.pos_x2)?;
        vec.write_f32::<LE>(self.pos_y2)?;
        vec.write_f32::<LE>(self.float_val2)?;
        vec.write_i32::<LE>(self.tile_x)?;
        vec.write_i32::<LE>(self.tile_y)?;
        vec.write_u32::<LE>(self.extra_data_size)?;

        if let Some(extra_data) = &self.extra_data {
            vec.write(&extra_data)?;
        }

        vec.write_u8(0)?; // null terminator

        Ok(vec)
    }
}
