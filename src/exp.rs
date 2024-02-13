use skyline::patching::Patch;
use unity::prelude::*;
use std::io::Write;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gamedata::unit::Unit;
use engage::gamedata::CapabilitySbyte2;
use engage::force::*;
use crate::string::*;
use crate::rng::*;
use engage::gamedata::JobData;
use engage::gameuserdata::GameUserData;
pub const EXP_KEY: &str = "G_EXP_TYPE";
pub struct ExpMod;
impl ConfigBasicMenuItemSwitchMethods for ExpMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry_norewind(EXP_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(EXP_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 6, 1);
        if toggle != result {
            GameVariableManager::set_number(EXP_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(EXP_KEY);
        if typeC == 0 {this.help_text = "Exp gain is set to normal.".into(); }
        else if typeC == 1 {this.help_text = "Exp is uncapped and scaled towards the player's average level.".into(); }
        else if typeC == 2 {this.help_text = "Units will not gain Exp.".into(); }
        else if typeC == 3 {this.help_text = "Units can gain more than 100 Exp.".into(); }
        else if typeC >= 4 {this.help_text = "???".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(EXP_KEY);
        if type_C == 1 { this.command_text = "Rubberband".into(); }
        else if type_C == 0 { this.command_text = "Default".into(); }
        else if type_C == 2 {this.command_text = "No Exp".into(); }
        else if type_C == 3 {this.command_text = "Uncapped".into(); }
        else if type_C == 4 {this.command_text = "??".into(); }
        else if type_C == 5 {this.command_text = "???".into(); }
        else if type_C == 6 {this.command_text = "????".into(); }
    }
}
// Structures required to change exp and level ups
#[unity::class("App", "UnitGrowSequence")]
pub struct UnitGrowSequence {
    proc: [u8; 96],
    m_CameraMode: i32,
    pub m_unit: &'static Unit,
    pub m_Exp: i32,
    pub m_OldLevel: i32,
    pub m_IsTalk: bool,
    pub m_SkillPoint: i32,
}

#[unity::class("App", "LevelUpSequnece")]
pub struct LevelUpSequnece {
    proc: [u8; 96],
    ResNameLevelUp: &'static Il2CppString,
    ResNameClassChange: &'static Il2CppString,
    pub m_unit: &'static mut Unit,
    pub m_grow: &'static mut Unit,
    pub  m_level: i32,
    m_isClassChange: bool,
}

// Random Functions





#[skyline::from_offset(0x023751b0)]
pub fn random_getMinMax(this: &Random, min: i32, max: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x2b4afa0)]
pub fn GetAverageLevel(difficulty: i32, sortieCount: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01a3aba0)]
pub fn Unit_LevelDown(this: &Unit, method_info: OptionalMethod);

//hook to the function that normalized exp to 100 to remove the cap if rubberband mode, or random exp
#[skyline::hook(offset = 0x01a39f60)]
pub fn normalizeExp(this: &Unit, exp: i32, method_info: OptionalMethod) -> i32 {
    let typeC = GameVariableManager::get_number(EXP_KEY);
    if typeC == 0 { return call_original!(this, exp, method_info); } //default
    else if typeC == 1 || typeC == 3 { return exp; } //uncapped
    else if typeC == 2 { return 0; } //No exp
    else if typeC == 5 { return 999; }
    else if typeC == 6 { return 9999; }
    else { 
        unsafe {
            let luck_stat = unit_get_capability(this, 4, true, None);

            let limit: i32 = luck_stat / 10 + 1;
            let factor = random_getMinMax(random_get_Game(None), 0, limit, None);

            return exp + (luck_stat + 1)*factor;
        }
    }
}


pub fn deactivate_exp_bar(deactivate: bool) {
    if deactivate { Patch::in_text(0x01f7f3bc).nop(); }
    else { Patch::in_text(0x01f7f3bc).bytes(&[0x19,0xa6,0x15,0x14]); }
}

//Rubberbanding function and disables the exp bar if it doesn't do anything
#[skyline::hook(offset = 0x01f7f360)]
pub fn addExp(this: &mut UnitGrowSequence, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_number(EXP_KEY);
    deactivate_exp_bar( typeC == 2  );
    if typeC != 1 {
        if typeC == 4 {  this.m_SkillPoint = this.m_Exp;}
        deactivate_exp_bar( this.m_SkillPoint == 0 && ( this.m_Exp == 0 || this.m_unit.m_Job.MaxLevel == this.m_unit.m_Level)  );
        call_original!(this, method_info); 
        return;
    }
    if this.m_unit.m_Job.MaxLevel == this.m_unit.m_Level && this.m_SkillPoint == 0 {
        deactivate_exp_bar( true  );
        call_original!(this, method_info);
    }
    let sp = this.m_SkillPoint;
    let exp = this.m_Exp;
    let unit = this.m_unit;
    let diff =  GameUserData::get_difficulty(false);
    let total_level: i32 = (unit.m_Level as i32) + (unit.m_InternalLevel as i32);
    unsafe { 
        let player_average = GetAverageLevel(2, 8, None) - 2*diff - 2;
        if player_average < 2 { call_original!(this, method_info);  } 
        else if player_average < total_level{ call_original!(this, method_info);   }
        else {
            let mut factor = player_average - total_level;
            if factor < 1 {factor = 1; }
            this.m_SkillPoint = factor*sp;
            this.m_Exp = factor*exp;
            deactivate_exp_bar( this.m_SkillPoint == 0 && this.m_Exp == 0  );
            call_original!(this, method_info); 
        }
    }
}
#[unity::from_offset("App", "Unit", "GetCapability")]
pub fn unit_get_capability(this: &Unit, type_: i32, calcEnhance: bool, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "SetBaseCapability")]
pub fn unit_set_base_capability(this: &Unit, index: i32, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01a2ff20)]
pub fn unit_get_capability_grow(this: &Unit, index: i32, is_autoGrow: bool, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x02054ae0)]
pub fn base_job(this: &JobData, method_info: OptionalMethod) -> &CapabilitySbyte2;
//  HP  Str Dex Spd Luck Def Mag Mdef Phys Sight Move
#[skyline::from_offset(0x01a54d60)]
pub fn get_HP(this: &Unit, method_info: OptionalMethod) -> i32;
//To Display Growth Rates on help stat text
#[unity::hook("App", "HelpParamSetter", "SetBattleInfo")]
pub fn SetBattleInfo(this: u64, engine: u64, unit: &Unit, typ_e: i32, text: &Il2CppString, method_info: OptionalMethod){
    unsafe {
    let mut grow: i32 = 0;
    let mut grow2: i32 = 0;
    let mut growth_rate_add = false;
    if unit.m_Force.is_none() {
        call_original!(this, engine, unit, typ_e, text, method_info); 
        return;
    }
    let force = unit.m_Force.unwrap().m_Type;
    match typ_e {
        0 => {
            grow = unit_get_capability_grow(unit, 0, false, None);
            grow2 = unit_get_capability_grow(unit, 0, true, None);
            growth_rate_add = true;
        },
        8 => {
            grow = unit_get_capability_grow(unit, 1, false, None);
            grow2 = unit_get_capability_grow(unit, 1, true, None);
            growth_rate_add = true;
        },
        10 => {
            grow = unit_get_capability_grow(unit, 2, false, None);
            grow2 = unit_get_capability_grow(unit, 2, true, None);
            growth_rate_add = true;
        },
        7 => {
            grow = unit_get_capability_grow(unit, 3, false, None);
            grow2 = unit_get_capability_grow(unit, 3, true, None);
            growth_rate_add = true;
        },
        14 => {
            grow = unit_get_capability_grow(unit, 4, false, None);
            grow2 = unit_get_capability_grow(unit, 4, true, None);
            growth_rate_add = true;
        },
        12 => {
            grow = unit_get_capability_grow(unit, 5, false, None);
            grow2 = unit_get_capability_grow(unit, 5, true, None);
            growth_rate_add = true;
        },
        9 => {
            grow = unit_get_capability_grow(unit, 6, false, None);
            grow2 = unit_get_capability_grow(unit, 6, true, None);
            growth_rate_add = true;
        },
        13 => {
            grow = unit_get_capability_grow(unit, 7, false, None);
            grow2 = unit_get_capability_grow(unit, 7, true, None);
            growth_rate_add = true;
        },
        15 => {
            grow = unit_get_capability_grow(unit, 8, false, None);
            grow2 = unit_get_capability_grow(unit, 8, true, None);
            growth_rate_add = true;
        },
        16 => {
            grow = unit_get_capability_grow(unit, 10, false, None);
            grow2 = unit_get_capability_grow(unit, 10, true, None);
            growth_rate_add = true;
        }
        _ => { growth_rate_add = false; }
    }
    let mut OKO: &Il2CppString = "".into();
    let mut growth_str: &Il2CppString = "".into();
    if typ_e == 0 && force < 3 {
        let hp = get_HP(unit, None);
        let posion = get_position_stack(unit);
        let phys = hp + unit_get_capability(unit, 5, true, None) - posion;
        let phys2 = phys + unit_get_capability(unit, 5, true, None) - posion;
        let phys4 = phys + 3*unit_get_capability(unit, 5, true, None) - 3*posion;

        let phys_str: &Il2CppString = format!("\n{} {}: {} / {} / {}", get_mess_str("MID_SYS_Weapon_Attack").get_string().unwrap(), get_mess_str("MID_SYS_Dmg").get_string().unwrap(), 
        phys, phys2 / 2 + (phys2 % 2).signum(), phys4 / 4 + (phys4 % 4).signum()).into();

        let mag = hp + unit_get_capability(unit, 7, true, None) - posion;
        let mag2 = mag + unit_get_capability(unit, 7, true, None) - posion;
        let mag4 = mag + 3*unit_get_capability(unit, 7, true, None) - 3*posion;

        let mag_str: &Il2CppString = format!("\n{} {}: {} / {} / {}", get_mess_str("MID_SYS_Magic_Attack").get_string().unwrap(), get_mess_str("MID_SYS_Dmg").get_string().unwrap(), 
        mag, mag2 / 2 + (mag2 % 2).signum(), mag4 / 4 + (mag4 % 4).signum()).into();

        OKO = concat_strings(phys_str, mag_str, None);
    }
    if (force != 1 && force != 2 ) && (growth_rate_add && grow > 0 ) {
        let mut level_str: &Il2CppString = format!(": {}%", grow).into();
        if grow2 != grow {
            if grow-grow2 < 0 { level_str = format!(": {}%", grow).into(); }
            else { level_str = format!(": {}% ({}% + {}%)", grow, grow2, grow-grow2).into(); }
        }
        growth_str = concat_strings3("\n".into(), Mess_Get("MID_GAMESTART_GROWMODE_SELECT_TITLE".into(), None) , level_str, None);
    }
    call_original!(this, engine, unit, typ_e, concat_strings3(text, OKO, growth_str, None), method_info);
}
}

//Function that does the level up and hooked it to do random level up type on 'Random'
pub fn move_level_up(this: &mut Unit, isNegative: bool){
    unsafe {
        let mut move_grow = unit_get_capability_grow(this, 10, false, None) + unit_get_capability_grow(this, 4, false, None) / 2; 
        let mut move_change = 0;
        let mut new_move_cap: i32 = 0;
        while move_grow > 99 {
            move_change += 1;
            move_grow -= 100;
        }
        let rng = random_getMinMax(random_get_Game(None), 0, 100, None);
        if  rng < move_grow {
            move_change += 1;
        }
        let old_cap: i32 = unit_get_capability(this, 10, false, None) - base_job(this.m_Job, None).array.m_item[10] as i32;
        if move_change == 0 { return; }
        if isNegative { new_move_cap = old_cap - move_change; }
        else { new_move_cap = old_cap + move_change; }
        unit_set_base_capability(this, 10, new_move_cap, None);

        let mut move_grow1 = unit_get_capability_grow(this, 9, false, None) + unit_get_capability_grow(this, 4, false, None) / 2; 
        let mut move_change1 = 0;
        let mut new_move_cap1: i32 = 0;
        while move_grow1 > 99 {
            move_change1 += 1;
            move_grow1 -= 100;
        }
        let rng1 = random_getMinMax(random_get_Game(None), 0, 100, None);
        if  rng1 < move_grow1 {
            move_change += 1;
        }
        let old_cap1: i32 = unit_get_capability(this, 9, false, None) - base_job(this.m_Job, None).array.m_item[9] as i32;
        if move_change1 == 0 { return; }
        if isNegative { new_move_cap1 = old_cap1 - move_change1; }
        else { new_move_cap1 = old_cap1 + move_change1; }
        unit_set_base_capability(this, 9, new_move_cap1, None);
    }
}



#[skyline::from_offset(0x01a3a040)]
pub fn Unit_LevelUP(this: &Unit, abort: i32, method_info: OptionalMethod);

//Function that prepares the window for level up. using this to call the level up function multiple times for multiple level up
#[skyline::hook(offset=0x01be1260)]
pub fn LevelUp_Prepare(this: &mut LevelUpSequnece, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_number(EXP_KEY);
    if typeC == 0 || typeC == 2 {
        call_original!(this, method_info);
        return;
    }
    call_original!(this, method_info);

    //println!("Level Up Prepare: unit lvl {}, grow level {}, level {}", this.m_unit.m_Level, this.m_grow.m_Level, this.m_level);
    if this.m_isClassChange {return; } //also used for class change, so return if class changed
    if typeC == 4 { move_level_up(this.m_grow, false); }
    // one level up already happened
    let nLevelUps = (this.m_unit.m_Level as i32) - this.m_level - 1;

    let mut count = 0;
    unsafe {
    if typeC == 4 {
        let luck = unit_get_capability(this.m_grow, 4, true, None);
        for x in 0..nLevelUps+1 {
            let rng = random_getMinMax(random_get_Game(None), 0, 100, None);
            if count == 0 {
                count += 1;
                if rng < luck { continue; }
                if rng < 100-luck {
                    Unit_LevelDown(this.m_grow, None);
                    move_level_up(this.m_grow, true);
                }
            }
            else {
                count += 1;
                 if rng < luck {
                    Unit_LevelUP(this.m_grow, 3, None);
                        move_level_up(this.m_grow, false);
                    }
                    else if rng < 2*luck {
                        Unit_LevelDown(this.m_grow, None);
                        move_level_up(this.m_grow, true);
                    }
                }
            }
        }
        else if nLevelUps < 1 { return; } 
        else { for x in 0..nLevelUps { Unit_LevelUP(this.m_grow, 3, None); } }
    }
}

//rewrite the function that adds exp to the unit to allow multiple levels
#[skyline::hook(offset=0x01a39d40)]
pub fn unit_add_exp(this: &mut Unit, exp: i32, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_number(EXP_KEY);
    if typeC == 0 || typeC == 2 {
        call_original!(this, exp, method_info); 
        return;
    }
    let job = &this.m_Job;
    let job_max_level = job.MaxLevel;
    let mut expPool = exp + (this.m_Exp as i32);
    let mut unit_level = this.m_Level;
    while 99 < expPool && unit_level < job_max_level {
        unit_level = unit_level + 1;
        expPool = expPool - 100;
    }
    this.m_Level = unit_level;
    if unit_level != job_max_level { this.m_Exp = expPool.try_into().unwrap(); }
    else { this.m_Exp = 0; }
}


extern "C" fn EXP() -> &'static mut ConfigBasicMenuItem { 
    let exp_label = setting_str("MID_SYS_Exp");
    ConfigBasicMenuItem::new_switch::<ExpMod>(exp_label.get_string().unwrap()) 
}

pub fn exp_install(){ cobapi::install_game_setting(EXP); }
