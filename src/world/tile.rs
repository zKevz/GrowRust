use std::{cell::RefCell, rc::Rc, time::Instant};

use byteorder::{WriteBytesExt, LE};

use crate::{
    consts::{self, items, tile_flags},
    item::{iteminfo::ItemInfo, iteminfo_manager::ITEM_MANAGER},
    player::player::Player,
    utils::{error::Result, math::Vec2u},
};

use super::{tileextra::TileExtra, world::World};

pub struct Tile {
    pub pos: Vec2u,
    pub fore: u16,
    pub back: u16,
    pub flags: u16,
    pub extra: TileExtra,
    pub world: Rc<RefCell<World>>,
    pub parent: u16, // lock parent index
    pub hit_count: u8,
    pub last_punch: Instant,
}

impl Tile {
    pub fn new(world: Rc<RefCell<World>>, x: u32, y: u32) -> Self {
        Self {
            world,

            pos: { Vec2u { x, y } },
            fore: 0,
            back: 0,
            flags: 0,
            extra: TileExtra::None,
            parent: 0,
            hit_count: 0,
            last_punch: Instant::now(),
        }
    }

    pub fn set_fore(&mut self, fore: u16) {
        self.fore = fore;

        let item = ITEM_MANAGER.get_item(fore as u32);
        if item.extra {
            self.flags |= consts::tile_flags::EXTRA_DATA;
        }
    }

    pub fn set_back(&mut self, back: u16) {
        self.back = back;
    }

    pub fn get_fore(&self) -> Result<&'static ItemInfo> {
        ITEM_MANAGER.get_item_safe(self.fore as u32)
    }

    pub fn get_back(&self) -> Result<&'static ItemInfo> {
        ITEM_MANAGER.get_item_safe(self.back as u32)
    }

    pub fn get_base(&self) -> Result<&'static ItemInfo> {
        if self.fore != items::BLANK {
            self.get_fore()
        } else {
            self.get_back()
        }
    }

    pub fn add_flag(&mut self, flags: u16) {
        self.flags |= flags;
    }

    pub fn remove_flag(&mut self, flags: u16) {
        self.flags &= !flags;
    }

    pub fn has_flag(&self, flags: u16) -> bool {
        (self.flags & flags) != 0
    }

    pub fn remove_base(&mut self) {
        if self.fore != items::BLANK {
            self.fore = items::BLANK;
        } else {
            self.back = items::BLANK;
        }

        self.hit_count = 0;
        self.last_punch = Instant::now();
        self.remove_flag(tile_flags::EXTRA_DATA);
        self.remove_flag(tile_flags::SEED);
        self.remove_flag(tile_flags::FLIPPED);
        self.remove_flag(tile_flags::OPEN);
        self.remove_flag(tile_flags::PUBLIC);
        self.remove_flag(tile_flags::SILENCED);
        self.remove_flag(tile_flags::RED);
        self.remove_flag(tile_flags::GREEN);
        self.remove_flag(tile_flags::BLUE);
    }

    pub fn serialize(&mut self, data: &mut Vec<u8>, _player: &Option<&mut Player>) -> Result<()> {
        data.write_u16::<LE>(self.fore)?;
        data.write_u16::<LE>(self.back)?;
        data.write_u16::<LE>(self.parent)?;
        data.write_u16::<LE>(self.flags)?;

        if self.has_flag(tile_flags::LOCKED) {
            data.write_u16::<LE>(self.parent)?;
        }

        if self.has_flag(tile_flags::EXTRA_DATA) {
            self.extra.serialize(data)?;
        }

        Ok(())
    }
}
