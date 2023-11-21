use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
pub const LEVEL_DIS_KEY: &str = "G_LEVEL_TYPE";
fn patchLvl(){
    GameVariableManager::make_entry_norewind(LEVEL_DIS_KEY, 0);
    let result = GameVariableManager::get_bool(LEVEL_DIS_KEY);
    if (result){
        Patch::in_text(0x01a5bbc0).bytes(&[0x02, 0x24, 0x82, 0x39]).unwrap();
        Patch::in_text(0x01a5bbc4).bytes(&[0x03, 0x60, 0x85, 0x39]).unwrap();
        Patch::in_text(0x01a5bbc8).bytes(&[0x40, 0x00, 0x03, 0x8B]).unwrap();
        Patch::in_text(0x01a5bbcc).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();

        Patch::in_text(0x01f9e280).bytes(&[0x50, 0xF6, 0xEA, 0x97]).unwrap();
        Patch::in_text(0x01f9e290).bytes(&[0x4C, 0xF6, 0xEA, 0x97]).unwrap();
        Patch::in_text(0x01c669fc).bytes(&[0x71, 0xD4, 0xF7, 0x97]).unwrap();
        Patch::in_text(0x01c66a0c).bytes(&[0x6D, 0xD4, 0xF7, 0x97]).unwrap();
        println!("Level Display switch to Total");
    }
    else {
        Patch::in_text(0x01a5bbc0).bytes(&[0x00, 0x24, 0x42, 0x39]).unwrap();
        Patch::in_text(0x01a5bbc4).bytes( &[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
        Patch::in_text(0x01f9e280).bytes(&[0x08, 0x56, 0xea, 0x97]).unwrap();
        Patch::in_text(0x01f9e290).bytes(&[0x04, 0x56, 0xea, 0x97]).unwrap();
        Patch::in_text(0x01c669fc).bytes(&[0x29, 0x34, 0xf7, 0x97]).unwrap();
        Patch::in_text(0x01c66a0c).bytes(&[0x25, 0x34, 0xf7, 0x97]).unwrap();
        println!("Level Display switch to Default");
    }
}
pub struct LevelMod;
impl ConfigBasicMenuItemSwitchMethods for LevelMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchLvl();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(LEVEL_DIS_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchLvl();
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
extern "C" fn level_callback() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<LevelMod>("Unit Level Display")
}

pub fn level_install(){
   cobapi::install_game_setting(level_callback);
}