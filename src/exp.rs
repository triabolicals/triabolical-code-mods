use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gamedata::unit::Unit;
use engage::force::*;
use engage::gamedata::JobData;
use engage::gameuserdata::GameUserData;
pub const EXP_KEY: &str = "G_EXP_TYPE";

pub struct ExpMod;
impl ConfigBasicMenuItemSwitchMethods for ExpMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry_norewind(EXP_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(EXP_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 4, 1);
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
        if typeC == 0 {this.help_text = format!("Exp gain is set to normal.").into(); }
        else if typeC == 1 {this.help_text = format!("Exp gain is uncapped and scaled towards the player's average level.").into(); }
        else if typeC == 2 {this.help_text = format!("Units will not gain Exp.").into(); }
        else if typeC == 3 {this.help_text = format!("Units can gain more than 100 Exp per action.").into(); }
        else if typeC == 4 {this.help_text = format!("Units will gain a random amount of Exp per action").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(EXP_KEY);
        if type_C == 1 { this.command_text = format!("Rubberband").into(); }
        else if type_C == 0 { this.command_text = format!("Default").into(); }
        else if type_C == 2 {this.command_text = format!("None").into(); }
        else if type_C == 3 {this.command_text = format!("Uncapped").into(); }
        else if type_C == 4 {this.command_text = format!("Random").into(); }
    }
}
// Structures required to change exp and level ups
#[unity::class("App", "UnitGrowSequence")]
pub struct UnitGrowSequence {
    proc: [u8; 96],
    m_CameraMode: i32,
    m_unit: &'static Unit,
    m_Exp: i32,
    m_OldLevel: i32,
    m_IsTalk: bool,
    m_SkillPoint: i32,
}

#[unity::class("App", "LevelUpSequnece")]
pub struct LevelUpSequnece {
    proc: [u8; 96],
    ResNameLevelUp: &'static Il2CppString,
    ResNameClassChange: &'static Il2CppString,
    m_unit: &'static Unit,
    m_grow: &'static Unit,
    m_level: i32,
    m_isClassChange: bool,
}

// Random Functions
#[unity::class("App", "Random")]
pub struct Random {}

#[unity::from_offset("App", "Random", "get_Game")]
pub fn random_get_Game(method_info: OptionalMethod) -> &'static Random;

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
    else { //random
        unsafe {
            let rng = random_get_Game(None);
            let new_exp: i32 = 5 + random_getMinMax(rng, 0, 95, None);
            return new_exp;
        }
    }
}

//Rubberbanding function
#[skyline::hook(offset = 0x01f7f360)]
pub fn addExp(this: &mut UnitGrowSequence, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_number(EXP_KEY);
    if typeC != 1 {
        call_original!(this, method_info); 
        return;
    }
    let sp = this.m_SkillPoint;
    let exp = this.m_Exp;
    let unit = this.m_unit;
    let diff =  GameUserData::get_difficulty(false);
    let total_level: i32 = (unit.m_Level + unit.m_InternalLevel) as i32;
    unsafe { 
        let player_average = GetAverageLevel(2, 8, None) - 2 - diff;
        if player_average < 2 { call_original!(this, method_info);  } 
        else if player_average < total_level{ call_original!(this, method_info);   }
        else {
            let mut factor = player_average - total_level;
            if factor < 1 {factor = 1; }
            this.m_SkillPoint = factor*sp;
            this.m_Exp = factor*exp;
            call_original!(this, method_info); 
        }
    }
}
//Function that does the level up and hooked it to do random level up type on 'Random'
#[skyline::hook(offset=0x01a3a040)]
pub fn Unit_LevelUP(this: &Unit, abort: i32, method_info: OptionalMethod){
    let growthType = GameVariableManager::get_number("G_GROWTH_TYPE");
    if growthType == 4 {
        unsafe { Unit_LevelDown(this, method_info); }
        return;
    }
    let typeC =  GameVariableManager::get_number(EXP_KEY);
    if typeC == 4{
        unsafe {
            let rng = random_get_Game(None);
            let s = random_getMinMax(rng, 0, 2, None);
            if s == 1 { call_original!(this, abort, method_info); }
            else if s == 2{ Unit_LevelDown(this, method_info); }
        }
    }
    else { call_original!(this, abort, method_info); }
}
//Function that prepares the window for level up. using this to call the level up function multiple times for multiple level up
#[skyline::hook(offset=0x01be1260)]
pub fn LevelUp_Prepare(this: &LevelUpSequnece, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_number(EXP_KEY);
    if typeC == 0 || typeC == 2 {
        call_original!(this, method_info);
        return;
    }
    call_original!(this, method_info);
    //println!("Level Up Prepare: unit lvl {}, grow level {}, level {}", this.m_unit.m_Level, this.m_grow.m_Level, this.m_level);
    if this.m_isClassChange {return; } //also used for class change, so return if class changed

    // one level up already happened
    let nLevelUps = (this.m_unit.m_Level as i32) - this.m_level - 1;
    if nLevelUps < 1 { return; }
    for x in 0..nLevelUps { unsafe { Unit_LevelUP(this.m_grow, 2, None); } }
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


extern "C" fn EXP() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<ExpMod>("Exp Mode") }

pub fn exp_install(){ cobapi::install_game_setting(EXP); }
