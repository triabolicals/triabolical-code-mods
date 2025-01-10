use unity::prelude::*;
use skyline::patching::Patch;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamevariable::*,
    mess::*,
};
use engage::gamedata::dispos::ChapterData;
use engage::gameuserdata::GameUserData;
use crate::string::*;
pub const BGM_KEY: &str = "G_BGM";
//Force BGM to player phase
/* 
#[skyline::hook(offset=0x02d56700)]
pub fn change_bgm_hook(this: u64, force_type: i32, proc: u64, is_turn: bool, method_info: OptionalMethod) {
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { 
        // to not count non-player phase changes against the special bgm turn count
        if force_type == 0 { call_original!(this, 0, proc, is_turn, method_info); }
        else { call_original!(this, 0, proc, false, method_info);  }

    }
    else { call_original!(this, force_type, proc, is_turn,method_info) }
}
//For Time Crystals Phase Change
#[skyline::hook(offset=0x02d56930)]
pub fn change_bgm2(this: u64, force_type: i32, method_info: OptionalMethod) {
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { call_original!(this, 0,  method_info) }
    else { call_original!(this, force_type, method_info) }
}
*/
pub fn patch_imm_bgm(){
    if GameVariableManager::get_bool(BGM_KEY) {
         Patch::in_text(0x0228d414).bytes(&[0x01, 0x00, 0x80, 0x52]).unwrap(); 
         Patch::in_text(0x02d568dc).bytes(&[0x14, 0x00, 0x80, 0x52]).unwrap(); // mov w20, 0x0 force 0 
         Patch::in_text(0x02d568e0).nop().unwrap(); // conditional check
        }
    else { 
        Patch::in_text(0x0228d414).bytes(&[0xe1, 0x03, 0x13, 0x2a]).unwrap();
        Patch::in_text(0x02d568dc).bytes(&[0x3f, 0x01, 0x14, 0x6b]).unwrap(); // mov w20, 0x0 force 0   3f 01 14 6b
        Patch::in_text(0x02d568e0).bytes(&[0x29, 0x02, 0x00, 0x54]).unwrap(); // conditional check 29 02 00 54
     }
}

pub struct BGMMod;
impl ConfigBasicMenuItemSwitchMethods for BGMMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_imm_bgm(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(BGM_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(BGM_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            patch_imm_bgm();
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(BGM_KEY) == 1 { this.help_text = "Player phase BGM overrides other phase BGM.".into(); } 
        else { this.help_text = "Default BGM Setting".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(BGM_KEY) == 0 { this.command_text = on_str();} 
        else { this.command_text = off_str(); }
    }
}
extern "C" fn bgm_switch() -> &'static mut ConfigBasicMenuItem { 
    let str0 = Mess::get("MID_CONFIG_BGM_CHANGE_ENEMYTURN");
    ConfigBasicMenuItem::new_switch::<BGMMod>(str0.to_string())
 }
pub fn bgm_install(){ cobapi::install_game_setting(bgm_switch); }