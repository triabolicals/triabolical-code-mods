use skyline::patching::Patch;
use unity::prelude::*;
use engage::gamevariable::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
pub const MAP_KEY: &str = "G_MAP_SKIP";

pub fn patchMap(){
    GameVariableManager::make_entry_norewind(MAP_KEY, 0);
    let active = GameVariableManager::get_number(MAP_KEY);
    if active == 0 { //None
        Patch::in_text(0x01ed91c0).bytes(&[0xfd,0x7b,0xbd,0xa9]).unwrap();
        Patch::in_text(0x01ed8370).bytes(&[0xfd, 0x7b, 0xbd, 0xa9]).unwrap();
    }
    else if active == 1 { // Tutorial
        Patch::in_text(0x01ed91c0).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
            Patch::in_text(0x01ed8370).bytes(&[0xfd, 0x7b, 0xbd, 0xa9]).unwrap();
            println!("Map Tutorials are skipped");
    }
    else if active == 2 {//Map
        Patch::in_text(0x01ed8370).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
        Patch::in_text(0x01ed91c0).bytes(&[0xfd,0x7b,0xbd,0xa9]).unwrap();
        println!("Map Dialogue are skipped");
    }
    else if active == 3 { //Both
        Patch::in_text(0x01ed91c0).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
        Patch::in_text(0x01ed8370).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
        println!("Map Tutorials and Dialogue are skipped");
    }
}
pub struct MapMod;
impl ConfigBasicMenuItemSwitchMethods for MapMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(MAP_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(MAP_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchMap();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active  =  GameVariableManager::get_number(MAP_KEY);
        if active == 0 { this.help_text = "Enables map dialogue and tutorials.".into(); } 
        else if active == 1 { this.help_text = "Skips in-map tutorials.".into(); }
        else if active == 2 { this.help_text = "Skips in-map dialogue.".into(); }
        else if active == 3 { this.help_text = "Skips in-map tutorials and dialogue.".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active  =  GameVariableManager::get_number(MAP_KEY);
        if active == 0 { this.command_text = "Off".into(); } 
        else if active == 1 { this.command_text = "Tutorials Only".into(); }
        else if active == 2 { this.command_text = "Dialogue Only".into(); }
        else if active == 3 { this.command_text = "Tutorials/Dialogue".into(); }
    }
}


#[no_mangle]
extern "C" fn maps() -> &'static mut ConfigBasicMenuItem {  engage::menu::config::ConfigBasicMenuItem::new_switch::<MapMod>("Skip Map Dialogue/Tutorials") }

pub fn map_mod_install() {
    cobapi::install_game_setting(maps);
}
