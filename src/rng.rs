use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::{gamedata::unit::Unit, gamevariable::*, random::*, force::*};
use crate::string::*;
use engage::gamedata::JobData;
use engage::gameuserdata::GameUserData;
pub const RNG_KEY: &str = "G_RNG_TYPE";
pub const SMASH_KEY: &str = "G_Smash_Attacks";
pub struct RNGMod;
pub fn patchRNG(){
    GameVariableManager::make_entry(RNG_KEY, 0);
    let result =  GameVariableManager::get_number(RNG_KEY);
    let replaceH = &[0x11, 0xa0, 0x13, 0x94];
    let replaceRN = &[0xe0, 0xd7, 0x9f, 0x1a];
    let replaceRig = &[0x20, 0x00, 0x80, 0x52];
  
    if (result == 0){
        Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        println!("RNG Mode set to None");
    }
    else if (result == 1){// 1 RN 
        Patch::in_text(0x02375510).bytes(replaceRig).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        println!("RNG Mode set to ignore 1RN");
    }
    else if (result == 2){ // Hybrid
        Patch::in_text(0x01e8d12c).bytes(replaceRig).unwrap();
        Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        println!("RNG Mode set to ignore Hybrid RN");
    }
    else if (result == 3){//1 RN + Hybrid 
        Patch::in_text(0x01e8d12c).bytes(replaceRig).unwrap();
        Patch::in_text(0x02375510).bytes(replaceRig).unwrap();
        println!("RNG Mode set to ignore 1RN and Hybrid RN");
    }
    else if result == 4 {
        Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        println!("RNG Mode set to 'Player Rig'");
    }
    else if result == 5 {
        Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        println!("RNG Mode set to 1 RN Only");
    }
    else if result == 6 {
        Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        println!("RNG Mode set to 2 RN for Hit Rates");
    }
}

pub fn patch_smash() {
    GameVariableManager::make_entry(SMASH_KEY, 0);
    let result =  GameVariableManager::get_bool(SMASH_KEY);
    if result {
        Patch::in_text(0x02472714).bytes(&[0x80, 0x0C, 0x80, 0x52]);
        Patch::in_text(0x02472CB8).bytes(&[0x8B, 0x02, 0x00, 0x54]);
        Patch::in_text(0x02472758).bytes(&[0x20, 0x00, 0x80, 0x52]);
        println!("Smashing activated");
    }
    else {
        Patch::in_text(0x02472714).bytes(&[0xAB, 0x0F, 0xE8, 0x97]);
        Patch::in_text(0x02472CB8).bytes(&[0x81, 0x02, 0x00, 0x54]);
        Patch::in_text(0x02472758).bytes(&[0xb2, 0x0f, 0xe8, 0x97]);
        println!("Smashing deactivated");

    }
}
impl ConfigBasicMenuItemSwitchMethods for RNGMod {
    fn init_content(this: &mut ConfigBasicMenuItem){  }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(RNG_KEY, 0);
        let toggle =  GameVariableManager::get_number(RNG_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 6, 1);

        if toggle != result {
            GameVariableManager::set_number(RNG_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchRNG();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(RNG_KEY);
        if typeC == 0 {this.help_text = "Default RNG behavior.".into(); }
        else if typeC == 1 { this.help_text = "Disables normal RNG. (Crits, Skill Procs, Well, Cooking, etc.)".into(); }
        else if typeC == 2 { this.help_text = "Disables hybrid RNG. (Hit Rates)".into(); }
        else if typeC == 3 { this.help_text = "Disables normal and hybrid RNG. (No Randomness)".into();  }
        else if typeC == 4 { this.help_text = "Player and Ally units will have favorable combat.".into(); }
        else if typeC == 5 { this.help_text = "Hit Rates will use 1 RN".into(); }
        else if typeC == 6 { this.help_text = "Hit Rates will use 2 RN".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(RNG_KEY);
        if type_C == 0 {this.command_text = "Default".into(); }
        else if type_C == 1 { this.command_text = "Rig Normal".into(); }
        else if type_C == 2 { this.command_text = "Rig Hybrid".into(); }
        else if type_C == 3 { this.command_text = "Rig Normal/Hybrid".into();  }
        else if type_C == 4 { this.command_text = "Rig Player Combat".into();  }
        else if type_C == 5 { this.command_text = "1 RN Hit Rates".into();  }
        else if type_C == 6 { this.command_text = "2 RN Hit Rates".into();  }
    }
}
pub struct SmashMod {}
impl ConfigBasicMenuItemSwitchMethods for SmashMod {
    fn init_content(this: &mut ConfigBasicMenuItem){  }//patch_smash();  }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(SMASH_KEY, 0);
        let toggle =  GameVariableManager::get_bool(SMASH_KEY);
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
        if GameVariableManager::get_bool(SMASH_KEY) {this.command_text = On_str(); }
        else { this.command_text = Off_str(); }
    }
}
#[no_mangle]
extern "C" fn RNG() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<RNGMod>("RNG Mode") }

extern "C" fn Smash() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<SmashMod>("All Smash Attacks") }
pub fn rng_install(){ 
    cobapi::install_game_setting(RNG);
    cobapi::install_game_setting(Smash);
}
pub fn copy_seed(src: &Random, dst: &mut Random){
    dst.seed1 = src.seed1;
    dst.seed2 = src.seed2;
    dst.seed3 = src.seed3;
    dst.seed4 = src.seed4;
}

#[skyline::hook(offset=0x01e8d0e0)]
pub fn hybrid_hook(ratio: i32, method_info: OptionalMethod) -> bool {
    if GameVariableManager::get_number(RNG_KEY) == 6 {
        unsafe {
            let rng = Random::get_game();
            let value1 = rng.get_value(100);
            let value2 = rng.get_value(100);
            return value1 + value2 <= 2*ratio
        }

    }
    else if GameVariableManager::get_number(RNG_KEY) == 5 {
        unsafe {
            let rng = Random::get_game();
            let value1 = rng.get_value(100);
            return value1 <= ratio;
        }
    }
    else {  call_original!(ratio, method_info) }

}