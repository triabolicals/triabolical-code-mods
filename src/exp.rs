use std::sync::Mutex;
use unity::prelude::*;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    random::*,
    gamevariable::*,
    mess::*,
    gamedata::unit::Unit,
};
use crate::{string::*, rng::*};
pub const EXP_KEY: &str = "G_EXP_TYPE";

pub static HYBRID_TABLE: Mutex<Vec<(i32, i32)>> = Mutex::new(Vec::new());

#[skyline::from_offset(0x01e8d200)]
fn get_hit_ratio_10000(ratio: i32, method_info: OptionalMethod) -> i32;

pub fn intialize_hybrid_table() {
    for x in 0..101 {
        let true_ratio = unsafe { get_hit_ratio_10000(x, None) };
        HYBRID_TABLE.lock().unwrap().push( (x, true_ratio/100) );
        //println!("Hit Rate: {}, True Hit: {}", x, true_ratio);
    }
}

fn get_hybrid_to_display(hybrid: i32) -> i32 {
    let table = HYBRID_TABLE.lock().unwrap();
    let found = table.iter().find(|&&x| x.1 == hybrid);
    if found.is_some() { found.unwrap().0 }
    else { hybrid }
}

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
            1 => {  "Units will gain 100 Exp per action." },
            2 => { "Units will not gain Exp." },
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
extern "C" fn exp_b_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult  {
    if GameVariableManager::get_number(EXP_KEY) == 3 {
        GameVariableManager::set_bool("勝利", true); 
    }
    return BasicMenuResult::new();
}


// Structures required to change exp and level ups
#[unity::class("App", "LevelUpSequnece")]
pub struct LevelUpSequnece {
    proc: [u8; 96],
    res_name_level_up: &'static Il2CppString,
    res_name_class_change: &'static Il2CppString,
    pub m_unit: &'static mut Unit,
    pub m_grow: &'static mut Unit,
    pub  m_level: i32,
    is_class_change: bool,
}

#[skyline::from_offset(0x2b4afa0)]
pub fn get_average_level(difficulty: i32, sortie_count: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01a39f60)]
pub fn unit_normalize_exp(this: &Unit, exp: i32, method_info: OptionalMethod) -> i32;

pub fn normalize_exp(this: &Unit, exp: i32) -> i32 {
    let exp_mode = GameVariableManager::get_number(EXP_KEY);
    match exp_mode {
        1 => { 0 }
        2 => { 0 }
        _ => {  unsafe { unit_normalize_exp(this, exp, None) } }
    }
}

//Rubberbanding function and disables the exp bar if it doesn't do anything
#[unity::hook("App","ExpSequence","CreateBind")]
pub fn exp_sequence_create_bind(proc: u64, unit: &Unit, exp: i32, skill_point: i32, method_info: OptionalMethod){
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
#[skyline::from_offset(0x01a54d60)]
pub fn get_hp(this: &Unit, method_info: OptionalMethod) -> i32;

pub fn copy_random(this: &mut Random, src: &Random){
    this.seed1 = src.seed1;
    this.seed2 = src.seed2;
    this.seed3 = src.seed3;
    this.seed4 = src.seed4;
}
//To Display Growth Rates on help stat text
#[unity::hook("App", "HelpParamSetter", "SetBattleInfo")]
pub fn set_battle_info_hook(this: u64, engine: u64, unit: &mut Unit, text_type: i32, text: &Il2CppString, method_info: OptionalMethod){
    unsafe {
        // The RN Queue in the hit help box
        call_original!(this, engine, unit, text_type, text, method_info); 
        if text_type == 3 && GameVariableManager::get_number(RNG_KEY) == 5 {
            let game_rng = Random::get_game();
            let rng = Random::instantiate().unwrap();
            let rng_hybrid = Random::instantiate().unwrap();
            copy_random(rng, game_rng);
            copy_random(rng_hybrid , game_rng);
            let mut rng_v: i32;
            let mut rng_h: i32;
            let mut rng_str = "1RN:\t".to_string();
            let mut rng_hybrid_str = "HRN:\t".to_string();
            for x in 0..25 {
                rng_v = rng.get_value(100000) / 1000;
                rng_h = rng_hybrid.get_value(10000) / 100;
                rng_hybrid_str = format!("{} {}", rng_hybrid_str, get_hybrid_to_display(rng_h)).to_string(); 
                rng_str = format!("{} {}", rng_str, rng_v).to_string(); 
            }

            let total = format!("\n{}\n{}", rng_str, rng_hybrid_str).to_string();
            let final_str = concat_strings(text, total.into(), None);
            call_original!(this, engine, unit, text_type, final_str, method_info);
            return;
        }
        else if text_type == 5 && GameVariableManager::get_number(RNG_KEY) == 5 {
            let game_rng = Random::get_game();
            game_rng.get_value(100);
            return;
        }
        if unit.force.is_none() { return; }
        let force = unit.force.unwrap().force_type;
        let stat_index = match text_type {
            0 => 0,
            8 => 1,
            10 => 2,
            7 => 3,
            14 => 4,
            12 => 5,
            9 => 6,
            13 => 7,
            15 => 8,
            _ => -1,
        };
        if stat_index == -1 { return; } // text is not a stat
        let next_level = predict_level_up(unit);
        let grow = unit.get_capability_grow(stat_index, false);   // total growth rate
        let grow2 = unit.get_capability_grow(stat_index, true);   // personal growth rate
        let mut oko: &Il2CppString = "".into();
        let mut growth_str: &Il2CppString = "".into();
        if text_type == 0 && force < 3 {
            let hp = unit.get_hp();
            let posion = get_position_stack(unit);
            let phys = hp + unit.get_capability(5, true) - posion;
            let phys2 = phys + unit.get_capability(5, true) - posion;
            let phys4 = phys + 3*unit.get_capability(5, true) - 3*posion;

            let phys_str: &Il2CppString = format!("\n{} {}: {} / {} / {}", Mess::get("MID_SYS_Weapon_Attack").to_string(), Mess::get("MID_SYS_Dmg").to_string(), 
            phys, phys2 / 2 + (phys2 % 2).signum(), phys4 / 4 + (phys4 % 4).signum()).into();

            let mag = hp + unit.get_capability(7, true) - posion;
            let mag2 = mag + unit.get_capability(7, true) - posion;
            let mag4 = mag + 3*unit.get_capability(7, true) - 3*posion;

            let mag_str: &Il2CppString = format!("\n{} {}: {} / {} / {}", Mess::get("MID_SYS_Magic_Attack").to_string(), Mess::get("MID_SYS_Dmg").to_string(), 
            mag, mag2 / 2 + (mag2 % 2).signum(), mag4 / 4 + (mag4 % 4).signum()).into();

            oko = concat_strings(phys_str, mag_str, None);
        }
        if ( force == 0 || force == 3 ) && grow > 0  {
            let mut level_str: &Il2CppString = format!(": {}%", grow).into();
            if grow2 != grow {
                if grow-grow2 < 0 { level_str = format!(": {}%", grow).into(); }
                else { level_str = format!(": {}% ({}% + {}%)", grow, grow2, grow-grow2).into(); }
            }
            growth_str = concat_strings3("\n".into(), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE") , level_str, None);
        }
        if next_level[ stat_index as usize] > 0 && GameVariableManager::get_number(crate::level::LEVEL_DIS_KEY) == 2 {
            growth_str = format!("{}\nNext Lvl: +{}", growth_str, next_level[ stat_index as usize]).into();
        }
        call_original!(this, engine, unit, text_type, concat_strings3(text, oko, growth_str, None), method_info);
    }
}

fn predict_level_up(unit: &mut Unit) -> [i8; 10] {
    let mut base_cap: [i8; 10] = [0; 10];
    let mut level_cap: [i8; 10] = [0; 10];
    let mut growth_cap: [u8; 10] = [0; 10];
    for x in 0..10 {
        level_cap[x] = unit.level_capability.capability[x];
        base_cap[x] = unit.base_capability.capability[x];
        growth_cap[x] = unit.grow_capability[x];
    }

    let mut out: [i8; 10] = [0; 10];
    let old_level = unit.level;
    unit.set_level(1);
    let seed = unit.grow_seed;
    unit.level_up(2);
    for x in 0..10 {
        out[x] = unit.base_capability.capability[x] - base_cap[x];
        unit.set_base_capability(x as i32, base_cap[x] as i32);
        unit.level_capability.capability[x] = level_cap[x];
        unit.grow_capability[x] = growth_cap[x];
    } 
    unit.set_level(old_level as i32);
    unit.grow_seed = seed;
    return out;
}


//rewrite the function that adds exp to the unit to allow multiple levels
#[skyline::hook(offset=0x01a39d40)]
pub fn unit_add_exp(this: &mut Unit, exp: i32, method_info: OptionalMethod){
    let exp_mode =  GameVariableManager::get_number(EXP_KEY);
    if exp_mode == 0 || exp_mode == 2 {
        call_original!(this, exp, method_info); 
        return;
    }
    let job = &this.job;
    let job_max_level = job.get_max_level();
    let mut exp_pool = exp + (this.exp as i32);
    let mut unit_level = this.level;
    let mut n_levels = 0;
    while 99 < exp_pool && unit_level < job_max_level {
        unit_level = unit_level + 1;
        exp_pool = exp_pool - 100;
        n_levels += 1;
    }
    //this.level = unit_level;
    if n_levels > 1 { 
        for _x in 0..n_levels-1 { this.level_up(3); } 
        this.level = unit_level;
    }
    if unit_level != job_max_level { this.exp = exp_pool.try_into().unwrap(); }
    else { this.exp = 0; }
}


extern "C" fn exp() -> &'static mut ConfigBasicMenuItem { 
    let exp_label = setting_str("MID_SYS_Exp");
    let switch = ConfigBasicMenuItem::new_switch::<ExpMod>(exp_label.to_string());
    switch.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = exp_b_call as _ );
    switch
}

pub fn exp_install(){ cobapi::install_game_setting(exp); }
