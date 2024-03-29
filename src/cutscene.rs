use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use crate::string::*;
pub const CUTSCENES_KEY: &str = "G_CUTSCENE";

pub struct CutsceneMod;
pub fn patchCutscenes(){
    GameVariableManager::make_entry_norewind(CUTSCENES_KEY, 0);
    let active = GameVariableManager::get_bool(CUTSCENES_KEY);
    if (active){
        let replace = &[0xC0, 0x03, 0x5F, 0xD6];
        Patch::in_text(0x01ed8e20).bytes(replace).unwrap();
        Patch::in_text(0x01ed8ef0).bytes(replace).unwrap();
        println!("Cutscenes/Movies are skipped");
    }
    else {
        Patch::in_text(0x01ed8e20).bytes(&[0xFD, 0x7B, 0xBE, 0xA9]).unwrap();
        Patch::in_text(0x01ed8ef0).bytes(&[0xFD, 0x7B, 0xBD, 0xA9]).unwrap();
        println!("Cutscenes/Movies are not skipped");
    }
}
impl ConfigBasicMenuItemSwitchMethods for CutsceneMod {
    fn init_content(this: &mut ConfigBasicMenuItem){  patchCutscenes(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(CUTSCENES_KEY, 0);
        let toggle = GameVariableManager::get_bool(CUTSCENES_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(CUTSCENES_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchCutscenes();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(CUTSCENES_KEY);
        if (toggle) { this.help_text = "Disables cutscenes and movies during chapter maps.".into(); } 
        else { this.help_text = "Enables cutscenes and movies during chapter maps.".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(CUTSCENES_KEY);
        if (toggle) { this.command_text = On_str();} 
        else { this.command_text =  Off_str(); }
    }
}

#[no_mangle]
extern "C" fn cutscene() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CutsceneMod>("Skip Cutscenes/Movies") }
pub fn cutscene_install(){ cobapi::install_game_setting(cutscene); }