#![feature(lazy_cell, ptr_sub_ptr)]
use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, unit::*};
use engage::gameuserdata::GameUserData;
use engage::gameuserdata::*;
use engage::gamevariable::GameVariableManager;
mod map;
mod support;
mod arena;
mod cutscene;
mod rng;
mod cook;
mod level;
mod hub; 

#[unity::class("App", "GameSaveData")]
pub struct GameSaveData {}

#[unity::class("App", "Stream")]
pub struct Stream {}


#[skyline::hook(offset = 0x2281a80)]
pub fn load_settings(this: &GameSaveData, stream: &Stream, method_info: OptionalMethod) -> bool {
    let value: bool = call_original!(this, stream, None);
    if value {
        rng::patchRNG();
        support::patchSupport();
        cutscene::patchCutscenes();
        map::patchMap();
        arena::patchArena();
        cook::patchCook();
        level::patchGrowth();
        level::changeCharacters();
    }
    return value;
}

#[skyline::main(name = "libtriabolical")]
pub fn main() {
    //Enables support/bond viewing in maps and exploration
    let replace = &[0x1f, 0x25, 0x00, 0x71];
    Patch::in_text(0x0209950C).bytes(replace).unwrap();
    Patch::in_text(0x020994E0).bytes(replace).unwrap();
    Patch::in_text(0x02099538).bytes(replace).unwrap();


    // testing 0%
    cutscene::cutscene_install();
    support::support_install();
    map::map_mod_install();
    arena::arena_install();
    level::level_install();
    rng::rng_install();
    cook::cook_install();
    skyline::install_hooks!(cutscene::TryGiftEvent, cutscene::ChangeBGM, cutscene::ChangeBGM2, rng::Unit_LevelUP, load_settings, level::Set__Level, level::UnitInfo_SetLevel, hub::well_items, hub::hub_next_map, hub::NextGmap, rng::addExp, rng::normalizeExp, rng::LevelUp_Prepare, rng::unit_add_exp);
    println!("triabolical code mods are loaded");

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };
        let err_msg = format!(
            "triabolical code mod plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );
        skyline::error::show_error(
            3,
            "Triabolical has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
}
