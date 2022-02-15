use byteorder::WriteBytesExt;

use crate::{
    consts::tileextra_type,
    utils::{error::Result, mem},
};

#[derive(Debug)]
pub enum TileExtra {
    None,
    Door { label: String },
}

impl TileExtra {
    pub fn serialize<T>(&self, data: &mut T) -> Result<()>
    where
        T: std::io::Write,
    {
        match self {
            TileExtra::None => panic!("Tried to serialize None Tileextra!"),

            TileExtra::Door { label } => {
                data.write_u8(tileextra_type::DOOR)?;
                mem::write_string(data, &label)?;
                data.write_u8(0)?;
            }
        }

        Ok(())
    }
}
