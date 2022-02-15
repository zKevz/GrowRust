use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    consts,
    consts::{item_clothing, item_type, items, packet::TankUpdatePacket, packet_type},
    enet_wrapper::host::ENetHost,
    item::iteminfo_manager::ITEM_MANAGER,
    utils::variant_function::VariantFunction,
    utils::{
        color::Color,
        error::{Error, Result},
    },
    utils::{math::Vec2f, variant_function::VariantFunction::*, variantlist::VariantList},
    world::{
        world::World,
        world_manager::{WorldManager, WORLD_TYPE_NORMAL},
    },
};
use byteorder::{WriteBytesExt, LE};
use enet_sys::{
    _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE, _ENetPeer, enet_packet_create,
    enet_peer_disconnect_later, enet_peer_disconnect_now, enet_peer_send,
};

use super::{clothing::Clothing, inventoryitem::InventoryItem};

#[derive(Debug)]
pub struct Player {
    pub inner_peer: *mut _ENetPeer,

    pub hash: i32,
    pub hash2: i32,
    pub fhash: i32,
    pub net_id: i32,
    pub user_id: i32,

    pub char_flags: u32,
    pub items_slots: u32,

    pub lmode: u8,
    pub player_age: u8,
    pub platform_id: u8,

    pub f: bool,
    pub authenticated: bool,

    pub rid: String,
    pub name: String,
    pub pass: String,
    pub gdpr: String,
    pub meta: String,
    pub cbits: String,
    pub category: String,
    pub mac_address: String,
    pub country_code: String,
    pub current_world: String,
    pub device_version: String,
    pub requested_name: String,

    pub pos: Vec2f,
    pub respawn_pos: Vec2f,

    pub cloth: Clothing,

    pub items: HashMap<u16, InventoryItem>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            inner_peer: std::ptr::null::<_ENetPeer>() as *mut _ENetPeer,

            hash: 0,
            hash2: 0,
            fhash: 0,
            net_id: -1,
            user_id: -1,

            char_flags: 0,
            items_slots: 16,

            lmode: 0,
            player_age: 0,
            platform_id: 0,

            f: false,
            authenticated: false,

            rid: String::new(),
            name: String::new(),
            pass: String::new(),
            gdpr: String::new(),
            meta: String::new(),
            cbits: String::new(),
            category: String::new(),
            mac_address: String::new(),
            country_code: String::new(),
            current_world: String::from("EXIT"),
            device_version: String::new(),
            requested_name: String::new(),

            pos: Vec2f::new(0.0, 0.0),
            respawn_pos: Vec2f::new(0.0, 0.0),

            cloth: Clothing::default(),

            items: HashMap::new(),
        }
    }
}

impl Player {
    pub fn new(peer: *mut _ENetPeer) -> Player {
        let mut player = Self {
            inner_peer: peer,
            ..Default::default()
        };

        player.add_item(items::FIST, 1, false).unwrap();
        player.add_item(items::WRENCH, 1, false).unwrap();
        player.cloth.skin_color = Color::new(0xC3, 0x95, 0x82);

        player
    }

    pub fn disconnect(&mut self) {
        unsafe {
            enet_peer_disconnect_later(self.inner_peer, 0);
        }
    }

    pub fn disconnect_now(&mut self) {
        unsafe {
            enet_peer_disconnect_now(self.inner_peer, 0);
        }
    }

    pub fn send_packet(&mut self, data: &[u8]) {
        if self.inner_peer.is_null() {
            return;
        }

        unsafe {
            let packet = enet_packet_create(
                data.as_ptr() as *const _,
                data.len(),
                _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE as u32,
            );

            enet_peer_send(self.inner_peer, 0, packet);
        }
    }

    pub fn send_packet_raw(&mut self, message_type: u32, data: &[u8]) {
        self.send_packet(&[&message_type.to_le_bytes(), data, &[0]].concat());
    }

    pub fn send_tankpacket(&mut self, packet: TankUpdatePacket) {
        self.send_tankpacket_ref(&packet);
    }

    pub fn send_tankpacket_ref(&mut self, packet: &TankUpdatePacket) {
        if let Ok(packet) = packet.serialize() {
            self.send_packet_raw(consts::message_type::GAME_PACKET, &packet);
        }
    }

    pub fn send_log(&mut self, log: &str) {
        self.send_packet_raw(
            consts::message_type::GAME_MESSAGE,
            format!("action|log\nmsg|{}\n", log).as_bytes(),
        );
    }

    pub fn send_varfn(&mut self, varfn: VariantFunction) {
        self.send_varfn_v(varfn, -1, -1);
    }

    pub fn send_varfn_v(&mut self, varfn: VariantFunction, net_id: i32, delay: i32) {
        if let Ok(varlist) = varfn.serialize() {
            self.send_varlist_v(varlist, net_id, delay);
        }
    }

    pub fn send_varlist(&mut self, varlist: VariantList) {
        self.send_varlist_v(varlist, -1, -1);
    }

    pub fn send_varlist_ref(&mut self, varlist: &VariantList) {
        self.send_varlist_ref_v(&varlist, -1, -1);
    }

    pub fn send_varlist_v(&mut self, varlist: VariantList, net_id: i32, delay: i32) {
        self.send_varlist_ref_v(&varlist, net_id, delay);
    }

    pub fn send_varlist_ref_v(&mut self, varlist: &VariantList, net_id: i32, delay: i32) {
        if let Ok(data) = varlist.serialize() {
            let mut tankpacket = TankUpdatePacket::with_extra_data(data);
            tankpacket.packet_type = consts::packet_type::CALL_FUNCTION;
            tankpacket.net_id = net_id;
            tankpacket.int_val = delay;
            self.send_tankpacket(tankpacket);
        }
    }

    pub fn send_world_menu(&mut self, host: &ENetHost) {
        const MENU: &str = concat!(
            "add_button|Showing: `wRandom Worlds``|_catselect_|0.6|3418423295|\n",
            "add_floater|GROW|0|1|65535\n",
            "add_floater|RUST|0|1|16711935\n",
            "add_floater|MADE|0|1|4278190335\n",
            "add_floater|BY|0|1|4294967295\n",
            "add_floater|KEVZ|0|1|13893631\n"
        );

        self.send_varfn(OnRequestWorldSelectMenu(MENU));
        self.send_varfn(OnConsoleMessage(&format!(
            "Where would you like to go? (`w{}`` online)",
            host.online_count()
        )));
    }

    pub fn equip(&mut self, id: u16) {
        let item = ITEM_MANAGER.get_item_safe(id as u32).unwrap();
        let exec = |reference: &mut u16| {
            if *reference == id {
                *reference = 0;
            } else {
                *reference = id;
            }
        };

        if item.item_type == item_type::ANCES {
            exec(&mut self.cloth.ances);
        } else {
            match item.bodypart {
                item_clothing::HAT => exec(&mut self.cloth.hat),
                item_clothing::HAIR => exec(&mut self.cloth.hair),
                item_clothing::HAND => exec(&mut self.cloth.hand),
                item_clothing::BACK => exec(&mut self.cloth.back),
                item_clothing::PANTS => exec(&mut self.cloth.pants),
                item_clothing::SHIRT => exec(&mut self.cloth.shirt),
                item_clothing::SHOES => exec(&mut self.cloth.shoes),
                item_clothing::FACE_ITEM => exec(&mut self.cloth.face),
                item_clothing::CHEST_ITEM => exec(&mut self.cloth.chest),
                _ => panic!("Invalid bodypart type: {}!", item.bodypart),
            }
        }
    }

    pub fn display(&self) -> String {
        let color = "`w";
        return format!("{}{}``", color, self.name);
    }

    pub fn is_guest(&self) -> bool {
        return self.name.is_empty();
    }

    pub fn get_world(&self, world_manager: &mut WorldManager) -> Result<Rc<RefCell<World>>> {
        if self.current_world == "EXIT" {
            return Err(Error::NotInWorld);
        }

        Ok(world_manager.get_or_create(&self.current_world, WORLD_TYPE_NORMAL))
    }

    pub fn get_chat_color(&self) -> String {
        let color = String::new();
        // later used
        color
    }

    pub fn get_spawn_info(&self, local: bool) -> String {
        format!(
            concat!(
                "spawn|avatar\n",
                "netID|{}\n",
                "userID|{}\n",
                "colrect|0|0|20|30\n",
                "posXY|{}|{}\n",
                "name|``{}\n",
                "country|{}\n",
                "invis|{}\n",
                "mstate|{}\n",
                "smstate|{}\n",
                "{}"
            ),
            self.net_id,
            self.user_id,
            self.pos.x,
            self.pos.y,
            self.name,
            self.country_code,
            0,
            0,
            0,
            if local { "type|local\n" } else { "" },
        )
    }

    pub fn items_full(&self) -> bool {
        self.items_slots <= self.items.len() as u32
    }

    pub fn items_full_c(&self, count: u32) -> bool {
        self.items_slots <= count + self.items.len() as u32
    }

    pub fn has_item(&self, id: u16) -> bool {
        match self.items.get(&id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_item(&mut self, id: u16, count: u8, visual: bool) -> Result<()> {
        assert_ne!(count, 0);

        if count > consts::MAXIMUM_ITEMS_COUNT {
            return Err(Error::TooManyItemError);
        }

        match self.items.get_mut(&id) {
            Some(item) => {
                if item.count as usize + count as usize > consts::MAXIMUM_ITEMS_COUNT as usize {
                    return Err(Error::TooManyItemError);
                }

                item.count += count;
            }

            None => {
                if self.items_full() {
                    return Err(Error::InventoryFullError);
                }

                self.items.insert(id, InventoryItem { count, flags: 0 });
            }
        }

        if visual {
            self.send_modify_inventory(id, count, true);
        }

        Ok(())
    }

    pub fn remove_item(&mut self, id: u16, count: u8, visual: bool) -> Result<()> {
        match self.items.get_mut(&id) {
            Some(item) => {
                let difference = item.count as i32 - count as i32;
                if difference < 0 {
                    return Err(Error::ItemCountNegative);
                } else if difference == 0 {
                    self.items.remove(&id);
                } else {
                    item.count -= count;
                }
            }

            None => return Err(Error::ItemNotFound),
        }

        if visual {
            self.send_modify_inventory(id, count, false);
        }

        Ok(())
    }

    pub fn remove_item_all(&mut self, id: u16, visual: bool) -> Result<()> {
        let count: u8;
        match self.items.get(&id) {
            Some(item) => {
                count = item.count;
                self.items.remove(&id);
            }
            None => return Err(Error::ItemNotFound),
        }

        if visual {
            self.send_modify_inventory(id, count, false);
        }

        Ok(())
    }

    pub fn send_inventory(&mut self) -> Result<()> {
        let mut data = Vec::<u8>::with_capacity(self.items.len() * 20);
        data.write_u8(1)?;
        data.write_u32::<LE>(self.items_slots)?;
        data.write_u16::<LE>(self.items.len() as u16)?;

        for (id, item) in self.items.iter() {
            data.write_u16::<LE>(*id as u16)?;
            data.write_u8(item.count)?;
            data.write_u8(item.flags)?;
        }

        let mut tankpacket = TankUpdatePacket::with_extra_data(data);
        tankpacket.packet_type = packet_type::SEND_INVENTORY_STATE;
        self.send_tankpacket(tankpacket);

        Ok(())
    }

    pub fn send_modify_inventory(&mut self, id: u16, count: u8, add: bool) {
        let mut tankpacket = TankUpdatePacket::default();
        tankpacket.int_val = id as i32;
        tankpacket.packet_type = packet_type::MODIFY_ITEM_INVENTORY;

        if add {
            tankpacket.field3 = count;
        } else {
            tankpacket.field2 = count;
        }

        self.send_tankpacket(tankpacket);
    }
}
