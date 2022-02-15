use crate::{item::iteminfo_manager::ITEM_MANAGER, player::clothing::Clothing, utils::math::Vec3f};

use super::{math::Vec2f, variantlist::VariantList};

pub enum VariantFunction<'a> {
    OnConsoleMessage(&'a str),
    OnSuperMain,
    OnRequestWorldSelectMenu(&'a str),
    OnFailedToEnterWorld,
    OnSetClothing(Clothing, bool),
    OnSpawn(&'a str),
    OnTalkBubble(i32, &'a str, i32, i32),
    OnRemove(&'a str),
    OnDialogRequest(&'a str),
    OnKilled(bool),
    OnSetFreezeState(bool),
    OnSetPos(Vec2f),
    OnPlayPositioned(&'a str),
    SetHasGrowID(bool, &'a str, &'a str),
}

impl<'a> VariantFunction<'a> {
    pub fn serialize(self) -> std::io::Result<VariantList<'a>> {
        let mut varlist = VariantList::new(&[]); // empty!
        match self {
            Self::OnConsoleMessage(str) => {
                varlist.push("OnConsoleMessage");
                varlist.push(str);
            }

            Self::OnSuperMain => {
                varlist.push("OnSuperMainStartAcceptLogonHrdxs47254722215a");
                varlist.push(ITEM_MANAGER.hash);
                varlist.push("ubistatic-a.akamaihd.net");
                varlist.push("0098/67197/cache/");
                varlist.push( concat!(
                    "cc.cz.madkite.freedom org.aqua.gg idv.aqua.bulldog com.cih.gamecih2 com.cih.gamecih com.cih.game_cih cn.maocai.gamekiller ",
                    "com.gmd.speedtime org.dax.attack com.x0.strai.frep com.x0.strai.free org.cheatengine.cegui org.sbtools.gamehack ",
                    "com.skgames.traffikrider org.sbtoods.gamehaca com.skype.ralder org.cheatengine.cegui.xx.multi1458919170111 com.prohiro.macro ",
                    "me.autotouch.autotouch com.cygery.repetitouch.free com.cygery.repetitouch.pro com.proziro.zacro com.slash.gamebuster"
                ));
                varlist.push(concat!(
                    "proto=149|choosemusic=audio/mp3/about_theme.mp3|active_holiday=0|wing_week_day=0|server_tick=8184975|clash_active=1|",
                    "drop_lavacheck_faster=1|isPayingUser=1|usingStoreNavigation=1|enableInventoryTab=1|bigBackpack=1|"
                ))
            }

            Self::OnRequestWorldSelectMenu(menu) => {
                varlist.push("OnRequestWorldSelectMenu");
                varlist.push(menu);
            }

            Self::OnFailedToEnterWorld => {
                varlist.push("OnFailedToEnterWorld");
                varlist.push(1);
            }

            Self::OnSetClothing(cloth, sound) => {
                varlist.push("OnSetClothing");
                varlist.push(Vec3f::new(
                    cloth.hat as f32,
                    cloth.shirt as f32,
                    cloth.pants as f32,
                ));
                varlist.push(Vec3f::new(
                    cloth.shoes as f32,
                    cloth.face as f32,
                    cloth.hand as f32,
                ));
                varlist.push(Vec3f::new(
                    cloth.back as f32,
                    cloth.hair as f32,
                    cloth.chest as f32,
                ));
                varlist.push(cloth.skin_color.as_u32());
                varlist.push(Vec3f::new(
                    cloth.ances as f32,
                    sound as i32 as f32,
                    0.0, // idk whats this
                ));
            }

            Self::OnSpawn(spawn) => {
                varlist.push("OnSpawn");
                varlist.push(spawn);
            }

            Self::OnTalkBubble(net_id, msg, color_id, overwrite) => {
                varlist.push("OnTalkBubble");
                varlist.push(net_id);
                varlist.push(msg);
                varlist.push(color_id);
                varlist.push(overwrite);
            }

            Self::OnRemove(str) => {
                varlist.push("OnRemove");
                varlist.push(str);
            }

            Self::OnDialogRequest(str) => {
                varlist.push("OnDialogRequest");
                varlist.push(str);
            }

            Self::OnKilled(state) => {
                varlist.push("OnKilled");
                varlist.push(state as i32);
            }

            Self::OnSetFreezeState(state) => {
                varlist.push("OnSetFreezeState");
                varlist.push(state as i32);
            }

            Self::OnSetPos(pos) => {
                varlist.push("OnSetPos");
                varlist.push(pos);
            }

            Self::OnPlayPositioned(audio) => {
                varlist.push("OnPlayPositioned");
                varlist.push(audio);
            }

            Self::SetHasGrowID(state, name, pass) => {
                varlist.push("SetHasGrowID");
                varlist.push(state as i32);
                varlist.push(name);
                varlist.push(pass);
            }
        }

        Ok(varlist)
    }
}
