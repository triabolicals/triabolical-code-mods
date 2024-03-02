use skyline::patching::Patch;
use unity::prelude::*;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gameuserdata::*,
    random::*,
    gamevariable::*,
    force::*,
    gamedata::{*, unit::Unit, person::*, }
};
use crate::string::*;
use crate::rng::*;
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
            if result == 6 { GameVariableManager::set_bool("勝利".into(), true); }
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

#[skyline::from_offset(0x2b4afa0)]
pub fn GetAverageLevel(difficulty: i32, sortieCount: i32, method_info: OptionalMethod) -> i32;

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
        let luck_stat = this.get_capability( 4, true);
        let limit: i32 = luck_stat / 10 + 1;
        let factor = Random::get_game().get_min_max(0, limit);
        return exp + (luck_stat + 1)*factor;
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
        deactivate_exp_bar( this.m_SkillPoint == 0 && ( this.m_Exp == 0 || this.m_unit.m_Job.get_max_level() == this.m_unit.m_Level)  );
        call_original!(this, method_info); 
        return;
    }
    if this.m_unit.m_Job.get_max_level() == this.m_unit.m_Level && this.m_SkillPoint == 0 {
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
    let force = unit.m_Force.unwrap().force_type;
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
    let mut OKO: &Il2CppString = "".into();
    let mut growth_str: &Il2CppString = "".into();
    if typ_e == 0 && force < 3 {
        let hp = unit.get_hp();
        let posion = get_position_stack(unit);
        let phys = hp + unit.get_capability_grow(5, true) - posion;
        let phys2 = phys + unit.get_capability_grow(5, true) - posion;
        let phys4 = phys + 3*unit.get_capability_grow(5, true) - 3*posion;

        let phys_str: &Il2CppString = format!("\n{} {}: {} / {} / {}", get_mess_str("MID_SYS_Weapon_Attack").get_string().unwrap(), get_mess_str("MID_SYS_Dmg").get_string().unwrap(), 
        phys, phys2 / 2 + (phys2 % 2).signum(), phys4 / 4 + (phys4 % 4).signum()).into();

        let mag = hp + unit.get_capability_grow(7, true) - posion;
        let mag2 = mag + unit.get_capability_grow(7, true) - posion;
        let mag4 = mag + 3*unit.get_capability_grow(7, true) - 3*posion;

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
        let mut move_grow = this.get_capability_grow( 10, false) + this.get_capability_grow( 4, false) / 2; 
        let mut move_change = 0;
        let mut new_move_cap: i32 = 0;
        let job_base = this.m_Job.get_base();
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

        if isNegative { new_move_cap = old_cap - move_change; }
        else { new_move_cap = old_cap + move_change; }
        this.set_base_capability( 10, new_move_cap );

        let mut sight_grow = this.get_capability_grow( 9, false) + this.get_capability_grow( 4, false) / 2; 
        let mut sight_change = 0;
        let mut new_sight_cap: i32 = 0;
        while sight_grow > 99 {
            sight_change  += 1;
            sight_grow -= 100;
        }
        let rng1 = game_rng.get_min_max(0, 100);
        if  rng1 < sight_grow {
            sight_change += 1;
        }
        let old_sight_cap: i32 = this.get_capability(9, false) - job_base[9] as i32;
        if isNegative { new_sight_cap  = old_sight_cap - sight_change ; }
        else { new_sight_cap = old_sight_cap + sight_change ; }
        this.set_base_capability( 9, new_sight_cap );
    }
}

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
            let luck = this.m_grow.get_capability(4, true);
            let game_rng = Random::get_game();
            for x in 0..nLevelUps+1 {
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
        else if nLevelUps < 1 { return; } 
        else { for x in 0..nLevelUps { this.m_grow.level_up(3); } }
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
    let job_max_level = job.get_max_level();
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
