use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use crate::game_var_i;

pub const LEVEL_DIS_KEY: &str = "G_LEVEL_TYPE";
pub const GROWTH_KEY: &str = "G_GROWTH_TYPE";
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
fn restoreDefault(){
    //Growth Mode Call
    Patch::in_text(0x01a3a3c4).bytes(&[0xe7, 0x6a, 0x2b, 0x94]).unwrap();
    //Random
    Patch::in_text(0x01a3a658).bytes(&[0x14,0x81,0x40, 0x39]).unwrap();
    //Random RNG 
    Patch::in_text(0x01a3a73c).bytes(&[0x5d, 0xeb, 0x24, 0x94]).unwrap();

    //Fixed
    Patch::in_text(0x01a3a410).bytes(&[0x14,0x81,0x40, 0x39]).unwrap();
}
fn patchGrowth(){
    GameVariableManager::make_entry_norewind(GROWTH_KEY, 0);
    let result = game_var_i::getNumber(GROWTH_KEY);
    restoreDefault();
    if (result == 0 ){ 
        restoreDefault();
    }
    else if (result == 1){
        //Opposite Mode
        let growthMode = game_var_i::getGrowMode();
        if (growthMode == 0) {//Random -> Fixed
            Patch::in_text(0x01a3a3c4).bytes(&[0x20, 0x00, 0x80, 0xd2]).unwrap();
        }
        else { //Fixed -> Random
            Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
        }

    }
    else if (result == 2) {
        // No Growths
        Patch::in_text(0x01a3a410).bytes(&[0x14,0x00,0x80,0xD2]).unwrap();
        Patch::in_text(0x01a3a658).bytes(&[0x14,0x00, 0x80,0xD2]).unwrap();
    }
    else if (result == 3){
        // Perfect Level Ups, forcing to Random and RNG set to 1
        Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
        Patch::in_text(0x01a3a73c).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
    }
    else if (result == 4){
        //Negative Growths to be determine

    }
}
pub struct GrowthMod;
impl ConfigBasicMenuItemSwitchMethods for  GrowthMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchGrowth();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = game_var_i::getNumber(GROWTH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);

        if toggle != result {
            game_var_i::setNumber(GROWTH_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchGrowth();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC = game_var_i::getNumber(GROWTH_KEY);
        let growthMode = game_var_i::getGrowMode();
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
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C = game_var_i::getNumber(GROWTH_KEY);
        let growthMode = game_var_i::getGrowMode();
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
        else {this.help_text = format!("Unknown").into(); }
    }
}
pub struct LevelMod;
impl ConfigBasicMenuItemSwitchMethods for LevelMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ patchLvl(); }
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
extern "C" fn growth_callback() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<GrowthMod>("Growth Mode")
}
pub fn level_install(){
    cobapi::install_game_setting(growth_callback);
   cobapi::install_game_setting(level_callback);

}