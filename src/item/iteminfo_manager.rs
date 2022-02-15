use std::{fs::File, io::Read, time::Instant};

use byteorder::{ReadBytesExt, LE};
use lazy_static::lazy_static;

use super::iteminfo::ItemInfo;
use crate::{
    consts::{item_type, packet::TankUpdatePacket},
    utils::{
        self,
        error::{Error, Result},
        mem,
    },
};

lazy_static! {
    pub static ref ITEM_MANAGER: ItemInfoManager =
        ItemInfoManager::load_from_file("data/items.dat").unwrap();
}
pub struct ItemInfoManager {
    pub hash: u32,
    pub items: Vec<ItemInfo>,
    pub packet: TankUpdatePacket,
}

impl ItemInfoManager {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let start = Instant::now();

        let mut file = File::open(path)?;
        let mut buffer = Vec::<u8>::new();

        file.read_to_end(&mut buffer)?;

        let mut data = buffer.as_slice();

        let _version = data.read_u16::<LE>()?;
        let items_count = data.read_u32::<LE>()?;

        let mut mgr = Self {
            hash: utils::hash(&buffer),
            items: Vec::with_capacity(items_count as usize),
            packet: TankUpdatePacket::default(),
        };

        for i in 0..items_count {
            let mut item = ItemInfo::default();
            item.id = data.read_u32::<LE>()? as u16;
            item.flags = data.read_u16::<LE>()?;
            item.item_type = data.read_u8()?;
            item.material = data.read_u8()?;

            item.name = mem::read_string_encrypted(&mut data, item.id)?;
            item.texture_path = mem::read_string(&mut data)?;
            item.texture_hash = data.read_u32::<LE>()?;

            item.visual_effect = data.read_u8()?;
            item.unk_0u32 = data.read_u32::<LE>()?;
            item.posx = data.read_u8()?;
            item.posy = data.read_u8()?;
            item.storage = data.read_u8()?;
            item.layer = data.read_u8()?;
            item.collision = data.read_u8()?;

            item.hits_to_destroy = data.read_u8()?;
            item.hits_to_destroy /= 6;
            item.heal_time = data.read_u32::<LE>()?;
            item.bodypart = data.read_u8()?;
            item.rarity = data.read_u16::<LE>()?;
            item.maxcount = data.read_u8()?;
            item.extra_file_path = mem::read_string(&mut data)?;
            item.extra_file_hash = data.read_u32::<LE>()?;

            item.ams = data.read_u32::<LE>()?;
            item.lid = item.ams;
            item.wid = item.lid;

            item.bname = mem::read_string(&mut data)?;
            item.bprefix = mem::read_string(&mut data)?;
            item.bsuffix = mem::read_string(&mut data)?;
            item.babil = mem::read_string(&mut data)?;

            item.seed_base = data.read_u8()?;
            item.seed_overlay = data.read_u8()?;
            item.tree_base = data.read_u8()?;
            item.tree_leaves = data.read_u8()?;
            item.seed_color = data.read_u32::<LE>()?;
            item.seed_overlay_color = data.read_u32::<LE>()?;

            data.read_exact(&mut item.unk0)?;

            item.tree_time = data.read_u32::<LE>()?;
            item.unk_4u32 = data.read_u32::<LE>()?;

            item.ia = mem::read_string(&mut data)?;
            item.estr = mem::read_string(&mut data)?;
            item.aa = mem::read_string(&mut data)?;
            item.overlay_object = data.read_u64::<LE>()?;

            item.unk_2u32 = data.read_u32::<LE>()?;
            data.read_exact(&mut item.unk1)?;

            item.unk_1u32 = data.read_u32::<LE>()?;
            item.unk_5u32 = data.read_u32::<LE>()?;
            item.pinfo = mem::read_string(&mut data)?;

            item.unk_3u32 = data.read_u32::<LE>()?;
            data.read_exact(&mut item.bps)?;

            item.unk_6u32 = data.read_u32::<LE>()?;
            item.unk_7u32 = data.read_u32::<LE>()?;

            assert_eq!(i, item.id as u32);

            match item.item_type {
                30
                | 133
                | item_type::DOOR
                | item_type::LOCK
                | item_type::SIGN
                | item_type::MAIN_DOOR
                | item_type::SEED
                | item_type::PORTAL
                | item_type::MAILBOX
                | item_type::BULLETIN
                | item_type::DICE
                | item_type::PROVIDER
                | item_type::ACHIEVEMENT
                | item_type::SUNGATE
                | item_type::HEART_MONITOR
                | item_type::DONATION_BOX
                | item_type::TOYBOX
                | item_type::MANNEQUIN
                | item_type::SECURITY_CAMERA
                | item_type::MAGIC_EGG
                | item_type::GAME_RESOURCES
                | item_type::GAME_GENERATOR
                | item_type::XENONITE
                | item_type::DRESSUP
                | item_type::CRYSTAL
                | item_type::BURGLAR
                | item_type::SPOTLIGHT
                | item_type::DISPLAY_BLOCK
                | item_type::VENDING_MACHINE
                | item_type::FISHTANK
                | item_type::SOLAR
                | item_type::FORGE
                | item_type::GIVING_TREE
                | item_type::GIVING_TREE_STUMP
                | item_type::STEAM_ORGAN
                | item_type::TAMAGOTCHI
                | item_type::SEWING
                | item_type::FLAG
                | item_type::LOBSTER_TRAP
                | item_type::ART_CANVAS
                | item_type::BATTLE_CAGE
                | item_type::PET_TRAINER
                | item_type::STEAM_ENGINE
                | item_type::LOCKBOT
                | item_type::WEATHER_SPECIAL
                | item_type::SPIRIT_STORAGE
                | item_type::DISPLAY_SHELF
                | item_type::VIP_ENTRANCE
                | item_type::CHAL_TIMER
                | item_type::CHAL_FLAG
                | item_type::FISH_MOUNT
                | item_type::PORTRAIT
                | item_type::WEATHER_SPECIAL2
                | item_type::FOSSIL_PREP
                | item_type::DNA_MACHINE
                | item_type::BLASTER
                | item_type::CHEMTANK
                | item_type::STORAGE
                | item_type::OVEN
                | item_type::SUPER_MUSIC
                | item_type::GEIGERCHARGE
                | item_type::ADVENTURE_RESET
                | item_type::TOMB_ROBBER
                | item_type::FACTION
                | item_type::RED_FACTION
                | item_type::GREEN_FACTION
                | item_type::BLUE_FACTION
                | item_type::FISHGOTCHI_TANK
                | item_type::MAGPLANT
                | item_type::ROBOT
                | item_type::TICKET
                | item_type::STATS_BLOCK
                | item_type::FIELD_NODE
                | item_type::OUIJA_BOARD
                | item_type::AUTO_ACTION_BREAK
                | item_type::AUTO_ACTION_HARVEST
                | item_type::AUTO_ACTION_HARVEST_SUCK
                | item_type::LIGHTNING_IF_ON
                | item_type::PHASED_BLOCK
                | item_type::PASSWORD_STORAGE
                | item_type::PHASED_BLOCK_2
                | item_type::WEATHER_INFINITY
                | item_type::COMPLETIONIST
                | item_type::FEEDING_BLOCK
                | item_type::KRANKENS_BLOCK
                | item_type::FRIENDS_ENTRANCE => {
                    item.extra = true;
                }

                _ => {}
            }

            mgr.items.push(item);
        }

        mgr.packet = TankUpdatePacket::with_extra_data(buffer);

        let end = Instant::now();
        let us = (end - start).as_micros();
        let ms = (end - start).as_millis();
        println!(
            "Serializing items takes {} microseconds / {} milliseconds!",
            us, ms
        );

        Ok(mgr)
    }

    // make sure it gets initialized at startup!
    pub fn touch(&self) {}

    pub fn get_item(&'static self, id: u32) -> &'static ItemInfo {
        &self.items[id as usize]
    }

    pub fn get_item_safe(&'static self, id: u32) -> Result<&'static ItemInfo> {
        if id >= self.items.len() as u32 {
            return Err(Error::IndexError);
        }

        Ok(&self.items[id as usize])
    }
}
