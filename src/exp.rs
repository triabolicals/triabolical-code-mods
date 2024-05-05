use unity::prelude::*;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gameuserdata::*,
    random::*,
    gamevariable::*,
    mess::*,
    gamedata::unit::Unit,
};
use crate::{string::*, rng::*};
pub const EXP_KEY: &str = "G_EXP_TYPE";

pub struct ExpMod;
impl ConfigBasicMenuItemSwitchMethods for ExpMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){  }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(EXP_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 6, 1);
        if toggle != result {
            GameVariableManager::set_number(EXP_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            if result == 6 { GameVariableManager::set_bool("勝利".into(), true); }
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let exp_mode =  GameVariableManager::get_number(EXP_KEY);
        if exp_mode == 0 {this.help_text = "Exp gain is set to normal.".into(); }
        else if exp_mode == 1 {this.help_text = "Exp is uncapped and scaled towards the player's average level.".into(); }
        else if exp_mode == 2 {this.help_text = "Units will not gain Exp.".into(); }
        else if exp_mode == 3 {this.help_text = "Units can gain more than 100 Exp.".into(); }
        else if exp_mode >= 4 {this.help_text = "???".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let exp_mode =  GameVariableManager::get_number(EXP_KEY);
        if exp_mode == 1 { this.command_text = "Rubberband".into(); }
        else if exp_mode == 0 { this.command_text = "Default".into(); }
        else if exp_mode == 2 {this.command_text = "No Exp".into(); }
        else if exp_mode == 3 {this.command_text = "Uncapped".into(); }
        else if exp_mode == 4 {this.command_text = "??".into(); }
        else if exp_mode == 5 {this.command_text = "???".into(); }
        else if exp_mode == 6 {this.command_text = "????".into(); }
    }
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
    if exp_mode == 0 { 
        unsafe { return unit_normalize_exp(this, exp, None); }
    }
    else if exp_mode == 1 || exp_mode == 3 { return exp; } //uncapped
    else if exp_mode == 2 { return 0; } //No exp
    else if exp_mode == 5 { return 999; }
    else if exp_mode == 6 { return 9999; }
    else { 
        let luck_stat = this.get_capability( 4, true);
        let limit: i32 = luck_stat / 10 + 1;
        let factor = Random::get_game().get_min_max(0, limit);
        return exp + (luck_stat + 1)*factor;
    }
}

//Rubberbanding function and disables the exp bar if it doesn't do anything
#[unity::hook("App","ExpSequence","CreateBind")]
pub fn exp_sequence_create_bind(proc: u64, unit: &Unit, exp: i32, skill_point: i32, method_info: OptionalMethod){
    let exp_mode = GameVariableManager::get_number(EXP_KEY);
    if exp_mode  == 2 { return; }   //No Exp  
    let new_exp = normalize_exp(unit, exp);

    if exp_mode != 1 {
        if skill_point == 0 && ( exp == 0 || unit.job.get_max_level() == unit.level)  { return;  }
        if exp_mode == 4 { call_original!(proc, unit, new_exp, new_exp, method_info); return; }
        else { call_original!(proc, unit, new_exp, skill_point, method_info); return; }
    }
    else {
        unsafe {
            if unit.job.get_max_level() == unit.level && skill_point == 0 { return; }
            let diff =  GameUserData::get_difficulty(false);
            let total_level: i32 = (unit.level as i32) + (unit.internal_level as i32);
            let player_average = get_average_level(2, 8, None) - 2*diff - 2;
            if player_average < 2 || player_average < total_level { 
                call_original!(proc, unit, new_exp, skill_point, method_info); 
                return; 
            }
            else {
                let mut factor = player_average - total_level;
                if factor < 1 {factor = 1; }
                let scaled_sp = factor*skill_point;
                let scaled_exp = factor*new_exp;
                if scaled_sp == 0 && scaled_exp == 0  { return; }
                else { call_original!(proc, unit, scaled_exp ,scaled_sp, method_info); }
            }
        }
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
pub fn set_battle_info_hook(this: u64, engine: u64, unit: &Unit, typ_e: i32, text: &Il2CppString, method_info: OptionalMethod){
    unsafe {
        // The RN Queue in the hit help box
    if typ_e == 3 && GameVariableManager::get_number(RNG_KEY) == 5 {
        let game_rng = Random::get_game();
        let rng = Random::instantiate().unwrap();
        copy_random(rng, game_rng);
        let mut rng_values: [i32; 40] = [0; 40];
        let mut rng_v: i32;
        let mut rng_str = String::new();
        for x in 0..40 {
            rng_v = rng.get_value(100);
            rng_values[x] = rng_v;
            if x == 20 || x == 0 { rng_str = format!("{}\n{}", rng_str, rng_v).to_string(); }
            else { rng_str = format!("{} {}", rng_str, rng_v).to_string(); }
        }
        let final_str = concat_strings(text, rng_str.into(), None);
        call_original!(this, engine, unit, typ_e, final_str, method_info);
        return;
    }
    let mut grow: i32 = 0;
    let mut grow2: i32 = 0;
    let growth_rate_add: bool;
    println!("Set Battle Info: {}", typ_e);
    if unit.force.is_none() {
        call_original!(this, engine, unit, typ_e, text, method_info); 
        return;
    }
    let force = unit.force.unwrap().force_type;
    match typ_e {
        0 => {
            grow = unit.get_capability_grow( 0, false);
            grow2 = unit.get_capability_grow( 0, true);
            growth_rate_add = true;
        },
        8 => {
            grow =unit.get_capability_grow( 1, false);
            grow2 =unit.get_capability_grow( 1, true);
            growth_rate_add = true;
        },
        10 => {
            grow =unit.get_capability_grow( 2, false);
            grow2 = unit.get_capability_grow( 2, true);
            growth_rate_add = true;
        },
        7 => {
            grow = unit.get_capability_grow( 3, false);
            grow2 = unit.get_capability_grow( 3, true);
            growth_rate_add = true;
        },
        14 => {
            grow = unit.get_capability_grow( 4, false);
            grow2 = unit.get_capability_grow( 4, true);
            growth_rate_add = true;
        },
        12 => {
            grow = unit.get_capability_grow( 5, false);
            grow2 = unit.get_capability_grow( 5, true);
            growth_rate_add = true;
        },
        9 => {
            grow = unit.get_capability_grow( 6, false);
            grow2 = unit.get_capability_grow( 6, true);
            growth_rate_add = true;
        },
        13 => {
            grow = unit.get_capability_grow( 7, false);
            grow2 = unit.get_capability_grow( 7, true);
            growth_rate_add = true;
        },
        15 => {
            grow = unit.get_capability_grow( 8, false);
            grow2 = unit.get_capability_grow( 8, true);
            growth_rate_add = true;
        },
        16 => {
            grow = unit.get_capability_grow( 10, false);
            grow2 = unit.get_capability_grow( 10, true);
            growth_rate_add = true;
        }
        _ => { growth_rate_add = false; }
    }
    let mut oko: &Il2CppString = "".into();
    let mut growth_str: &Il2CppString = "".into();
    if typ_e == 0 && force < 3 {
        let hp = unit.get_hp();
        let posion = get_position_stack(unit);
        let phys = hp + unit.get_capability(5, true) - posion;
        let phys2 = phys + unit.get_capability(5, true) - posion;
        let phys4 = phys + 3*unit.get_capability(5, true) - 3*posion;

        let phys_str: &Il2CppString = format!("\n{} {}: {} / {} / {}", Mess::get("MID_SYS_Weapon_Attack").get_string().unwrap(), Mess::get("MID_SYS_Dmg").get_string().unwrap(), 
        phys, phys2 / 2 + (phys2 % 2).signum(), phys4 / 4 + (phys4 % 4).signum()).into();

        let mag = hp + unit.get_capability(7, true) - posion;
        let mag2 = mag + unit.get_capability(7, true) - posion;
        let mag4 = mag + 3*unit.get_capability(7, true) - 3*posion;

        let mag_str: &Il2CppString = format!("\n{} {}: {} / {} / {}", Mess::get("MID_SYS_Magic_Attack").get_string().unwrap(), Mess::get("MID_SYS_Dmg").get_string().unwrap(), 
        mag, mag2 / 2 + (mag2 % 2).signum(), mag4 / 4 + (mag4 % 4).signum()).into();

        oko = concat_strings(phys_str, mag_str, None);
    }
    if (force != 1 && force != 2 ) && (growth_rate_add && grow > 0 ) {
        let mut level_str: &Il2CppString = format!(": {}%", grow).into();
        if grow2 != grow {
            if grow-grow2 < 0 { level_str = format!(": {}%", grow).into(); }
            else { level_str = format!(": {}% ({}% + {}%)", grow, grow2, grow-grow2).into(); }
        }
        growth_str = concat_strings3("\n".into(), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE") , level_str, None);
    }
    call_original!(this, engine, unit, typ_e, concat_strings3(text, oko, growth_str, None), method_info);
    }
}
#[skyline::from_offset(0x01a385c0)]
fn unit_get_index(this: &Unit, kind: i32, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit","get_Index")]
fn unit_get_indexx(this: &Unit, method_info: OptionalMethod) -> i32;
//Function that does the level up and hooked it to do random level up type on 'Random'
pub fn move_level_up(this: &mut Unit, is_negative: bool){
    let mut move_grow = this.get_capability_grow( 10, false) + this.get_capability_grow( 4, false) / 2; 
    let mut move_change = 0;
    let new_move_cap: i32;
    let job_base = this.job.get_base();
    while move_grow > 99 {
        move_change += 1;
        move_grow -= 100;
    }        
    let game_rng = Random::get_game();
    let rng = game_rng.get_min_max(0, 100);
    if  rng < move_grow {
        move_change += 1;
    }
    let old_cap: i32 = this.get_capability_grow(10, false) - job_base[10] as i32;

    if is_negative { new_move_cap = old_cap - move_change; }
    else { new_move_cap = old_cap + move_change; }
    this.set_base_capability( 10, new_move_cap );

    let mut sight_grow = this.get_capability_grow( 9, false) + this.get_capability_grow( 4, false) / 2; 
    let mut sight_change = 0;
    let new_sight_cap: i32;
    while sight_grow > 99 {
        sight_change  += 1;
        sight_grow -= 100;
    }
    let rng1 = game_rng.get_min_max(0, 100);
    if  rng1 < sight_grow {
        sight_change += 1;
    }
    let old_sight_cap: i32 = this.get_capability(9, false) - job_base[9] as i32;
    if is_negative { new_sight_cap  = old_sight_cap - sight_change ; }
    else { new_sight_cap = old_sight_cap + sight_change ; }
    this.set_base_capability( 9, new_sight_cap );
}

//Function that prepares the window for level up. using this to call the level up function multiple times for multiple level up
#[skyline::hook(offset=0x01be1260)]
pub fn level_up_prepare_hook(this: &mut LevelUpSequnece, method_info: OptionalMethod){
    let exp_mode =  GameVariableManager::get_number(EXP_KEY);
    if exp_mode == 0 || exp_mode == 2 {
        call_original!(this, method_info);
        return;
    }
    call_original!(this, method_info);

    //println!("Level Up Prepare: unit lvl {}, grow level {}, level {}", this.m_unit.m_Level, this.m_grow.m_Level, this.m_level);
    if this.is_class_change {return; } //also used for class change, so return if class changed
    if exp_mode == 4 { move_level_up(this.m_grow, false); }
    // one level up already happened
    let n_level_ups = (this.m_unit.level as i32) - this.m_level - 1;

    let mut count = 0;
    if exp_mode == 4 {
        let luck = this.m_grow.get_capability(4, true);
        let game_rng = Random::get_game();
        for _x in 0..n_level_ups+1 {
            let rng = game_rng.get_min_max(0, 100);
            if count == 0 {
                count += 1;
                if rng < luck { continue; }
                if rng < 100-luck {
                    this.m_grow.level_down();
                    move_level_up(this.m_grow, true);
                }
            }
            else {
                count += 1;
                if rng < luck {
                    this.m_grow.level_up(3);                        
                    move_level_up(this.m_grow, false);
                }
                else if rng < 2*luck {
                    this.m_grow.level_down();
                    move_level_up(this.m_grow, true);
                }
            }
        }
    }
    else if n_level_ups < 1 { return; } 
    else { for _x in 0..n_level_ups { this.m_grow.level_up(3); } }
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
    while 99 < exp_pool && unit_level < job_max_level {
        unit_level = unit_level + 1;
        exp_pool = exp_pool - 100;
    }
    this.level = unit_level;
    if unit_level != job_max_level { this.exp = exp_pool.try_into().unwrap(); }
    else { this.exp = 0; }
}


extern "C" fn exp() -> &'static mut ConfigBasicMenuItem { 
    let exp_label = setting_str("MID_SYS_Exp");
    ConfigBasicMenuItem::new_switch::<ExpMod>(exp_label.get_string().unwrap()) 
}

pub fn exp_install(){ cobapi::install_game_setting(exp); }
