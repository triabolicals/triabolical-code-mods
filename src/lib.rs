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
use engage::gamedata::skill::SkillArray;
use crate::string::Mess_Get;
pub static mut rigPlayer: bool = false;

mod bgm;
mod map;
mod menus;
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
#[unity::class("App", "BattleInfoSide")]
pub struct BattleInfoSide {
    pub m_info: &'static BattleInfo,
    pub m_side_type: i32,
    pad: i32,
    pub unit: Option<&'static Unit>,
}
#[unity::class("App", "BattleCalculator")]
pub struct BattleCalculator {
    pub m_Mode: i32,
    pub m_info: &'static BattleInfo,

}

#[skyline::from_offset(0x01e7f750)]
pub fn BattleInfo_GetUnit(this: &BattleInfo, index: i32,  method_info: OptionalMethod) -> &Unit; 

#[skyline::hook(offset=0x02470d60)]
pub fn calc_action_hook(this: &BattleCalculator, side_type: i32, method_info: OptionalMethod) -> bool {
    unsafe {
        if GameVariableManager::get_number(rng::RNG_KEY) == 4 { 
            Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]);
            Patch::in_text(0x02375504).bytes(&[0x00, 0x08, 0x22, 0x1e]);
            Patch::in_text(0x02375508).bytes(&[0x81, 0x01, 0x22, 0x1e]);
            let unit = BattleInfo_GetUnit(this.m_info, side_type, method_info);
            if unit.m_Force.is_some() {
                if unit.person.get_asset_force() == 0 || unit.m_Force.unwrap().force_type == 0 { 
                    Patch::in_text(0x01e8d12c).bytes(&[0x20, 0x00, 0x80, 0x52]);
                    Patch::in_text(0x02375508).bytes(&[0x01,0x10, 0x2E, 0x1E]);
                }
                else { 
                    Patch::in_text(0x01e8d12c).bytes(&[0xE0,0xE1, 0x84, 0x52]);
                    Patch::in_text(0x02375504).bytes(&[0x6C,0x0C, 0x80, 0x52]);
                }
            }
        }
    }
    let result = call_original!(this, side_type, method_info);
    Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]);
    Patch::in_text(0x02375504).bytes(&[0x00, 0x08, 0x22, 0x1e]);
    Patch::in_text(0x02375508).bytes(&[0x81, 0x01, 0x22, 0x1e]);
    return result;
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
        rng::patch_smash();
        arena::patchArena();
        cook::patchCook();
        level::patchGrowth();
        character::get_lueur_gender();
        character::changeCharacters();
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

    Patch::in_text(0x01fdea34).bytes(&[0x01,0x04,0x80, 0x52]);

    // Lunatic Random 
    return_true(0x01bfaec0);
    // Enable Growth Select
    return_true(0x01bfae40);

    //DLC
    //return_true(0x029f4270);

    bgm::bgm_install(); 
    cutscene::cutscene_install(); //Cutscenes
    support::support_install(); //Support
    map::map_mod_install(); // Map Dialogue/Tutorials
    arena::arena_install(); //Arena Skip
    cook::cook_install(); // Cooking Outcomes
    exp::exp_install(); //Exp mode
    level::level_install(); //Level Display and Growth Type
    rng::rng_install(); // RNG 
    character::char_install();  // Single Character Mode
    gift::gift_install(); //Gift Settings
    //cobapi::register_system_event_handler(load_settings);
    skyline::install_hooks!(string::HelpParamSetter_SetItemData, bgm::ChangeBGM, bgm::ChangeBGM2, level::LevelUpWindow_SetupParams);
    skyline::install_hooks!(calc_action_hook);
   // skyline::install_hooks!(hub::well_items, exp::hybrid_hook, exp::prob_100);
    skyline::install_hooks!( menus::GmapMenu_SubShopMenu_CreateBind,menus::NoticeBoardSequence_CreateBind,menus::hub_menu_create_bind);
    skyline::install_hooks!(rng::hybrid_hook, exp::SetBattleInfo, load_settings1, arena::arena_finish_training );
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
