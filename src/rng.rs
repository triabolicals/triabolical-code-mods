use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;
use engage::gameuserdata::GameUserData;

pub const RNG_KEY: &str = "G_RNG_TYPE";
pub const EXP_KEY: &str = "G_EXP_TYPE";
pub struct RNGMod;
pub fn patchRNG(){
    GameVariableManager::make_entry_norewind(RNG_KEY, 0);
    let result =  GameVariableManager::get_number(RNG_KEY);
    let replaceH = &[0x11, 0xa0, 0x13, 0x94];
    let replaceRN = &[0xe0, 0xd7, 0x9f, 0x1a];
    let replaceRig = &[0x20, 0x00, 0x80, 0x52];
    if (result == 0){
        Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        println!("RNG Mode set to None");
    }
    else if (result == 1){// 1 RN 
        Patch::in_text(0x02375510).bytes(replaceRig).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        println!("RNG Mode set to ignore 1RN");
    }
    else if (result == 2){ // Hybrid
        Patch::in_text(0x01e8d12c).bytes(replaceRig).unwrap();
        Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        println!("RNG Mode set to ignore Hybrid RN");
    }
    else if (result == 3){//1 RN + Hybrid 
        Patch::in_text(0x01e8d12c).bytes(replaceRig).unwrap();
        Patch::in_text(0x02375510).bytes(replaceRig).unwrap();
        println!("RNG Mode set to ignore 1RN and Hybrid RN");
    }
}

impl ConfigBasicMenuItemSwitchMethods for RNGMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchRNG();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(RNG_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);

        if toggle != result {
            GameVariableManager::set_number(RNG_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchRNG();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(RNG_KEY);
        if typeC == 0 {this.help_text = format!("Default RNG behavior.").into(); }
        else if typeC == 1 { this.help_text = format!("Disables normal RNG. (Crits, Skill Procs, Well, Cooking, etc.)").into(); }
        else if typeC == 2 { this.help_text = format!("Disables hybrid RNG. (Hit Rates)").into(); }
        else if typeC == 3 { this.help_text = format!("Disables normal and hybrid RNG. (No Randomness)").into();  }
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(RNG_KEY);
        if type_C == 0 {this.command_text = format!("None").into(); }
        else if type_C == 1 { this.command_text = format!("Rig Normal").into(); }
        else if type_C == 2 { this.command_text = format!("Rig Hybrid").into(); }
        else if type_C == 3 { this.command_text = format!("Rig Normal/Hybrid").into();  }
        else {this.help_text = format!("Unknown").into(); }
    }
}
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

pub struct ExpMod;
impl ConfigBasicMenuItemSwitchMethods for ExpMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        GameVariableManager::make_entry_norewind(EXP_KEY, 0);
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_bool(EXP_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(EXP_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_bool(EXP_KEY);
        if typeC == false {this.help_text = format!("Exp gain is set to normal.").into(); }
        else {this.help_text = format!("Exp gain is scaled towards the player's average level.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_bool(EXP_KEY);
        if type_C { this.command_text = format!("Rubberband").into(); }
        else { this.command_text = format!("Default").into(); }
    }
}
#[skyline::from_offset(0x2b4afa0)]
pub fn GetAverageLevel(difficulty: i32, sortieCount: i32, method_info: OptionalMethod) -> i32;

#[skyline::hook(offset = 0x01a39f60)]
pub fn normalizeExp(this: &Unit, exp: i32, method_info: OptionalMethod) -> i32 {
    let typeC =  GameVariableManager::get_bool(EXP_KEY);
    if typeC == false {
        return call_original!(this, exp, method_info);
    }
    else { return exp; }
}

//Rubberbanding function
#[skyline::hook(offset = 0x01f7f360)]
pub fn addExp(this: &mut UnitGrowSequence, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_bool(EXP_KEY);
    if typeC == false {
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

//Function that does the level up
#[skyline::from_offset(0x01a3a040)]
pub fn Unit_LevelUP(this: &Unit, abort: i32, method_info: OptionalMethod);

//Function that prepares the window for level up. using this to call the level up function multiple times for multiple level up
#[skyline::hook(offset=0x01be1260)]
pub fn LevelUp_Prepare(this: &LevelUpSequnece, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_bool(EXP_KEY);
    if typeC == false {
        call_original!(this, method_info);
        return;
    }
    call_original!(this, method_info);
    println!("Level Up Prepare: unit lvl {}, grow level {}, level {}", this.m_unit.m_Level, this.m_grow.m_Level, this.m_level);
    if this.m_isClassChange {return; }
    let nLevelUps = (this.m_unit.m_Level as i32) - this.m_level - 1;
    if nLevelUps < 1 { return; }
    for x in 0..nLevelUps {
        unsafe { Unit_LevelUP(this.m_grow, 2, None); }
    }
}

//rewrite the function that adds exp to the unit to allow multiple levels
#[skyline::hook(offset=0x01a39d40)]
pub fn unit_add_exp(this: &mut Unit, exp: i32, method_info: OptionalMethod){
    let typeC =  GameVariableManager::get_bool(EXP_KEY);
    if typeC == false {
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

#[no_mangle]
extern "C" fn RNG() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<RNGMod>("RNG Rigging Mode") }
extern "C" fn EXP() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<ExpMod>("Exp Mode") }
pub fn rng_install(){ 
    cobapi::install_game_setting(RNG);
    cobapi::install_game_setting(EXP);
}