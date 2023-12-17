use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
pub const BGM_KEY: &str = "G_BGM";

pub struct BGMmod;
impl ConfigBasicMenuItemSwitchMethods for BGMmod {
    fn init_content(this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry_norewind(BGM_KEY, 0);}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(BGM_KEY);
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
        let toggle = GameVariableManager::get_bool(BGM_KEY);
        if (toggle) { this.help_text = format!("Player phase BGM overrides other phase BGM.").into(); } 
        else { this.help_text = format!("Default BGM Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(BGM_KEY);
        if (toggle) { this.command_text = format!("Skip Enemy/Ally Phase").into();} 
        else { this.command_text = format!("Default").into(); }
    }
}

//Force BGM to player phase
#[skyline::hook(offset=0x02d56700)]
pub fn ChangeBGM(this: u64, forceType: i32, proc: u64, isTurn: bool, method_info: OptionalMethod) {
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { call_original!(this, 0, proc, isTurn, method_info) }
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

extern "C" fn bgm() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<BGMmod>("Battle BGM Settings") }
pub fn bgm_install(){ cobapi::install_game_setting(bgm); }
