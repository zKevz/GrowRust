use std::{cell::RefMut, time::Instant};

use super::event::EventContext;
use crate::{
    consts::{item_type, items, packet::TankUpdatePacket, packet_type},
    item::iteminfo_manager::ITEM_MANAGER,
    utils::{
        error::{Error, Result},
        math::Vec2f,
        variant_function::VariantFunction::*,
    },
    world::world::World,
};

const RETURN_STATE_NONE: u32 = 0;
const RETURN_STATE_REMOVE_ITEM: u32 = 1 << 0;
const RETURN_STATE_SEND_MODIFY_ITEM_VISUAL: u32 = 1 << 1;

pub fn handle(mut ctx: EventContext) -> Result<()> {
    if let Ok(world) = ctx.player.get_world(ctx.world_manager) {
        let mut world = world.borrow_mut();

        let mut tankpacket = TankUpdatePacket::from(&ctx.packet_data[4..])?;
        tankpacket.net_id = ctx.player.net_id;

        match tankpacket.packet_type {
            packet_type::STATE => {
                ctx.player.pos = Vec2f {
                    x: tankpacket.pos_x,
                    y: tankpacket.pos_y,
                };
                ctx.player.char_flags = tankpacket.flags;

                if world.peers.len() > 1 {
                    world.push_tankpacket(tankpacket);
                    world.send_all();
                }
            }

            packet_type::TILE_CHANGE_REQUEST => {
                let item = ITEM_MANAGER.get_item_safe(tankpacket.int_val as u32)?;
                let result: Result<()>;
                match item.id {
                    items::FIST => result = on_tile_punch_request(&mut ctx, tankpacket, &mut world),
                    items::WRENCH => {
                        result = on_tile_wrench_request(&mut ctx, tankpacket, &mut world)
                    }
                    _ => match on_tile_build_request(&mut ctx, tankpacket, &mut world) {
                        Ok(state) => {
                            result = Ok(());

                            if state & RETURN_STATE_REMOVE_ITEM != 0 {
                                ctx.player.remove_item(
                                    item.id,
                                    1,
                                    state & RETURN_STATE_SEND_MODIFY_ITEM_VISUAL != 0,
                                )?;
                            }
                        }
                        Err(e) => result = Err(e),
                    },
                }

                world.send_all();

                return result;
            }

            packet_type::ITEM_ACTIVATE_REQUEST => {
                let item = ITEM_MANAGER.get_item_safe(tankpacket.int_val as u32)?;

                if ctx.player.has_item(item.id) {
                    match item.item_type {
                        item_type::ANCES | item_type::CLOTHES => {
                            ctx.player.equip(item.id);

                            world.push_varfn_v(
                                OnSetClothing(ctx.player.cloth, true),
                                ctx.player.net_id,
                                -1,
                            );
                            world.send_all();
                        }
                        _ => {}
                    }
                } else {
                    return Err(Error::InvalidPacketError);
                }
            }

            packet_type::TILE_ACTIVATE_REQUEST => {
                let tile =
                    world.get_tile_safe(tankpacket.tile_x as u32, tankpacket.tile_y as u32)?;
                let fore = ITEM_MANAGER.get_item_safe(tile.fore as u32)?;

                match fore.item_type {
                    item_type::MAIN_DOOR => {
                        ctx.world_manager.exit_world(ctx.player, world);
                        ctx.player.send_world_menu(ctx.host);
                    }

                    _ => {}
                }
            }

            packet_type::SET_ICON_STATE => {
                world.push_tankpacket(tankpacket);
                world.send_all();
            }

            _ => println!("Unhandled packet type: {}", tankpacket.packet_type),
        }
    }

    Ok(())
}

fn on_tile_punch_request(
    _: &mut EventContext,
    mut tankpacket: TankUpdatePacket,
    world: &mut RefMut<World>,
) -> Result<()> {
    let tile = world.get_tile_safe(tankpacket.tile_x as u32, tankpacket.tile_y as u32)?;
    let base = tile.get_base()?;
    if base.id == items::BLANK {
        return Ok(());
    }

    if (Instant::now() - tile.last_punch).as_secs() > base.heal_time as u64 {
        tile.hit_count = 0;
        tile.last_punch = Instant::now();
    }

    tile.hit_count += 1;

    tankpacket.packet_type = packet_type::TILE_APPLY_DAMAGE;
    tankpacket.int_val = 6; // tile damage

    if tile.hit_count >= base.hits_to_destroy {
        tankpacket.packet_type = packet_type::TILE_CHANGE_REQUEST;
        tankpacket.int_val = items::FIST as i32;
        tile.remove_base();
    }

    world.push_tankpacket(tankpacket);

    Ok(())
}

fn on_tile_wrench_request(
    _: &mut EventContext,
    _: TankUpdatePacket,
    _: &mut RefMut<World>,
) -> Result<()> {
    Ok(())
}

fn on_tile_build_request(
    ctx: &mut EventContext,
    tankpacket: TankUpdatePacket,
    world: &mut RefMut<World>,
) -> Result<u32> {
    let tile = world.get_tile_safe(tankpacket.tile_x as u32, tankpacket.tile_y as u32)?;
    let fore = tile.get_fore()?;
    let base = tile.get_base()?;
    let item = ITEM_MANAGER.get_item_safe(tankpacket.int_val as u32)?;

    if !ctx.player.has_item(item.id) {
        return Err(Error::InvalidPacketError);
    }

    if base.id != items::BLANK {}

    if item.item_type == item_type::BACKGROUND
        || item.item_type == item_type::BACKGD_SFX_EXTRA_FRAME
        || item.item_type == item_type::MUSIC_NOTE
    {
        tile.set_back(item.id);
    } else {
        if fore.id != items::BLANK {
            return Ok(RETURN_STATE_NONE);
        }

        match item.item_type {
            _ => {
                // there is guild item and flag in flags2 but i dont wanna share it :PP
                if item.extra {
                    ctx.player.send_log("`4Oops!`` This item is not handled yet. Please report to one of the developers.");
                    return Ok(RETURN_STATE_NONE);
                }
            }
        }

        tile.set_fore(item.id);
    }

    world.push_tankpacket(tankpacket);

    Ok(RETURN_STATE_REMOVE_ITEM)
}
