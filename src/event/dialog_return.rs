use std::collections::HashMap;

use crate::{
    database::playerdb,
    utils::error::{Error, Result},
    utils::variant_function::VariantFunction::*,
};

use super::event::EventContext;

pub fn handle(ctx: EventContext, map: HashMap<&str, &str>) -> Result<()> {
    match map.get("dialog_name") {
        Some(&dialog_name) => match dialog_name {
            "growid" => {
                let name = map.get("name");
                let pass = map.get("pass");
                let verify_pass = map.get("verify_pass");

                if let Some(name) = name {
                    if let Some(pass) = pass {
                        if let Some(verify_pass) = verify_pass {
                            if name.len() < 3 || name.len() > 12 {
                                ctx.player
                                    .send_log("`4Name must between 3 to 12 characters.``");
                                return Err(Error::Disconnected);
                            }

                            if pass.len() < 8 || pass.len() > 18 {
                                ctx.player
                                    .send_log("`Password must between 8 to 18 characters.``");
                                return Err(Error::Disconnected);
                            }

                            if name.chars().any(|x| !x.is_ascii_alphanumeric()) {
                                ctx.player
                                    .send_log("`4Name cannot contains special characters.``");
                                return Err(Error::Disconnected);
                            }

                            if pass != verify_pass {
                                ctx.player.send_log("`4Password doesn't match.``");
                                return Err(Error::Disconnected);
                            }

                            ctx.player.name = name.to_string();
                            ctx.player.pass = pass.to_string();

                            match playerdb::create_player_database(ctx.player) {
                                Ok(_) => {
                                    ctx.player.send_log("`6Your account has been created! Please wait while entering the server.``");
                                    ctx.player.send_varfn(SetHasGrowID(true, name, pass));
                                    ctx.player.send_varfn_v(OnSuperMain, -1, 869);
                                    ctx.player.authenticated = true;
                                }

                                Err(e) => {
                                    match e {
                                        Error::NameAlreadyExists => {
                                            ctx.player.send_log("`4Name already exists!``");
                                        }

                                        _ => ctx.player.send_log(&format!(
                                            "`4Unknown error: {}!``",
                                            e.to_string()
                                        )),
                                    }

                                    return Err(e);
                                }
                            }
                        }
                    }
                }
            }

            _ => {}
        },

        None => return Err(Error::InvalidPacketError),
    }

    Ok(())
}
