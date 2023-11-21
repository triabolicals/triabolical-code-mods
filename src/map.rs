use skyline::patching::Patch;
use unity::prelude::*;
use engage::gamevariable::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};

pub const MDIALOGUE_KEY: &str = "G_MDIALOGUE";
pub const MTUTORIAL_KEY: &str = "G_MTUTORIAL";

fn patchTutorial(){
    GameVariableManager::make_entry_norewind(MTUTORIAL_KEY, 0);
    let active = GameVariableManager::get_bool(MTUTORIAL_KEY);
    if (active){ Patch::in_text(0x01ed91c0).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap(); }
    else {  Patch::in_text(0x01ed91c0).bytes(&[0xfd,0x7b,0xbd,0xa9]).unwrap(); }
}
fn patchDialog(){
    GameVariableManager::make_entry_norewind(MDIALOGUE_KEY, 0);
    let active = GameVariableManager::get_bool(MDIALOGUE_KEY);
    if (active){ Patch::in_text(0x01ed8370).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap(); }
    else { Patch::in_text(0x01ed8370).bytes(&[0xfd, 0x7b, 0xbd, 0xa9]).unwrap(); }
}

pub struct MapDialog;
impl ConfigBasicMenuItemSwitchMethods for MapDialog {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchDialog();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(MDIALOGUE_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(MDIALOGUE_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchDialog();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active = GameVariableManager::get_bool(MDIALOGUE_KEY);
        if (active){ this.help_text = format!("Disables map dialogue.").into(); } 
        else { this.help_text = format!("Enables map dialogue.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active = GameVariableManager::get_bool(MDIALOGUE_KEY);
        if (active) { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}

pub struct MapTutorial;
impl ConfigBasicMenuItemSwitchMethods for MapTutorial {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchTutorial();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(MTUTORIAL_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(MTUTORIAL_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchTutorial();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(MTUTORIAL_KEY);
        if (toggle) { this.help_text = format!("Disables in-map tutorials.").into(); } 
        else { this.help_text = format!("Enables in-map tutorials").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(MTUTORIAL_KEY);
        if (toggle) { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}

#[no_mangle]
extern "C" fn mapDialog() -> &'static mut ConfigBasicMenuItem { 
    engage::menu::config::ConfigBasicMenuItem::new_switch::<MapDialog>("Skip Map Dialogue")
}

#[no_mangle]
extern "C" fn mapTutorial() -> &'static mut ConfigBasicMenuItem { 
    engage::menu::config::ConfigBasicMenuItem::new_switch::<MapTutorial>("Skip Map Tutorials") 
}

pub fn map_mod_install() {
    cobapi::install_game_setting(mapDialog);
    cobapi::install_game_setting(mapTutorial);
}