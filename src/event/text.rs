use std::collections::HashMap;

use crate::{
    database::playerdb,
    item::iteminfo_manager::ITEM_MANAGER,
    utils::{
        error::{Error, Result},
        variant_function::VariantFunction::*,
    },
};

use super::{commands, dialog_return, event::EventContext};

fn get_map_from_str(text: &str) -> Result<(&str, HashMap<&str, &str>)> {
    let mut map = HashMap::new();

    let index = text.find('|');
    let index_of_newline = text.find('\n');
    let key = if let Some(index) = index {
        &text[..index]
    } else if let Some(index) = index_of_newline {
        &text[..index]
    } else {
        return Err(Error::InvalidPacketError);
    };

    for line in text.split('\n') {
        if line.len() <= 2 {
            continue;
        }

        let line = if line.starts_with('|') {
            &line[1..]
        } else {
            line
        };

        let index = line.find('|');
        if let Some(index) = index {
            map.insert(&line[..index], &line[(index + 1)..]);
        } else {
            map.insert(line, "");
        }
    }

    Ok((key, map))
}

pub fn handle(ctx: EventContext) -> Result<()> {
    let text = ctx.text_data;
    let player = ctx.player;
    let world_manager = ctx.world_manager;

    if text.chars().any(|x| !x.is_ascii()) {
        return Err(Error::InvalidPacketError);
    }

    let (key, map) = get_map_from_str(text)?;

    if let Some(&action) = map.get("action") {
        match action {
            "quit" => return Err(Error::Disconnected),

            "dialog_return" => {
                let ctx = EventContext {
                    host: ctx.host,
                    player,
                    world_manager,
                    text_data: text,
                    packet_data: ctx.packet_data,
                };

                return dialog_return::handle(ctx, map);
            }

            _ => {}
        }
    }

    if player.authenticated {
        match key {
            "action" => match map.get("action") {
                Some(&action) => match action {
                    "enter_game" => {
                        const DIALOG: &str = concat!(
                            "set_default_color|`o|\n",
                            "add_label_with_icon|big|Grow Rust|left|18|\n",
                            "add_spacer|small|\n",
                            "add_textbox|Hello player! You are in `6Grow Rust`` server!|left|\n",
                            "add_textbox|This server was made by `6kevz#2211``.|left|\n",
                            "add_textbox|What makes this server unique is that it was built with the `6Rust`` programming language.|left|\n",
                            "add_textbox|This project is opensourced at my (`6kevz#2211``) github.|left|\n",
                            "add_textbox|Anyways, if you are going to use this server as base server, don't forget to credit me! :D|left|\n",
                            "add_spacer|small|\n",
                            "add_url_button||`1Kevz's Github``|NOFLAGS|https://github.com/zKevz|Visit `1Kevz's Github``?|0|0|\n",
                            "add_url_button||`1Project's Repository``|NOFLAGS|https://github.com/zKevz/GrowRust|Visit this project repository in github?|0|0|\n",
                            "add_spacer|small|\n",
                            "end_dialog|lol||Close|\n",
                            "add_quick_exit\n"
                        );

                        player.send_log("`9Welcome to `6Grow Rust``!``");
                        player.send_inventory()?;
                        player.send_world_menu(ctx.host);
                        player.send_varfn(OnDialogRequest(DIALOG));
                    }

                    "refresh_item_data" => {
                        player.send_tankpacket_ref(&ITEM_MANAGER.packet);
                    }

                    "join_request" => match map.get("name") {
                        Some(name) => {
                            world_manager.join_world(player, name, None, true)?;
                        }
                        _ => {}
                    },

                    "getDRAnimations" => {}

                    "respawn" => {
                        if let Ok(world) = player.get_world(world_manager) {
                            let mut world = world.borrow_mut();
                            world.respawn_player(player, false);
                        } else {
                            return Err(Error::InvalidPacketError);
                        }
                    }

                    "respawn_spike" => {
                        if let Ok(world) = player.get_world(world_manager) {
                            let mut world = world.borrow_mut();
                            world.respawn_player(player, true);
                        } else {
                            return Err(Error::InvalidPacketError);
                        }
                    }

                    "input" => match map.get("text") {
                        Some(text) => {
                            if text.starts_with('/') {
                                let ctx = EventContext {
                                    host: ctx.host,
                                    player,
                                    world_manager,
                                    text_data: text,
                                    packet_data: ctx.packet_data,
                                };

                                commands::handle(ctx)?;
                            } else {
                                if let Ok(world) = player.get_world(world_manager) {
                                    let chat_color = player.get_chat_color();
                                    let mut world = world.borrow_mut();

                                    world.push_varfn(OnTalkBubble(
                                        player.net_id,
                                        &format!("{}{}``", chat_color, text),
                                        0,
                                        0,
                                    ));
                                    world.push_varfn(OnConsoleMessage(&format!(
                                        "CP:0_PL:4_OID:_CT:[W]_ `o<`w{}``> {}{}``",
                                        player.display(),
                                        chat_color,
                                        text
                                    )));
                                    world.send_all();
                                } else {
                                    return Err(Error::InvalidPacketError);
                                }
                            }
                        }
                        _ => {}
                    },

                    "quit_to_exit" => {
                        if let Ok(world) = player.get_world(world_manager) {
                            world_manager.exit_world(player, world.borrow_mut());
                            player.send_world_menu(ctx.host);
                        } else {
                            return Err(Error::InvalidPacketError);
                        }
                    }

                    _ => println!("Unhandled action: '{}'", action),
                },

                None => return Err(Error::InvalidPacketError),
            },

            _ => {}
        }
    } else {
        match key {
            "tankIDName" | "requestedName" => {
                let get_value = |key: &str| -> Result<&str> {
                    match map.get(key) {
                        Some(&value) => Ok(value),
                        None => Err(Error::InvalidPacketError),
                    }
                };

                if let Ok(name) = get_value("tankIDName") {
                    player.name = name.to_string();
                    player.pass = get_value("tankIDPass")?.to_string();
                }

                player.rid = get_value("rid")?.to_string();
                player.meta = get_value("meta")?.to_string();
                player.gdpr = get_value("GDPR")?.to_string();
                player.cbits = get_value("cbits")?.to_string();
                player.category = get_value("category")?.to_string();
                player.mac_address = get_value("mac")?.to_string();
                player.country_code = get_value("country")?.to_string();
                player.device_version = get_value("deviceVersion")?.to_string();
                player.requested_name = get_value("requestedName")?.to_string();

                player.hash = get_value("hash")?.parse()?;
                player.fhash = get_value("fhash")?.parse()?;
                player.lmode = get_value("lmode")?.parse()?;
                player.player_age = get_value("player_age")?.parse()?;
                player.platform_id = get_value("platformID")?.parse()?;
                player.f = get_value("f")? == "1";

                if player.is_guest() {
                    const DIALOG: &str = concat!(
                        "set_default_color|`o|\n",
                        "add_label_with_icon|big|GrowID Creation|left|204|\n",
                        "add_spacer|small|\n",
                        "add_button|create|`6Create!``|noflags|0|0|\n",
                        "add_text_input|name|GrowID:||12|\n",
                        "add_smalltext|GrowID should be between 3 and 12 characters long.|\n",
                        "add_text_input_password|pass|Password:||18|\n",
                        "add_text_input_password|verify_pass|Verify Password:||18|\n",
                        "add_smalltext|Password should be between 8 and 18 characters long.|\n",
                        "add_spacer|small|\n",
                        "end_dialog|growid|||\n"
                    );

                    player.send_varfn(OnDialogRequest(DIALOG));
                    return Ok(());
                } else {
                    match playerdb::authenticate_player(player) {
                        Ok(_) => {}
                        Err(e) => {
                            player.send_log("`4Invalid username or password.``");
                            return Err(e);
                        }
                    }
                }

                player.authenticated = true;
                player.send_varfn(OnSuperMain);
            }
            _ => return Err(Error::InvalidPacketError),
        }
    }

    Ok(())
}
