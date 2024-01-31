use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gamedata::unit::Unit;
use engage::force::*;
use engage::gamedata::JobData;
use engage::gameuserdata::GameUserData;
pub const RNG_KEY: &str = "G_RNG_TYPE";

pub struct RNGMod;
pub fn patchRNG(){
    GameVariableManager::make_entry_norewind(RNG_KEY, 0);
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
}

impl ConfigBasicMenuItemSwitchMethods for RNGMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchRNG();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(RNG_KEY, 0);
        let toggle =  GameVariableManager::get_number(RNG_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 4, 1);

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
        if typeC == 0 {this.help_text = format!("Default RNG behavior.").into(); }
        else if typeC == 1 { this.help_text = format!("Disables normal RNG. (Crits, Skill Procs, Well, Cooking, etc.)").into(); }
        else if typeC == 2 { this.help_text = format!("Disables hybrid RNG. (Hit Rates)").into(); }
        else if typeC == 3 { this.help_text = format!("Disables normal and hybrid RNG. (No Randomness)").into();  }
        else if typeC == 4 { this.help_text = format!("Player and Ally units will have favorable combat.").into(); }
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(RNG_KEY);
        if type_C == 0 {this.command_text = format!("None").into(); }
        else if type_C == 1 { this.command_text = format!("Rig Normal").into(); }
        else if type_C == 2 { this.command_text = format!("Rig Hybrid").into(); }
        else if type_C == 3 { this.command_text = format!("Rig Normal/Hybrid").into();  }
        else if type_C == 4 { this.command_text = format!("Rig Player Combat").into();  }
        else {this.help_text = format!("Unknown").into(); }
    }
}

#[no_mangle]
extern "C" fn RNG() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<RNGMod>("RNG Rigging Mode") }
pub fn rng_install(){ cobapi::install_game_setting(RNG); }