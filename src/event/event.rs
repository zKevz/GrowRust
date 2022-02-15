use byteorder::{ReadBytesExt, LE};

use crate::{
    consts::{self, config, message_type},
    enet_wrapper::host::ENetHost,
    player::player::Player,
    utils::error::{Error, Result},
    world::world_manager::WorldManager,
};

use super::{tank, text};

pub struct EventContext<'a> {
    pub host: &'a ENetHost,
    pub player: &'a mut Player,
    pub world_manager: &'a mut WorldManager,

    pub text_data: &'a str,
    pub packet_data: &'a [u8],
}

pub fn handle(mut ctx: EventContext) -> Result<()> {
    let mut packet_data = ctx.packet_data;
    if packet_data.len() <= config::MINIMUM_PACKET_SIZE
        || packet_data.len() >= config::MAXIMUM_PACKET_SIZE
    {
        return Err(Error::InvalidPacketError);
    }

    let packet_type = packet_data.read_u32::<LE>()?;
    match packet_type {
        message_type::GENERIC_TEXT | message_type::GAME_MESSAGE => {
            ctx.text_data = std::str::from_utf8(&packet_data[..packet_data.len() - 1])?;
            text::handle(ctx)?;
        }

        consts::message_type::GAME_PACKET => {
            tank::handle(ctx)?;
        }

        _ => {}
    }

    Ok(())
}
