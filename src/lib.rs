#![feature(lazy_cell, ptr_sub_ptr)]
use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::*;
use engage::gameuserdata::GameUserData;
use engage::gamevariable::GameVariableManager;
mod map;
mod support;
mod arena;
mod cutscene;
mod rng;
mod cook;
mod level;

#[unity::class("App", "GameSaveData")]
pub struct GameSaveData {}

#[unity::class("App", "Stream")]
pub struct Stream {}

#[skyline::from_offset(0x293a700)]
pub fn get_IsItemReturn(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2939950)]
pub fn get_well_useFlag(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "get_ExchangeLevel")]
pub fn get_well_exchangeLevel(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "get_Seed")]
pub fn get_well_seed(method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x2939a80)]
pub fn set_well_flag(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x2939dc0)]
pub fn set_well_level(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x293a100)]
pub fn set_seed(value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "GetItem")]
pub fn well_get_item(this: &u64, method_info: OptionalMethod);

#[skyline::from_offset(0x293ac80)]
pub fn well_CreateBind(this: &u64, method_info: OptionalMethod);

#[skyline::hook(offset = 0x023ed2c0)]
pub fn well_items(this: &u64, method_info: OptionalMethod){
    call_original!(this, method_info);
    unsafe {
        let sequence = GameUserData::get_sequence();
        let well_check: &str = "G_拠点_裏武器イベント";
        let check2 = GameVariableManager::get_number(well_check);
        if check2 > 2 {
            let can_get_items = get_IsItemReturn(None);
            let put_in_items = get_well_useFlag(None);
            if can_get_items == true && sequence == 5 { well_get_item(this, method_info); }
            else if put_in_items == 0 && sequence == 4 { well_CreateBind(this, method_info); }
        }
    }
}

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
    skyline::install_hooks!(load_settings, level::Set__Level, level::UnitInfo_SetLevel, well_items);
    println!("triabolical code mods are loaded");
    
    
}
