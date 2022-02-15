use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    player::{clothing::Clothing, inventoryitem::InventoryItem, player::Player},
    utils::error::{Error, Result},
};

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
    pub pass: String,
    pub net_id: i32,
    pub user_id: i32,
    pub cloth: Clothing,
    pub items: HashMap<u16, InventoryItem>,
}

pub fn create_player_database(player: &mut Player) -> Result<()> {
    if !Path::new("data/players").exists() {
        fs::create_dir("data/players")?;
    }

    if !Path::new("data/players/userid.txt").exists() {
        fs::write("data/players/userid.txt", "1")?;
    }

    if Path::new(&format!("data/players/{}.json", player.name)).exists() {
        return Err(Error::NameAlreadyExists);
    }

    let content = fs::read_to_string("data/players/userid.txt")?;
    let user_id = content.parse::<i32>()?;
    player.user_id = user_id;

    fs::write("data/players/userid.txt", (player.user_id + 1).to_string())?;
    save_player(player)?;

    Ok(())
}

pub fn authenticate_player(player: &mut Player) -> Result<()> {
    let pass = player.pass.to_owned(); // cache it first.
    load_player(player)?;

    if player.pass != pass {
        return Err(Error::WrongPassword);
    }

    Ok(())
}

pub fn load_player(player: &mut Player) -> Result<()> {
    if !Path::new("data/players").exists() {
        fs::create_dir("data/players")?;
    }

    let content = fs::read_to_string(format!("data/players/{}.json", player.name))?;
    let data = serde_json::from_str::<PlayerData>(&content)?;

    player.name = data.name;
    player.pass = data.pass;
    player.cloth = data.cloth;
    player.items = data.items;
    player.net_id = data.net_id;
    player.user_id = data.user_id;

    Ok(())
}

pub fn save_player(player: &mut Player) -> Result<()> {
    if !Path::new("data/players").exists() {
        fs::create_dir("data/players")?;
    }

    let mut data = PlayerData {
        name: player.name.to_owned(),
        pass: player.pass.to_owned(),
        cloth: player.cloth,
        items: HashMap::new(),
        net_id: player.net_id,
        user_id: player.user_id,
    };

    for (key, pair) in player.items.iter() {
        data.items.insert(*key, *pair);
    }

    fs::write(
        format!("data/players/{}.json", data.name),
        serde_json::to_string_pretty(&data)?,
    )?;

    Ok(())
}
