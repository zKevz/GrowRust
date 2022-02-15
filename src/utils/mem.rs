use super::error::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};

const ENCRYPT_KEY: &'static str = "PBG892FXX982ABC*";

pub fn write_string<T>(stream: &mut T, str: &str) -> Result<()>
where
    T: std::io::Write,
{
    stream.write_u16::<LE>(str.len() as u16)?;
    stream.write_all(str.as_bytes())?;

    Ok(())
}

pub fn read_string<T>(stream: &mut T) -> Result<String>
where
    T: std::io::Read,
{
    let size = stream.read_u16::<LE>()?;
    let mut string = String::with_capacity(size as usize);

    for _ in 0..size {
        string.push(stream.read_u8()? as char);
    }

    Ok(string)
}

pub fn read_string_long<T>(stream: &mut T) -> Result<String>
where
    T: std::io::Read,
{
    let size = stream.read_u32::<LE>()?;
    let mut string = String::with_capacity(size as usize);

    for _ in 0..size {
        string.push(stream.read_u8()? as char);
    }

    Ok(string)
}

pub fn read_string_encrypted<T>(stream: &mut T, id: u16) -> Result<String>
where
    T: std::io::Read,
{
    let size = stream.read_u16::<LE>()?;
    let mut string = String::with_capacity(size as usize);

    for i in 0..size {
        let val = stream.read_u8()?;
        string.push(
            (val ^ ENCRYPT_KEY.as_bytes()[(i as usize + id as usize) % ENCRYPT_KEY.len()]) as char,
        );
    }

    Ok(string)
}
