#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use skyline::patching::Patch;
use engage::{
    gamedata::unit::*,
    gamevariable::GameVariableManager,
};
use cobapi::{Event, SystemEvent};

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

pub fn create_variables() {
    GameVariableManager::make_entry(level::LEVEL_DIS_KEY, 0);
    GameVariableManager::make_entry(level::GROWTH_KEY, 0);
    GameVariableManager::make_entry(bgm::BGM_KEY, 0);
    GameVariableManager::make_entry(character::CHARACTER_KEY, 0);
    GameVariableManager::make_entry(rng::RNG_KEY, 0);
    GameVariableManager::make_entry(rng::SMASH_KEY, 0);
    GameVariableManager::make_entry(cook::COOK_KEY, 0);
    GameVariableManager::make_entry(gift::GIFT_KEY, 0);
    GameVariableManager::make_entry(exp::EXP_KEY, 0);
    GameVariableManager::make_entry(cutscene::CUTSCENES_KEY, 0);
    GameVariableManager::make_entry(arena::ARENA_KEY, 0);
    GameVariableManager::make_entry(support::SUPPORT_KEY, 0);
    GameVariableManager::make_entry(map::MAP_KEY, 0);
    character::get_lueur_gender();
    character::change_characters();
}
pub fn patch_all(){
    gift::patch_gift();
    rng::patch_rng();
    support::patch_support();
    cutscene::patch_cutscenes();
    map::patch_map();
    rng::patch_smash();
    arena::patch_arena();
    cook::patch_cook();
    bgm::patch_imm_bgm();
    level::patch_growth();
}
extern "C" fn create_settings(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::ProcInstJump {proc, label } => {
                if proc.hashcode == -1912552174 && *label == 28 {
                    create_variables();
                    patch_all();
                }
                if proc.hashcode == -1118443598 && *label == 0 {
                    support::update_reliances();
                }
            }
            _ => {},
        }
    } 
    else {  println!("We received a missing event, and we don't care!"); }
}

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
    pub mode: i32,
    pub m_info: &'static BattleInfo,

}

#[skyline::from_offset(0x01e7f750)]
pub fn battle_info_get_unit(this: &BattleInfo, index: i32,  method_info: OptionalMethod) -> &Unit; 

#[skyline::hook(offset=0x02470d60)]
pub fn calc_action_hook(this: &BattleCalculator, side_type: i32, method_info: OptionalMethod) -> bool {
    unsafe {
        if GameVariableManager::get_number(rng::RNG_KEY) == 4 { 
            Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]).unwrap();
            Patch::in_text(0x02375504).bytes(&[0x00, 0x08, 0x22, 0x1e]).unwrap();
            Patch::in_text(0x02375508).bytes(&[0x81, 0x01, 0x22, 0x1e]).unwrap();
            let unit = battle_info_get_unit(this.m_info, side_type, method_info);
            if unit.force.is_some() {
                if unit.person.get_asset_force() == 0 || unit.force.unwrap().force_type == 0 { 
                    Patch::in_text(0x01e8d12c).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
                    Patch::in_text(0x02375508).bytes(&[0x01,0x10, 0x2E, 0x1E]).unwrap();
                }
                else { 
                    Patch::in_text(0x01e8d12c).bytes(&[0xE0,0xE1, 0x84, 0x52]).unwrap();
                    Patch::in_text(0x02375504).bytes(&[0x6C,0x0C, 0x80, 0x52]).unwrap();
                }
            }
        }
    }
    let result = call_original!(this, side_type, method_info);
    Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]).unwrap();
    Patch::in_text(0x02375504).bytes(&[0x00, 0x08, 0x22, 0x1e]).unwrap();
    Patch::in_text(0x02375508).bytes(&[0x81, 0x01, 0x22, 0x1e]).unwrap();
    return result;
}
#[skyline::hook(offset = 0x2281a80)]
pub fn load_settings1(this: u64, stream: u64, method_info: OptionalMethod) -> bool {
    crate::hub::add_to_juke_box();
    let value: bool = call_original!(this, stream, method_info);
    if value {
        create_variables();
        patch_all();
    }
    return value;
}
pub fn return_true(address: usize){
    Patch::in_text(address).bytes(&[0x20,0x00, 0x80, 0x52]).unwrap();
    Patch::in_text(address+0x4).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
}

#[skyline::main(name = "libtriabolical")]
pub fn main() {
    //Enables support/bond viewing in maps and exploration
    let replace = &[0x1f, 0x25, 0x00, 0x71];
    Patch::in_text(0x0209950C).bytes(replace).unwrap();
    Patch::in_text(0x020994E0).bytes(replace).unwrap();
    Patch::in_text(0x02099538).bytes(replace).unwrap();
    Patch::in_text(0x01a2a7c0).bytes(&[0xe1,0x0e,0x80,0x12]).unwrap();
    Patch::in_text(0x01a2a7c4).bytes(&[0x02,0x0f,0x80,0x52]).unwrap();

    Patch::in_text(0x01fdea34).bytes(&[0x01,0x04,0x80, 0x52]).unwrap();

    // Lunatic Random 
    return_true(0x01bfaec0);
    // Enable Growth Select
    return_true(0x01bfae40);

    //DLC
    //return_true(0x029f4270);
    cobapi::register_system_event_handler(create_settings);
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
    cobapi::register_system_event_handler(create_settings);
    skyline::install_hooks!( string::help_param_setter_set_person, character::set_tip_text, string::help_param_setter_set_god, string::help_param_setter_set_item_data_hook, bgm::change_bgm_hook, level::level_up_window_setup_hook);
    skyline::install_hooks!(calc_action_hook);
    skyline::install_hooks!( menus::gmap_menu_sub_shop_menu_create_bind,menus::notice_board_create_bind, menus::hub_menu_create_bind);
    skyline::install_hooks!( exp::set_battle_info_hook, load_settings1, arena::arena_finish_training );
    skyline::install_hooks!(support::reliance_can_level_up, level::set_total_level, level::unit_info_set_level_hook, exp::exp_sequence_create_bind, exp::level_up_prepare_hook, exp::unit_add_exp);
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
