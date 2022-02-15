use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

use byteorder::{WriteBytesExt, LE};
use enet_sys::{
    _ENetPacket, _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE, enet_packet_create, enet_peer_send,
};

use crate::{
    consts::{self, items, message_type, packet::TankUpdatePacket, packet_flags, packet_type},
    enet_wrapper::peer::ENetPeer,
    player::player::Player,
    utils::{
        self,
        error::{Error, Result},
        math::{Vec2f, Vec2u},
        mem,
        variant_function::VariantFunction::{self, *},
        variantlist::VariantList,
    },
    world::tileextra::TileExtra,
};

use super::{tile::Tile, worldobject::WorldObject};

pub struct World {
    pub secret1: u32,
    pub name: String,
    pub tiles: Vec<Tile>,
    pub peers: Vec<ENetPeer>,
    pub width: u32,
    pub height: u32,
    pub version: u16,
    pub packets: Vec<*mut _ENetPacket>,
    pub objects: HashMap<u32, WorldObject>,
    pub door_pos: Vec2f,
    pub weather_id: u32,
    pub last_net_id: i32,
    pub last_object_id: i32,
    pub weather_base_id: u32,
}

impl World {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            name,
            width,
            height,
            secret1: 0x40,
            peers: Vec::new(),
            tiles: Vec::with_capacity((width * height) as usize),
            version: 0x14,
            packets: Vec::new(),
            objects: HashMap::new(),
            door_pos: Vec2f::new(0.0, 0.0),
            weather_id: 0,
            last_net_id: 0,
            last_object_id: 0,
            weather_base_id: 0,
        }
    }

    pub fn new_none() -> Self {
        Self {
            name: String::new(),
            secret1: 0x40,
            peers: Vec::new(),
            tiles: Vec::new(),
            width: 0,
            height: 0,
            version: 0x14,
            packets: Vec::new(),
            objects: HashMap::new(),
            door_pos: Vec2f::new(0.0, 0.0),
            weather_id: 0,
            last_net_id: 0,
            last_object_id: 0,
            weather_base_id: 0,
        }
    }

    pub fn generate_normal(name: String) -> Rc<RefCell<Self>> {
        const WIDTH: u32 = 100;
        const HEIGHT: u32 = 60;

        const LAVA_LAYER: u32 = HEIGHT - 10;
        const ROCK_LAYER: u32 = (HEIGHT / 2) + 2;
        const START_LAYER: u32 = HEIGHT / 2;
        const BEDROCK_LAYER: u32 = HEIGHT - 5;

        let world = Rc::new(RefCell::new(World::new(name, WIDTH, HEIGHT)));

        let main_door_pos = Vec2u {
            x: utils::random(0..WIDTH),
            y: START_LAYER - 1,
        };

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut tile = Tile::new(world.clone(), x, y);
                let mut reference = world.borrow_mut();

                if y >= START_LAYER {
                    tile.set_back(items::CAVE_BACKGROUND);

                    if y >= BEDROCK_LAYER {
                        tile.set_fore(items::BEDROCK);
                    } else if main_door_pos.x == x && main_door_pos.y + 1 == y {
                        tile.set_fore(items::BEDROCK);
                    } else if y <= BEDROCK_LAYER && y >= LAVA_LAYER {
                        let chance: i32 = utils::random(0..75);

                        if chance > 20 {
                            tile.set_fore(items::DIRT);
                        } else if chance > 3 {
                            tile.set_fore(items::LAVA);
                        } else {
                            tile.set_fore(items::ROCK);
                        }
                    } else if y >= ROCK_LAYER {
                        let chance: i32 = utils::random(0..55);

                        if chance <= 1 {
                            tile.set_fore(items::ROCK);
                        } else {
                            tile.set_fore(items::DIRT);
                        }
                    } else {
                        tile.set_fore(items::DIRT);
                    }
                } else if main_door_pos.x == x && main_door_pos.y == y {
                    tile.set_fore(items::MAIN_DOOR);
                    tile.extra = TileExtra::Door {
                        label: "EXIT".to_string(),
                    };
                    
                    reference.door_pos = Vec2f {
                        x: (x * 32) as f32,
                        y: (y * 32) as f32,
                    };
                }

                reference.tiles.push(tile);
            }
        }

        world
    }

    pub fn iter(&mut self) -> impl std::iter::Iterator<Item = &mut Player> + '_ {
        let a = self
            .peers
            .iter_mut()
            .map(|x| x.get_data::<Player>().unwrap());
        a
    }

    pub fn get_net_id(&mut self) -> i32 {
        let id = self.last_net_id;
        self.last_net_id += 1;
        id
    }

    pub fn push_packet(&mut self, data: &[u8]) {
        unsafe {
            let packet = enet_packet_create(
                data.as_ptr() as *const _,
                data.len(),
                _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE as u32,
            );

            self.packets.push(packet);
        }
    }

    pub fn push_packet_raw(&mut self, message_type: u32, data: &[u8]) {
        self.push_packet(&[&message_type.to_le_bytes(), data, &[0]].concat());
    }

    pub fn push_tankpacket(&mut self, tankpacket: TankUpdatePacket) {
        if let Ok(packet) = tankpacket.serialize() {
            self.push_packet_raw(message_type::GAME_PACKET, &packet);
        }
    }

    pub fn push_varfn(&mut self, varfn: VariantFunction) {
        self.push_varfn_v(varfn, -1, -1);
    }

    pub fn push_varfn_v(&mut self, varfn: VariantFunction, net_id: i32, delay: i32) {
        if let Ok(varlist) = varfn.serialize() {
            self.push_varlist_v(varlist, net_id, delay);
        }
    }

    pub fn push_varlist(&mut self, varlist: VariantList) {
        self.push_varlist_v(varlist, -1, -1);
    }

    pub fn push_varlist_v(&mut self, varlist: VariantList, net_id: i32, delay: i32) {
        if let Ok(data) = varlist.serialize() {
            let mut tankpacket = TankUpdatePacket::with_extra_data(data);
            tankpacket.packet_type = consts::packet_type::CALL_FUNCTION;
            tankpacket.net_id = net_id;
            tankpacket.int_val = delay;
            self.push_tankpacket(tankpacket);
        }
    }

    pub fn send_all(&mut self) {
        if self.packets.is_empty() {
            return;
        }

        for packet in self.packets.iter() {
            for peer in self.peers.iter_mut() {
                unsafe {
                    enet_peer_send(peer.get_inner(), 0, *packet);
                }
            }
        }

        self.packets.clear();
    }

    pub fn respawn_player(&mut self, player: &mut Player, killed: bool) {
        self.respawn_player_with_delay(player, killed, -1);
    }

    pub fn respawn_player_with_delay(&mut self, player: &mut Player, killed: bool, mut delay: i32) {
        if !killed {
            player.send_varfn_v(OnKilled(true), player.net_id, delay);
            player.send_varfn_v(OnSetFreezeState(true), player.net_id, delay);
        }

        delay += 2000;
        player.send_varfn_v(OnSetPos(self.door_pos), player.net_id, delay);
        player.send_varfn_v(OnSetFreezeState(false), player.net_id, delay);

        self.push_varfn_v(OnPlayPositioned("audio/teleport.wav"), player.net_id, delay);
        self.send_all();
    }

    pub fn get_tile_safe<'a>(&'a mut self, x: u32, y: u32) -> Result<&'a mut Tile> {
        let index = x + y * self.width;
        if index >= self.tiles.len() as u32 {
            return Err(Error::IndexError);
        }

        Ok(&mut self.tiles[index as usize])
    }

    pub fn serialize_to(&mut self, player: &mut Player) -> Result<()> {
        let start = Instant::now();

        let mut data = Vec::<u8>::with_capacity((self.width * self.height * 10) as usize); // so that it doesnt need to reallocate everytime
        let result = self.serialize(&mut data, Some(player));

        let end = Instant::now();
        let us = (end - start).as_micros();
        let ms = (end - start).as_millis();

        let mut tankpacket = TankUpdatePacket::with_extra_data(data);
        tankpacket.flags |= packet_flags::EXTENDED;
        tankpacket.packet_type = packet_type::SEND_MAP_DATA;

        player.send_tankpacket(tankpacket);
        player.send_varfn(OnConsoleMessage(&format!(
            "World packet takes {} microseconds / {} milliseconds.",
            us, ms
        )));

        result
    }

    pub fn serialize(&mut self, data: &mut Vec<u8>, player: Option<&mut Player>) -> Result<()> {
        data.write_u16::<LE>(self.version)?;
        data.write_u32::<LE>(self.secret1)?;
        mem::write_string(data, &self.name)?;
        data.write_u32::<LE>(self.width)?;
        data.write_u32::<LE>(self.height)?;
        data.write_u32::<LE>(self.width * self.height)?;

        for tile in self.tiles.iter_mut() {
            tile.serialize(data, &player)?;
        }

        let objects_count = self.objects.len() as u32;
        data.write_u32::<LE>(objects_count)?;
        data.write_i32::<LE>(self.last_object_id)?;

        for (id, object) in self.objects.iter() {
            data.write_u32::<LE>(*id)?;
            data.write_u16::<LE>(object.item_id)?;
            data.write_f32::<LE>(object.pos.x)?;
            data.write_f32::<LE>(object.pos.y)?;
            data.write_u8(object.count)?;
            data.write_u8(object.flags)?;
        }

        data.write_u32::<LE>(self.weather_base_id)?;
        data.write_u32::<LE>(self.weather_id)?;

        Ok(())
    }
}
