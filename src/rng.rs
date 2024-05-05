use skyline::patching::Patch;
use unity::prelude::*;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamevariable::*,
};
use crate::string::*;
pub const RNG_KEY: &str = "G_RNG_TYPE";
pub const SMASH_KEY: &str = "G_Smash_Attacks";

pub struct RNGMod;
pub fn patch_rng(){
    let result = GameVariableManager::get_number(RNG_KEY);
    let replace_h = &[0x11, 0xa0, 0x13, 0x94];
    let replace_rn = &[0xe0, 0xd7, 0x9f, 0x1a];
    let replace_rig = &[0x20, 0x00, 0x80, 0x52];
    if result == 0 || result == 5 {
        Patch::in_text(0x02375510).bytes(replace_rn).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replace_h).unwrap();
        if result == 5 { println!("RN display is active"); }
        else { println!("RNG Mode set to None"); }
    }
    else if result == 1 {// 1 RN 
        Patch::in_text(0x02375510).bytes(replace_rig).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replace_h).unwrap();
        println!("RNG Mode set to ignore 1RN");
    }
    else if result == 2 { // Hybrid
        Patch::in_text(0x01e8d12c).bytes(replace_rig).unwrap();
        Patch::in_text(0x02375510).bytes(replace_rn).unwrap();
        println!("RNG Mode set to ignore Hybrid RN");
    }
    else if result == 3{//1 RN + Hybrid 
        Patch::in_text(0x01e8d12c).bytes(replace_rig).unwrap();
        Patch::in_text(0x02375510).bytes(replace_rig).unwrap();
        println!("RNG Mode set to ignore 1RN and Hybrid RN");
    }
    else if result == 4 {
        Patch::in_text(0x02375510).bytes(replace_rn).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replace_h).unwrap();
        println!("RNG Mode set to 'Player Rig'");
    }
}

pub fn patch_smash() {
    if GameVariableManager::get_bool(SMASH_KEY) {
        Patch::in_text(0x02472714).bytes(&[0x80, 0x0C, 0x80, 0x52]).unwrap();
        Patch::in_text(0x02472CB8).bytes(&[0x8B, 0x02, 0x00, 0x54]).unwrap();
        Patch::in_text(0x02472758).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
        println!("Smashing activated");
    }
    else {
        Patch::in_text(0x02472714).bytes(&[0xAB, 0x0F, 0xE8, 0x97]).unwrap();
        Patch::in_text(0x02472CB8).bytes(&[0x81, 0x02, 0x00, 0x54]).unwrap();
        Patch::in_text(0x02472758).bytes(&[0xb2, 0x0f, 0xe8, 0x97]).unwrap();
        println!("Smashing deactivated");
    }
}
impl ConfigBasicMenuItemSwitchMethods for RNGMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_rng(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(RNG_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 5, 1);
        if toggle != result {
            GameVariableManager::set_number(RNG_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_rng();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_c =  GameVariableManager::get_number(RNG_KEY);
        if type_c == 0 {this.help_text = "Default RNG behavior.".into(); }
        else if type_c == 1 { this.help_text = "Disables normal RNG. (Crits, Skill Procs, Well, Cooking, etc.)".into(); }
        else if type_c == 2 { this.help_text = "Disables hybrid RNG. (Hit Rates)".into(); }
        else if type_c == 3 { this.help_text = "Disables normal and hybrid RNG. (No Randomness)".into();  }
        else if type_c == 4 { this.help_text = "Player and Ally units will have favorable combat.".into(); }
        else if type_c == 5 { this.help_text = "Upcoming RNs are displayed in the 'Hit' help box.".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_c =  GameVariableManager::get_number(RNG_KEY);
        if type_c == 0 {this.command_text = "Default".into(); }
        else if type_c == 1 { this.command_text = "Rig Normal".into(); }
        else if type_c == 2 { this.command_text = "Rig Hybrid".into(); }
        else if type_c == 3 { this.command_text = "Rig Normal/Hybrid".into();  }
        else if type_c == 4 { this.command_text = "Rig Player Combat".into();  }
        else if type_c == 5 { this.command_text = "Display RN Queue".into();  }
    }
}
pub struct SmashMod;
impl ConfigBasicMenuItemSwitchMethods for SmashMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_smash();  }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(SMASH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(SMASH_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_smash(); 
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_bool(SMASH_KEY) {this.help_text = "First hit of every attack will smash.".into(); }
        else { this.help_text = "Default behavior for smash attacks.".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_bool(SMASH_KEY) {this.command_text = on_str(); }
        else { this.command_text = off_str(); }
    }
}
#[no_mangle]
extern "C" fn rng() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<RNGMod>("RNG Mode") }
#[no_mangle]
extern "C" fn smash() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<SmashMod>("All Smash Attacks") }

pub fn rng_install(){ 
    cobapi::install_game_setting(rng);
    cobapi::install_game_setting(smash);
}