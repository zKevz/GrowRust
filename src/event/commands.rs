use crate::{item::iteminfo_manager::ITEM_MANAGER, utils::error::Result};

use super::event::EventContext;

pub fn handle(ctx: EventContext) -> Result<()> {
    ctx.player
        .send_log(format!("`6{}``", ctx.text_data).as_str());

    if ctx.text_data.len() > 1 {
        let mut iter = ctx.text_data.split_ascii_whitespace();
        match iter.next() {
            Some(str) => match str {
                "/test" => {
                    ctx.player.send_log("Works!");
                }

                "/give" => {
                    // fucking messy..
                    if let Some(args1) = iter.next() {
                        if let Some(args2) = iter.next() {
                            let id = args1.parse::<u16>()?;
                            let count = args2.parse::<u8>()?;

                            if let Ok(item) = ITEM_MANAGER.get_item_safe(id as u32) {
                                ctx.player.add_item(id, count, true)?;
                                ctx.player
                                    .send_log(&format!(">> `6Given {} {}``!", count, item.name));

                                return Ok(());
                            }
                        }
                    }

                    ctx.player.send_log(">> Usage: /give <id> <count>");
                }

                _ => ctx.player.send_log("`4Unknown command!"),
            },

            _ => {}
        }
    }

    Ok(())
}
