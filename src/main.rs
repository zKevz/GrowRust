use std::{fs, net::Ipv4Addr, path::Path};

use server::{
    consts,
    database::playerdb,
    enet_wrapper::{self, event::ENetEventType, host::ENetHost},
    event::event::{self, EventContext},
    item::iteminfo_manager::ITEM_MANAGER,
    player::player::Player,
    world::world_manager::WorldManager,
};

const PORT: u16 = 10000;

fn main() {
    enet_wrapper::initialize();

    if !Path::new("data").exists() {
        fs::create_dir("data").unwrap();
    }

    if !Path::new("data/items.dat").exists() {
        eprintln!("Error: Could not find items.dat in 'data/items.dat'!");
        return;
    }

    ITEM_MANAGER.touch();

    let mut host = ENetHost::new(Ipv4Addr::UNSPECIFIED, PORT, 1024);
    let mut world_manager = WorldManager::new();

    loop {
        if let Some(event) = host.service(5) {
            //let time_start = Instant::now();

            match event {
                ENetEventType::None => continue,

                ENetEventType::Connect(mut peer) => {
                    let mut player = Player::new(peer.get_inner());
                    player.send_packet_raw(consts::message_type::SERVER_HELLO, &[]);

                    peer.set_data(Some(player));
                }

                ENetEventType::Disconnect(mut peer) => match peer.get_data::<Player>() {
                    Some(player) => {
                        if let Ok(world) = player.get_world(&mut world_manager) {
                            world_manager.exit_world(player, world.borrow_mut());
                        }

                        if player.authenticated && !player.is_guest() {
                            match playerdb::save_player(player) {
                                Ok(_) => {}
                                Err(e) => println!("Failed to save player! Error: {}", e),
                            }
                        }

                        peer.set_data::<Player>(None);
                        println!("Player disconnected!");
                    }

                    _ => {}
                },

                ENetEventType::Receive(mut peer, enet_packet) => match peer.get_data::<Player>() {
                    Some(player) => {
                        let ctx = EventContext {
                            host: &host,
                            player: player,
                            world_manager: &mut world_manager,

                            text_data: "",
                            packet_data: enet_packet.get_data(),
                        };

                        if event::handle(ctx).is_err() {
                            player.disconnect();
                        }
                    }

                    _ => {}
                },
            }

            // disabling this cuz it fucking flooded the console
            // let time_end = Instant::now();
            // let micros = (time_end - time_start).as_micros();
            // println!("Event took {} microseconds!", micros);
        }
    }
}
