use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    path::Path,
    rc::Rc,
};

use crate::{
    consts::message_type,
    database::worlddb,
    enet_wrapper::peer::ENetPeer,
    player::player::Player,
    utils::error::{Error, Result},
    utils::math::Vec2f,
    utils::variant_function::VariantFunction::*,
};

use super::world::World;

pub const WORLD_TYPE_NORMAL: u8 = 0;

pub struct WorldManager {
    pub worlds: HashMap<String, Rc<RefCell<World>>>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Rc<RefCell<World>> {
        self.worlds.get(&name.to_uppercase()).unwrap().clone()
    }

    pub fn get_safe(&self, name: &str) -> Option<Rc<RefCell<World>>> {
        match self.worlds.get(&name.to_uppercase()) {
            Some(world) => Some(world.clone()),
            None => None,
        }
    }

    pub fn get_or_create(&mut self, name: &str, world_type: u8) -> Rc<RefCell<World>> {
        let name = name.to_uppercase();
        if !self.worlds.contains_key(&name) {
            let world: Rc<RefCell<World>>;

            if Path::new(&format!("data/worlds/{}.bin", name)).exists() {
                world = worlddb::load_world(&name).unwrap(); // unwrap incase i fucked something
            } else {
                match world_type {
                    WORLD_TYPE_NORMAL => {
                        world = World::generate_normal(name.to_owned());
                    }

                    _ => panic!("Invalid world type!"),
                }

                worlddb::save_world(world.borrow_mut()).unwrap();
            }

            self.worlds.insert(name.to_owned(), world);
        }

        self.worlds.get(&name).unwrap().clone()
    }

    pub fn is_valid(&self, player: &mut Player, name: &String) -> bool {
        let err: &str;
        if name.is_empty() {
            err = "Sorry, world names cannot be empty.  Try again.";
        } else if name == "EXIT" {
            err = "Exit from what? Press back if you're done playing.";
        } else if name.len() > 15 {
            err = "World name too long, try again.";
        } else if name.chars().any(|x| !x.is_alphanumeric()) {
            err = "Sorry, spaces and special characters are not allowed in world or door names.  Try again.";
        } else {
            return true;
        }

        player.send_varfn(OnConsoleMessage(err));
        player.send_varfn(OnFailedToEnterWorld);

        false
    }

    pub fn join_world(
        &mut self,
        player: &mut Player,
        name: &str,
        pos: Option<Vec2f>,
        check: bool,
    ) -> Result<()> {
        let name = name.to_uppercase();

        if check && !self.is_valid(player, &name) {
            return Err(Error::NameError);
        }

        if let Ok(world) = player.get_world(self) {
            self.exit_world(player, world.borrow_mut());
        }

        let world = self.get_or_create(name.as_str(), WORLD_TYPE_NORMAL);
        let mut world = world.borrow_mut();

        player.pos = pos.unwrap_or(world.door_pos);
        player.net_id = world.get_net_id();
        player.respawn_pos = world.door_pos;
        player.current_world = world.name.to_owned();

        world.serialize_to(player)?;

        let spawn = player.get_spawn_info(false);

        let packet1 = OnSpawn(&spawn).serialize()?;
        let packet2 = OnSetClothing(player.cloth, false).serialize()?;

        player.send_varfn(OnSpawn(&player.get_spawn_info(true)));
        player.send_varlist_ref_v(&packet2, player.net_id, -1);
        player.send_packet_raw(
            message_type::GAME_MESSAGE,
            "action|play_sfx\nfile|audio/door_open.wav\ndelayMS|0\n".as_bytes(),
        );

        for x in world.iter() {
            x.send_varlist_ref(&packet1);
            x.send_varlist_ref_v(&packet2, player.net_id, -1);
            x.send_packet_raw(
                message_type::GAME_MESSAGE,
                "action|play_sfx\nfile|audio/door_open.wav\ndelayMS|0\n".as_bytes(),
            );

            player.send_varfn(OnSpawn(&x.get_spawn_info(false)));
            player.send_varfn_v(OnSetClothing(x.cloth, false), x.net_id, -1);
        }

        world.peers.push(ENetPeer::new(player.inner_peer));

        Ok(())
    }

    pub fn exit_world(&mut self, player: &mut Player, mut world: RefMut<World>) {
        player.current_world = "EXIT".to_string();

        for (i, peer) in world.peers.iter().enumerate() {
            if peer.inner_peer == player.inner_peer {
                let str = format!(
                    "`5<{} left, `w{}`` others here>``",
                    player.display(),
                    world.peers.len()
                );

                world.push_packet_raw(
                    message_type::GAME_MESSAGE,
                    "action|play_sfx\nfile|audio/door_shut.wav\ndelayMS|0\n".as_bytes(),
                );
                world.push_varfn(OnConsoleMessage(&str));
                world.push_varfn(OnTalkBubble(player.net_id, &str, 0, 0));
                world.push_varfn(OnRemove(&format!("netID|{}\n", player.net_id)));
                world.send_all();

                world.peers.remove(i);

                // if there are no players left, remove the world from cache!
                if world.peers.is_empty() {
                    self.worlds.remove(&world.name);
                    worlddb::save_world(world).unwrap();
                }

                break;
            }
        }
    }
}
