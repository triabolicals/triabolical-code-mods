use unity::prelude::*;
use unity::il2cpp::object::Array;
pub use concat_string::concat_string;
use engage::{    
    mess::*,
    gamevariable::*,
    tmpro::*,
    gamedata::{*, item::*, skill::*, unit::*},
};

pub mod ai;
pub mod battle;
pub mod deathlist;
pub mod jobgrow;
pub mod item;
pub mod level;

#[unity::class("App", "HelpParamSetter")]
pub struct HelpParamSetter {
    junk : [u8; 0x50],
    pub title_atk: &'static mut TextMeshProUGUI,
    pub value_atk: &'static mut TextMeshProUGUI,
    junk2 : [u64; 13],
    pub efficacy_none: &'static mut TextMeshProUGUI,
    pub efficacy_icons: &'static Array<u64>,
    pub title_weapon_level: &'static mut TextMeshProUGUI,
    pub value_weapon_level: &'static mut TextMeshProUGUI,
    junk3: u64,
    pub contexts_text: &'static mut TextMeshProUGUI,
}


pub fn help_menu_call_install(){ jobgrow::job_menu_calls_install(); }

pub fn install_help_hooks() {
    skyline::install_hooks!(
        help_param_setter_set_god,
        ai::help_param_setter_set_person,
        battle::set_battle_info_hook,
        deathlist::loading_set_tip_text_hook,
        item::help_param_setter_set_item_data_hook,
        jobgrow::class_change_set_job_details_hook,
        level::set_total_level,
        level::level_up_window_setup_hook,
        // level::unit_info_set_level_hook,
    );
    println!("Finished with installing Help Hook");
}

// Emblem Description to contain Link Emblem + Link Engage Attack
#[skyline::hook(offset=0x021621a0)]
pub fn help_param_setter_set_god(this: &mut HelpParamSetter, frame: u64, god: &GodData, method_info: OptionalMethod){
    call_original!(this, frame, god, method_info);
    let link_gid = god.get_link_gid();
    let link_engage_atk = god.get_engage_attack_link();
    if link_gid.is_none() || link_engage_atk.is_none() { return; }

    if let Some(link_god) = GodData::get(link_gid.unwrap()) {
        let god_name = Mess::get(link_god.mid);
        let eng_atk_name = Mess::get(SkillData::get(link_engage_atk.unwrap()).unwrap().name.expect("Linked Engage Attack has no name!"));
        let link_text = format!("\nLink: {}  ({}) ", eng_atk_name, god_name);
        let text = this.contexts_text.get_text();
        this.contexts_text.set_text(concat_string!(text.to_string(), link_text).into(), true);
    }
}