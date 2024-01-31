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
   // println!("Change force IMMM called with force {}", forceType);
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { call_original!(this, 0,  method_info) }
    else { call_original!(this, forceType, method_info) }
}

#[skyline::hook(offset=0x01dde130)]
pub fn FieldBgmSpecialTurn(turn :i32, method_info: OptionalMethod){
    call_original!(turn, method_info);
}
#[skyline::hook(offset=0x02d54fb0)]
pub fn startSpecialBattleBgmContinueTurn(this: &mut FieldBgmManager, method_info: OptionalMethod){
    println!("Start Field Bgm Continue Turn Called: {}", this.m_BattleBgmContinueTurn);
    this.m_BattleBgmContinueTurn = 1;
    call_original!(this, method_info);
}
#[skyline::hook(offset=0x0228d210)]
pub fn bgmHook(forceType: i32, _super: u64, method_info: OptionalMethod){
    if GameVariableManager::get_bool(BGM_KEY) {
        if forceType != 0 { return; }
    }
    call_original!(forceType, _super, method_info);
}

pub fn patch_bgm(){
    /* 
    if GameVariableManager::get_number(BGM_KEY) == 1 {
        Patch::in_text(0x0236c8bc).bytes(&[0xC0, 0x03, 0x5F, 0xD6]);
    }
    else {
        Patch::in_text(0x0236c8bc).bytes(&[0x55, 0x82, 0xFC, 0x17]);
    }
    
*/
}
pub struct bgmmod;
impl ConfigBasicMenuItemSwitchMethods for bgmmod {
    fn init_content(this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry(BGM_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(BGM_KEY, 0); 
        let toggle =  GameVariableManager::get_number(BGM_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 1, 1);
        if toggle != result {
            GameVariableManager::set_number(BGM_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_bgm();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(BGM_KEY) == 1 { this.help_text = format!("Player phase BGM overrides other phase BGM.").into(); } 
        else { this.help_text = format!("Default BGM Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(BGM_KEY);
        if GameVariableManager::get_number(BGM_KEY) == 1 { this.command_text = On_str();} 
        else { this.command_text = Off_str(); }
    }
}
extern "C" fn bgm_switch() -> &'static mut ConfigBasicMenuItem { 
    let str0 = get_mess_str("MID_CONFIG_BGM_CHANGE_ENEMYTURN");
    ConfigBasicMenuItem::new_switch::<bgmmod>(str0.get_string().unwrap())
 }
pub fn bgm_install(){ cobapi::install_game_setting(bgm_switch); }