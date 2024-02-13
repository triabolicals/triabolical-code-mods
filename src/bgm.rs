use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use crate::string::*;
pub const BGM_KEY: &str = "G_BGM";



#[unity::class("App", "FieldBgmManager")]
pub struct FieldBgmManager {
    junk: [u64; 13],
    pub m_BattleBgmContinueTurn: i32,
}
//Force BGM to player phase
#[skyline::hook(offset=0x02d56700)]
pub fn ChangeBGM(this: u64, forceType: i32, proc: u64, isTurn: bool, method_info: OptionalMethod) {
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { 
        // to not count non-player phase changes against the special bgm turn count
        if forceType == 0 { call_original!(this, 0, proc, isTurn, method_info); }
        else { call_original!(this, 0, proc, false, method_info);  }

    }
    else { call_original!(this, forceType, proc, isTurn,method_info) }
}
//For Time Crystals Phase Change
#[skyline::hook(offset=0x02d56930)]
pub fn ChangeBGM2(this: u64, forceType: i32, method_info: OptionalMethod) {
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { call_original!(this, 0,  method_info) }
    else { call_original!(this, forceType, method_info) }
}
pub struct bgmmod;
impl ConfigBasicMenuItemSwitchMethods for bgmmod {
    fn init_content(this: &mut ConfigBasicMenuItem){ }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(BGM_KEY, 0); 
        let toggle =  GameVariableManager::get_bool(BGM_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(BGM_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(BGM_KEY) == 1 { this.help_text = "Player phase BGM overrides other phase BGM.".into(); } 
        else { this.help_text = "Default BGM Setting".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(BGM_KEY) == 0 { this.command_text = On_str();} 
        else { this.command_text = Off_str(); }
    }
}
extern "C" fn bgm_switch() -> &'static mut ConfigBasicMenuItem { 
    let str0 = get_mess_str("MID_CONFIG_BGM_CHANGE_ENEMYTURN");
    ConfigBasicMenuItem::new_switch::<bgmmod>(str0.get_string().unwrap())
 }
pub fn bgm_install(){ cobapi::install_game_setting(bgm_switch); }