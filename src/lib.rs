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
mod ai;
mod accessory;

pub fn create_variables() {
    GameVariableManager::make_entry(level::LEVEL_DIS_KEY, 0);
    GameVariableManager::make_entry(level::GROWTH_KEY, 0);
    GameVariableManager::make_entry(bgm::BGM_KEY, 0);
    GameVariableManager::make_entry(rng::RNG_KEY, 0);
    GameVariableManager::make_entry(rng::SMASH_KEY, 0);
    GameVariableManager::make_entry(cook::COOK_KEY, 0);
    GameVariableManager::make_entry(gift::GIFT_KEY, 0);
    GameVariableManager::make_entry(exp::EXP_KEY, 0);
    GameVariableManager::make_entry(cutscene::CUTSCENES_KEY, 0);
    GameVariableManager::make_entry(arena::ARENA_KEY, 0);
    GameVariableManager::make_entry(support::SUPPORT_KEY, 0);
    GameVariableManager::make_entry(map::MAP_KEY, 0);
    GameVariableManager::make_entry("G_AI", 0);
    GameVariableManager::make_entry("G_Chef_Index", 0);
    GameVariableManager::make_entry(gift::DLC_COMPLETE, 0);
    GameVariableManager::make_entry("G_JobGrowth", 0);
    GameVariableManager::make_entry("G_DLC_Complete2", 0);
    //character::change_characters();
    println!("My Code Mods variables are created");
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
    ai::patch_ignorance();
}
extern "C" fn create_settings(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::ProcInstJump {proc, label } => {
                if proc.hashcode == -1912552174 && *label == 28 {
                    create_variables();
                    patch_all();
                }
                if proc.hashcode == -1118443598 {
                    create_variables();
                    support::update_reliances();
                    gift::complete_dlc_chapters();
                    map::clear_all_tutorials();
                    support::update_reliances();
                }
                if proc.hashcode == -988690862 && *label == 0 { 
                    crate::hub::add_to_juke_box();
                    hub::adjust_fish_data();
                    accessory::install_accessory_sub_menu();
                    crate::string::gather_effective_sids();
                    crate::string::add_x_call();
                }
                if proc.hashcode == -339912801 && *label == 2 {
                    accessory::install_accessory_sub_menu();
                }
            }
            _ => {},
        }
    } 
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
    if GameVariableManager::get_number(rng::RNG_KEY) == 4 { 
        Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]).unwrap();
        Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap();
        let unit =     unsafe { battle_info_get_unit(this.m_info, side_type, method_info) };
        if unit.force.is_some() {
            if unit.person.get_asset_force() == 0 || unit.force.unwrap().force_type == 0 { 
                Patch::in_text(0x01e8d12c).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();   // Hybrid Rng set to 1
                Patch::in_text(0x02375510).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();   // 1 RN Rng set to 1
            }
            else { 
                Patch::in_text(0x01e8d12c).bytes(&[0xE0,0xE1, 0x84, 0x52]).unwrap();    // Hybrid RNG set to 9999
                Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap();   // 1 RN RNG set to normal
            }
        }
        let result = call_original!(this, side_type, method_info);
        Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]).unwrap();
        Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap();
        return result;
    }
    call_original!(this, side_type, method_info)
}
#[skyline::hook(offset = 0x2281a80)]
pub fn load_settings1(this: u64, stream: u64, method_info: OptionalMethod) -> bool {
    let value: bool = call_original!(this, stream, method_info);
    if value {
        create_variables();
        patch_all();
    }
    return value;
}
pub fn return_true(address: usize){
   let _ = Patch::in_text(address).bytes(&[0x20,0x00, 0x80, 0x52]).unwrap();
   let _ = Patch::in_text(address+0x4).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
}
pub fn mov_1(address: usize){
    let _ = Patch::in_text(address).bytes(&[0x20,0x00, 0x80, 0x52]).unwrap();
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
    mov_1(0x02d55134);  // allows bgm to be set
    // Enable Growth Select
    return_true(0x01bfae40);
    // map level display is now cap at 99 instead of 20
    Patch::in_text(0x0252d124).bytes(&[0x60, 0x0c, 0x80, 0x52]).unwrap();

    //DLC heroes
    //mov_1(0x0253d7c0);
    //mov_1(0x0253d8b0);

    //Patch::in_text(0x01f05f20).bytes(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    //Patch::in_text(0x01f03e40).bytes(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    //Patch::in_text(0x01f0bf60).bytes(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    //return_true(0x029f4270);
    cobapi::register_system_event_handler(create_settings);
    bgm::bgm_install(); 
    cutscene::cutscene_install(); //Cutscenes
    support::support_install(); //Support
    map::map_mod_install(); // Map Dialogue/Tutorials
    arena::arena_install(); //Arena Skip
    ai::ai_install();   // AI + Support Info
    cook::cook_install(); // Cooking Outcomes
    //Exp mode
    level::level_install(); //Level Display and Growth Type
    //string::job_growths_install();
    rng::rng_install(); // RNG 
    //character::char_install();  // Single Character Mode
    gift::gift_install(); //Gift Settings
    gift::dlc_map_install();
    exp::exp_install(); 

    skyline::install_hooks!( ai::help_param_setter_set_person, character::set_tip_text, string::help_param_setter_set_item_data_hook);
    skyline::install_hooks!(calc_action_hook, level::level_up_window_setup_hook);
    skyline::install_hooks!( string::help_param_setter_set_god, menus::gmap_menu_sub_shop_menu_create_bind, string::set_job_details );
    skyline::install_hooks!( menus::notice_board_create_bind, menus::hub_menu_create_bind);
    skyline::install_hooks!( exp::set_battle_info_hook, load_settings1, arena::arena_finish_training, rng::battle_math, rng::prob_100);
    skyline::install_hooks!( unit_item_use_hook, level::set_total_level, level::unit_info_set_level_hook, exp::exp_sequence_create_bind);
    println!("MyCode Mods are loaded");
    exp::intialize_hybrid_table();
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
            "My Code Mods plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );
        skyline::error::show_error(
            0,
            "My Code Mods has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
}

/*
#[unity::class("App", "ClassChange.ChangeJobData")]
pub struct ChangeJobData {
    pub job: &'static JobData,
    pub job_weapon_mask: &'static WeaponMask,
    pub original_job_weapon_mask: &'static WeaponMask,
    pub proof_type: i32, 
    __: i32,
    pub cost_level: &'static Il2CppString,
    pub is_enough_level: bool,
    pub junk: [u8; 7],
    pub cost_weapon_mask: &'static WeaponMask,
    pub equippable_weapon_mask: &'static WeaponMask,
    pub enough_item: bool,
    pub is_gender: bool,
    pub is_default_job: bool,
}
#[skyline::hook(offset=0x019c6700)]
pub fn check_unit_has_sid_jid(this: &mut ChangeJobData, unit: &Unit, method_info: OptionalMethod) -> bool {
    let job_sid: &Il2CppString = format!("SID_{}", this.job.jid.get_string().unwrap()).into();

    let result = call_original!(this, unit, method_info);   // Handles if the unit is able to re-class with seal count, weapon prof, etc

    // Handles promoted classes and default classes
    let person_job = unit.person.get_job().unwrap();
    if this.job.parent.index == person_job.parent.index {   // Same job, same index
        return result;
    }
    // if person's job can promote to selected job if in the class tree
    if this.job.is_high() && person_job.is_low() {
        let high_jobs = person_job.get_high_jobs();
        for x in 0..high_jobs.len() {
            if high_jobs[x].parent.index == this.job.parent.index {
                return result;
            }
        }
    }

    // if person's job is base and the select job is promoted, check if job is in the person's class tree
    if this.job.is_low() && person_job.is_high() {
        let low_jobs = person_job.get_low_jobs();
        for x in 0..low_jobs.len() {
            if low_jobs[x].parent.index == this.job.parent.index {
                return result;
            }
        }
    }
    // Get SID_JID_XXXXX from the current class 
    let job_sid: &Il2CppString = format!("SID_{}", this.job.jid.get_string().unwrap()).into();
   
   if SkillData::get(&job_sid.get_string().unwrap()).is_none() {   // the SID does not exists
        this.is_gender = false; // Hides the result
        return false;  // Unit cannot reclass 
   }
    // If unit does not have the Skill, hide the class
    if !(unit.has_private_skill(job_sid) || unit.has_sid(job_sid)) {
        println!("Unit does not have skill");
        this.is_gender = false; // Hides the result
        return false;  // Unit cannot reclass 
    }
    return true;
}
*/
use engage::gamedata::{item::*, unit::*, skill::*};

#[skyline::hook(offset=0x01a4d990)]
pub fn unit_item_use_hook(this: &Unit, item: &ItemData, method_info: OptionalMethod) {
    if item.usetype == 42 {
        let give_skills = item.get_give_skills();
        this.equip_skill_pool.skill_array_add(give_skills);
    }
        call_original!(this, item, method_info);
}
