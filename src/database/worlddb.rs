use std::{
    cell::{RefCell, RefMut},
    fs::{self, File},
    io::Write,
    path::Path,
    rc::Rc,
};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::{
    consts::{item_type, tile_flags, tileextra_type},
    item::iteminfo_manager::ITEM_MANAGER,
    utils::{error::Result, math::Vec2f, mem},
    world::{tile::Tile, world::World, worldobject::WorldObject, tileextra::TileExtra},
};

const VERSION: u32 = 0;

pub fn save_world(mut world: RefMut<World>) -> Result<()> {
    if !Path::new("data/worlds").exists() {
        fs::create_dir("data/worlds")?;
    }

    let mut data = Vec::<u8>::with_capacity((world.width * world.height * 20) as usize);
    world.serialize(&mut data, None)?;

    let mut file = File::create(format!("data/worlds/{}.bin", world.name))?;
    file.write_u32::<LE>(VERSION)?;
    file.write(&data)?;

    Ok(())
}

pub fn load_world(name: &str) -> Result<Rc<RefCell<World>>> {
    if !Path::new("data/worlds").exists() {
        fs::create_dir("data/worlds")?;
    }

    let world = Rc::new(RefCell::new(World::new_none()));
    let reference = &mut world.borrow_mut();

    let mut file = File::open(format!("data/worlds/{}.bin", name))?;
    let _version = file.read_u32::<LE>()?;

    reference.version = file.read_u16::<LE>()?;
    reference.secret1 = file.read_u32::<LE>()?;
    reference.name = mem::read_string(&mut file)?;
    reference.width = file.read_u32::<LE>()?;
    reference.height = file.read_u32::<LE>()?;
    let count = file.read_u32::<LE>()?;

    reference.tiles = Vec::with_capacity(count as usize);

    for i in 0..count {
        let mut tile = Tile::new(world.clone(), i % reference.width, i / reference.width);

        tile.fore = file.read_u16::<LE>()?;
        tile.back = file.read_u16::<LE>()?;
        tile.parent = file.read_u16::<LE>()?;
        tile.flags = file.read_u16::<LE>()?;

        if tile.has_flag(tile_flags::LOCKED) {
            let _ = file.read_u16::<LE>()?;
        }

        if tile.has_flag(tile_flags::EXTRA_DATA) {
            let fore = ITEM_MANAGER.get_item_safe(tile.fore as u32)?;
            let tileextra_type = file.read_u8()?;
            match tileextra_type {
                tileextra_type::DOOR => {
                    let label = mem::read_string(&mut file)?;
                    let _ = file.read_u8()?;

                    tile.extra = TileExtra::Door { label };
                    if fore.item_type == item_type::MAIN_DOOR {
                        reference.door_pos = Vec2f {
                            x: tile.pos.x as f32 * 32.0,
                            y: tile.pos.y as f32 * 32.0,
                        }
                    }
                }

                _ => {}
            }
        }

        reference.tiles.push(tile);
    }

    let objects_count = file.read_u32::<LE>()?;
    reference.last_object_id = file.read_i32::<LE>()?;

    for _ in 0..objects_count {
        let id = file.read_u32::<LE>()?;
        let object = WorldObject {
            item_id: file.read_u16::<LE>()?,
            pos: {
                Vec2f {
                    x: file.read_f32::<LE>()?,
                    y: file.read_f32::<LE>()?,
                }
            },
            count: file.read_u8()?,
            flags: file.read_u8()?,
        };

        reference.objects.insert(id, object);
    }

    reference.weather_base_id = file.read_u32::<LE>()?;
    reference.weather_id = file.read_u32::<LE>()?;

    Ok(world.clone())
}
