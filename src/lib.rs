#![feature(lazy_cell, ptr_sub_ptr)]
use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, unit::*};
use engage::gameuserdata::GameUserData;
use engage::gameuserdata::*;
use engage::gamevariable::GameVariableManager;
use cobapi::Event;
use cobapi::SystemEvent;
use unity::il2cpp::object::Array;
use engage::gamedata::item::ItemData;
use engage::gamedata::person::SkillArray;
use crate::string::Mess_Get;
pub static mut rigPlayer: bool = false;
mod bgm;
mod map;
mod support;
mod arena;
mod cutscene;
mod rng;
mod cook;
mod level;
mod hub; 
mod exp;
mod gift;
mod character;
mod string;

/* 
#[no_mangle]
extern "C" fn load_settings(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::CatalogLoaded => println!("If you only care about knowing when files have been added to the game, handle it here."),
            SystemEvent::SaveLoaded { slot_id } => {
                println!("The slot being loaded was #{}", slot_id);
                rng::patchRNG();
                support::patchSupport();
                cutscene::patchCutscenes();
                map::patchMap();
                arena::patchArena();
                cook::patchCook();
                level::patchGrowth();
                character::changeCharacters();
            },
            // This syntax means you do not intend to deal with the other events and will do nothing if they are received.
            _ => (),
        }
    } 
    else { 
        println!("We received a missing event, and we don't care!");
     }
}
*/
#[unity::class("App", "BattleInfo")]
pub struct BattleInfo {}

#[unity::class("App", "BattleCalculator")]
pub struct BattleCalculator {
    pub m_Mode: i32,
    pub m_info: &'static BattleInfo,

}
#[skyline::from_offset(0x01e7f750)]
pub fn BattleInfo_GetUnit(this: &BattleInfo, index: i32,  method_info: OptionalMethod) -> &Unit; 

#[skyline::hook(offset=0x02471060)]
pub fn CalcAttack(this: &BattleCalculator, sideType: i32, method_info: OptionalMethod) -> bool {
    unsafe {
        let unit = BattleInfo_GetUnit(this.m_info, sideType, method_info);
        if unit.m_Force.is_some() {
            if unit.m_Force.unwrap().m_Type == 0 || unit.m_Force.unwrap().m_Type == 2 { rigPlayer = true; }
            else { rigPlayer = false; }
        }
        else { rigPlayer = false; }
        let rig = GameVariableManager::get_number(rng::RNG_KEY);
        if rig == 4 && rigPlayer { Patch::in_text(0x02375510).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap(); }
        let result = call_original!(this, sideType, method_info);
        if rig == 4 || rig == 0 { Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap(); }
        return result;
    }
}
#[skyline::hook(offset=0x01e8d420)]
pub fn RandomCheckHit(this: i32, method_info: OptionalMethod) -> bool {
    let result = call_original!(this, method_info);
    unsafe {
        if GameVariableManager::get_number(rng::RNG_KEY) == 4 {
            if rigPlayer && this > 0 { 
                Patch::in_text(0x02375510).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
                if !result { 
                    println!("Hit Ratio: {} - Converting this miss into hit for player unit.", this);
                    return true;
                }
             }
            else if !rigPlayer && this < 100 { 
                Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap();
                if result {
                    println!("Hit Ratio: {} - Converting this hit into miss for enemy unit.", this);
                    return false; 
                }
            }
        }
    }
    return  result;
}


#[skyline::hook(offset = 0x2281a80)]
pub fn load_settings1(this: u64, stream: u64, method_info: OptionalMethod) -> bool {
    let value: bool = call_original!(this, stream, None);
    if value {
        gift::patch_gift();
        rng::patchRNG();
        support::patchSupport();
        cutscene::patchCutscenes();
        map::patchMap();
        arena::patchArena();
        cook::patchCook();
        level::patchGrowth();
        //character::get_lueur_gender();
        //character::changeCharacters();
        bgm::patch_bgm();
    }
    return value;
}
pub fn return_true(address: usize){
    Patch::in_text(address).bytes(&[0x20,0x00, 0x80, 0x52]);
    Patch::in_text(address+0x4).bytes(&[0xC0, 0x03, 0x5F, 0xD6]);
}
#[skyline::main(name = "libtriabolical")]
pub fn main() {
    //Enables support/bond viewing in maps and exploration
    let replace = &[0x1f, 0x25, 0x00, 0x71];
    Patch::in_text(0x0209950C).bytes(replace).unwrap();
    Patch::in_text(0x020994E0).bytes(replace).unwrap();
    Patch::in_text(0x02099538).bytes(replace).unwrap();
    Patch::in_text(0x01a2a7c0).bytes(&[0xe1,0x0e,0x80,0x12]);
    Patch::in_text(0x01a2a7c4).bytes(&[0x02,0x0f,0x80,0x52]);

    //Level Up Window gauge size
    //Patch::in_text(0x01beaa00).bytes(&[0xC0,0x12,0x80, 0x52]);

    Patch::in_text(0x01fdea34).bytes(&[0x01,0x04,0x80, 0x52]);
    bgm::bgm_install(); 
    cutscene::cutscene_install(); //Cutscenes
    support::support_install(); //Support
    map::map_mod_install(); // Map Dialogue/Tutorials
    arena::arena_install(); //Arena Skip
    cook::cook_install(); // Cooking Outcomes
    exp::exp_install(); //Exp mode
    level::level_install(); //Level Display and Growth Type
    rng::rng_install(); // RNG 
    //character::char_install();  // Single Character Mode
    gift::gift_install(); //Gift Settings
    //hub::well_install();
    //cobapi::register_system_event_handler(load_settings);
    skyline::install_hooks!(string::HelpParamSetter_SetItemData, bgm::ChangeBGM, bgm::ChangeBGM2, level::LevelUpWindow_SetupParams);
    skyline::install_hooks!(RandomCheckHit, CalcAttack);
    //skyline::install_hooks!(hub::hub_next_map, hub::NextGmap, hub::well_items);
    skyline::install_hooks!(exp::SetBattleInfo, load_settings1);
    skyline::install_hooks!(level::Set__Level, level::UnitInfo_SetLevel, exp::addExp, exp::normalizeExp, exp::LevelUp_Prepare, exp::unit_add_exp);
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
