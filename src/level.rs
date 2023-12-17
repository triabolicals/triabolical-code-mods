use skyline::patching::Patch;
use unity::prelude::*;
use std::ops::Deref;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gameuserdata::GameUserData;
use engage::{gamedata::*, singleton::SingletonClass};
use engage::{gamedata::person::*, gamedata::unit::*};

// Level, Growth Mods
pub const LEVEL_DIS_KEY: &str = "G_LEVEL_TYPE";
pub const GROWTH_KEY: &str = "G_GROWTH_TYPE";

//Structure and functions to hook to Level display settings

#[unity::class("TMPro", "TextMeshProUGUI")]
pub struct TextMeshProUGUI {}

#[unity::class("App", "UnitStatusSetter")]
pub struct UnitStatusSetter {
    junk: [u8; 376],
    level: &'static UnitStatusSetter_ValueParam,
}

#[unity::class("App", "UnitStatusSetter_ValueParam")]
pub struct UnitStatusSetter_ValueParam {
    setter: &'static UnitStatusSetter,
    m_root_ptr: u64,
    m_title: &'static TextMeshProUGUI,
    m_value: &'static TextMeshProUGUI,
    //
}
#[unity::class("App", "UnitInfoParamSetter")]
pub struct UnitInfoParamSetter {
    junk : [u8; 160],
    m_level : &'static TextMeshProUGUI,
}

#[skyline::hook(offset = 0x1f9d320)]
pub fn UnitInfo_SetLevel(this: &UnitInfoParamSetter, unit: Option<&Unit>, x: i32, z: i32, bSelectedGod: bool, god: &GodUnit, method_info: OptionalMethod){
    call_original!(this, unit, x, z, bSelectedGod, god, method_info);
    match unit {
        Some(p) => {
            unsafe {
                GameVariableManager::make_entry_norewind(LEVEL_DIS_KEY, 0);
                let result = GameVariableManager::get_bool(LEVEL_DIS_KEY);
                let enLevel = unit_GetEnhancedLevel(p, None);
                let mut displayed_level = enLevel;
                if result { displayed_level = enLevel + (p.m_InternalLevel as i32); }
                TrySetText(this.m_level, displayed_level, None);
            }
        },
        None => {},
    }
}

#[skyline::from_offset(0x290f1c0)]
pub fn TrySetText(tmp: &TextMeshProUGUI, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x1b58360)]
pub fn SetValueDirect(this: &UnitStatusSetter_ValueParam, str: &Il2CppString, dir: i32, isLimit: bool, method_info: OptionalMethod);

//Hooking to where the game sets the level display in the unit status screen
#[skyline::hook(offset = 0x1c66980)]
pub fn Set__Level(this: &UnitStatusSetter, unit: &Unit, unit_no_enhance: &Unit, method_info: OptionalMethod){
    call_original!(this, unit, unit_no_enhance, method_info);
    GameVariableManager::make_entry_norewind(LEVEL_DIS_KEY, 0);
    let result = GameVariableManager::get_bool(LEVEL_DIS_KEY);

    unsafe {
        let enLevel = unit_GetEnhancedLevel(unit, None);
        let no_enLevel = unit_GetEnhancedLevel(unit_no_enhance, None);
        let unit_level = unit_no_enhance.m_Level;
        let max_level = jobdata_get_max_level(unit_no_enhance.m_Job, None);
        let boost: i32 = (no_enLevel < enLevel) as i32;
        let at_limit: bool = max_level <= unit_level;
        let displayed_level = enLevel;
        if result {
            let internal_level = unit_no_enhance.m_InternalLevel;
            if internal_level == 0{
                let level_str = format!("{}", displayed_level).into();
                SetValueDirect(this.level, level_str , boost, at_limit, None);
            }
            else {
                let level_str = format!("{}/{}", displayed_level, internal_level).into();
                SetValueDirect(this.level, level_str , boost, at_limit, None);
            }
        }
        else {
            let level_str = format!("{}", displayed_level).into();
            SetValueDirect(this.level, level_str , boost, at_limit, None);
        }
    }
}

// Growth mode default
fn restoreDefault(){
    //Growth Mode Call
    Patch::in_text(0x01a3a3c4).bytes(&[0xe7, 0x6a, 0x2b, 0x94]).unwrap();
    //Random
    Patch::in_text(0x01a3a658).bytes(&[0x14,0x81,0x40, 0x39]).unwrap();
    //Random RNG 
    Patch::in_text(0x01a3a73c).bytes(&[0x5d, 0xeb, 0x24, 0x94]).unwrap();
    //Fixed
    Patch::in_text(0x01a3a410).bytes(&[0x14,0x81, 0x40, 0x39]).unwrap();
    // Level Down but add the level instead of subtracting it
    Patch::in_text(0x01a3ac8c).bytes(&[0x08, 0x05, 0x0, 0x51]).unwrap();

}

pub fn patchGrowth(){
    GameVariableManager::make_entry_norewind(GROWTH_KEY, 0);
    let result =  GameVariableManager::get_number(GROWTH_KEY);
    restoreDefault();
    if (result == 0 ){ 
        println!("Growth set to save file default");
        restoreDefault(); 
    }
    else if (result == 1){
        //Opposite Mode
        let growthMode = GameUserData::get_grow_mode();
        if (growthMode == 0) {//Random -> Fixed
            Patch::in_text(0x01a3a3c4).bytes(&[0x20, 0x00, 0x80, 0xd2]).unwrap();
            println!("Growth set to 'Fixed' from save file default of 'Random'");
        }
        else { //Fixed -> Random
            Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
            println!("Growth set to 'Random' from save file default of 'Fixed'");
        }
    }
    else if (result == 2) {
        // No Growths
        Patch::in_text(0x01a3a410).bytes(&[0x14,0x00,0x80,0xD2]).unwrap();
        Patch::in_text(0x01a3a658).bytes(&[0x14,0x00, 0x80,0xD2]).unwrap();
        println!("Growth set to 'No Growths'");
    }
    else if (result == 3){
        // Perfect Level Ups, forcing to Random and RNG set to 1
        Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
        Patch::in_text(0x01a3a73c).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
        println!("Growth set to 'Perfect'");
    }
    else if result == 4 {
        //negative growths 
        restoreDefault(); 
        Patch::in_text(0x01a3ac8c).bytes(&[0x08, 0x05, 0x00, 0x11]).unwrap();
        println!("Growth set to 'Negative'");
    }
}
pub struct GrowthMod;
impl ConfigBasicMenuItemSwitchMethods for  GrowthMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ patchGrowth(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(GROWTH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 4, 1);

        if toggle != result {
            GameVariableManager::set_number(GROWTH_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchGrowth();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(GROWTH_KEY);
        let growthMode = GameUserData::get_grow_mode();
        if typeC == 0 {
            if (growthMode == 1) { this.help_text = format!("Default growth mode. (Default: Fixed)").into(); }
            else { this.help_text = format!("Default growth mode: (Default: Random)").into(); }
        }
        else if typeC == 1 {
            if (growthMode == 1) { this.help_text = format!("Switch growth mode. (Fixed to Random)").into(); }
            else { this.help_text = format!("Switch growth mode. (Random to Fixed)").into(); }
        }
        else if typeC == 2 { this.help_text = format!("Units will not gain stats on level ups.").into(); }
        else if typeC == 3 { this.help_text = format!("Units will gain perfect level ups.").into();  }
        else if typeC == 4 {this.help_text = format!("Units will lose stats on level up.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(GROWTH_KEY);
        let growthMode = GameUserData::get_grow_mode();
        if type_C == 0 {
            if (growthMode == 1) { this.command_text = format!("Default: Fixed").into(); }
            else { this.command_text =format!("Default: Random").into(); }
        }
        else if type_C == 1 { 
            if (growthMode == 0) { this.command_text = format!("Switch: Fixed").into(); }
            else { this.command_text =format!("Switch: Random").into(); } 
        }
        else if type_C == 2 { this.command_text = format!("No Growths").into(); }
        else if type_C == 3 { this.command_text = format!("Perfect Growths").into();  }
        else if type_C == 4 { this.command_text = format!("Negative Growths").into(); }
    }
}
pub struct LevelMod;
impl ConfigBasicMenuItemSwitchMethods for LevelMod {
    fn init_content(this: &mut ConfigBasicMenuItem){  }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(LEVEL_DIS_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        if (toggle) { this.help_text = format!("Displays unit's total level. (Internal + Displayed Level)").into(); } 
        else { this.help_text = format!("Default level display. (Displayed Level)").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        if (toggle){ this.command_text = format!("Total Level").into();} 
        else { this.command_text = format!("Default").into(); }
    }
}

#[no_mangle]
extern "C" fn level_callback() -> &'static mut ConfigBasicMenuItem {  ConfigBasicMenuItem::new_switch::<LevelMod>("Unit Level Display")}
#[no_mangle]
extern "C" fn growth_callback() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<GrowthMod>("Growth Mode")}
#[no_mangle]
pub fn level_install(){
    cobapi::install_game_setting(growth_callback);
    cobapi::install_game_setting(level_callback);
}