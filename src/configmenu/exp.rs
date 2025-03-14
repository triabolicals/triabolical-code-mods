use unity::prelude::*;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamevariable::*,
    gamedata::unit::Unit,
};
use super::*;

pub struct ExpMod;
impl ConfigBasicMenuItemSwitchMethods for ExpMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){  }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(EXP_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(EXP_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = match  GameVariableManager::get_number(EXP_KEY) {
            0 => { "Exp gain is set to normal." },
            1 => {  "Units will level up after any exp gaining actions." },
            2 => { "Units will not gain Exp / SP." },
            _ => { "???"},
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(EXP_KEY) {
            1 => {  "Full Level" },
            2 => { "No Exp" },
            0 => { "Default" },
            _ => { "???"},
        }.into();
    }
}
extern "C" fn exp_b_call(_this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult  {
    if GameVariableManager::get_number(EXP_KEY) == 3 {GameVariableManager::set_bool("勝利", true);  }
    return BasicMenuResult::new();
}

extern "C" fn exp() -> &'static mut ConfigBasicMenuItem { 
    let exp_label = crate::utils::setting_str("MID_SYS_Exp");
    let switch = ConfigBasicMenuItem::new_switch::<ExpMod>(exp_label.to_string());
    switch.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = exp_b_call as _ );
    switch
}

pub fn exp_install(){ 
    println!("Exp Installed");
    cobapi::install_game_setting(exp); }

// Structures required to change exp and level ups

#[skyline::from_offset(0x01a39f60)]
pub fn unit_normalize_exp(this: &Unit, exp: i32, method_info: OptionalMethod) -> i32;

pub fn normalize_exp(this: &Unit, exp: i32) -> i32 {
    let exp_mode = GameVariableManager::get_number(EXP_KEY);
    match exp_mode {
        1|2 => { 0 }
        _ => {  unsafe { unit_normalize_exp(this, exp, None) } }
    }
}

//Rubberbanding function and disables the exp bar if it doesn't do anything
#[unity::hook("App","ExpSequence","CreateBind")]
pub fn exp_sequence_create_bind_hook(proc: u64, unit: &Unit, exp: i32, skill_point: i32, method_info: OptionalMethod){
    let exp_mode = GameVariableManager::get_number(EXP_KEY);
    if exp_mode == 1 {
        unit.level_up(2);
        return;
    }
    if exp_mode == 2 { return; }   //No Exp  
    let new_exp = normalize_exp(unit, exp);

    if exp_mode != 1 {
        if skill_point == 0 && ( exp == 0 || unit.job.get_max_level() == unit.level)  { return;  }
        if exp_mode == 4 { call_original!(proc, unit, new_exp, new_exp, method_info); return; }
        else { call_original!(proc, unit, new_exp, skill_point, method_info); return; }
    }
    else {
        if unit.job.get_max_level() == unit.level && skill_point == 0 { return; }
        call_original!(proc, unit, new_exp, skill_point , method_info); 
    }
}
//  HP  Str Dex Spd Luck Def Mag Mdef Phys Sight Move
