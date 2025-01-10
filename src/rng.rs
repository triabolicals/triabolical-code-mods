use skyline::patching::Patch;
use unity::prelude::*;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamevariable::*,
    random::*,
};
use crate::string::*;
pub const RNG_KEY: &str = "G_RNG_TYPE";
pub const SMASH_KEY: &str = "G_Smash_Attacks";

pub struct RNGMod;
pub fn patch_rng(){
    /*
    let result = GameVariableManager::get_number(RNG_KEY);
    let replace_h = &[0x11, 0xa0, 0x13, 0x94];
    let replace_rn = &[0xe0, 0xd7, 0x9f, 0x1a];
    let replace_rig = &[0x20, 0x00, 0x80, 0x52];
    match result {
        1 => {
            Patch::in_text(0x02375510).bytes(replace_rig).unwrap();
            Patch::in_text(0x01e8d12c).bytes(replace_h).unwrap();
            println!("RNG Mode set to ignore 1RN");
        }
        2 => {
            Patch::in_text(0x01e8d12c).bytes(replace_rig).unwrap();
            Patch::in_text(0x02375510).bytes(replace_rn).unwrap();
            println!("RNG Mode set to ignore Hybrid RN");
        }
        3 => {
            Patch::in_text(0x01e8d12c).bytes(replace_rig).unwrap();
            Patch::in_text(0x02375510).bytes(replace_rig).unwrap();
            println!("RNG Mode set to ignore 1RN and Hybrid RN");
        }
        4 => {
            Patch::in_text(0x02375510).bytes(replace_rn).unwrap();
            Patch::in_text(0x01e8d12c).bytes(replace_h).unwrap();
            println!("RNG Mode set to 'Player Rig'");
        }
        _ => {
            Patch::in_text(0x02375510).bytes(replace_rn).unwrap();
            Patch::in_text(0x01e8d12c).bytes(replace_h).unwrap();
        }
    }
    */
}

#[skyline::hook(offset=0x01e8d0e0)]
pub fn battle_math(ratio: i32, method_info: OptionalMethod) -> bool {
    match GameVariableManager::get_number(RNG_KEY) {
        2|3 => { ratio > 0 }
        6 => {
            let rng = Random::get_game();
            rng.get_value(100) < ratio
        }
        7 => {
            let rng = Random::get_game();
            let rn = ( rng.get_value(100) + rng.get_value(100) ) >> 1;
            rn < ratio
        }
        _ => { call_original!(ratio, method_info) }
    }
    /*
    let game_rng = Random::get_game();
    let rng = Random::instantiate().unwrap();
    copy_random(rng, game_rng);
    let value = rng.get_value(10000);
    let result = call_original!(ratio, method_info);
    println!("Hybrid Ratio: {}, RN Value: {}, Result = {}", ratio, value, result);
    return result;
    */
}
#[skyline::hook(offset=0x01e8d0b0)]
pub fn prob_100(this: i32, method_info: OptionalMethod) -> bool {
    match GameVariableManager::get_number(RNG_KEY) {
        1|3 => { this > 0 }
        _ => { call_original!(this, method_info) }
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
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 7, 1);
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
        let type_c =  
        this.help_text = match GameVariableManager::get_number(RNG_KEY) {
            1 => { "Disables normal RNG. (Crits, Skill Procs, Well, Cooking, etc.)" },
            2 => { "Disables hybrid RNG. (Hit Rates)" },
            3 => { "Disables normal and hybrid RNG. (No Randomness)" },
            4 => { "Player and Ally units will have favorable combat." },
            5 => { "RNs are displayed in the 'Hit' help box. True Hit in Preview." },
            6 => { "Hit rates are determined by 1RN"}
            7 => { "Hit rates will be determined by 2RN."}
            _ => { "Default RNG behavior." }
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(RNG_KEY) {
            1 => { "Rig 1RN"},
            2 => { "Rig Hybrid"},
            3 => { "Rig All"},
            4 => { "Rig Player Combat"},
            5 => { "Display RNs"},
            6 => { "1RN Hit Rates"},
            7 => { "2RN Hit Rates"},
            _ => { "Default" },
        }.into();
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
        this.help_text = 
            if GameVariableManager::get_bool(SMASH_KEY) { "First hit of every attack will smash."}
            else { "Default behavior for smash attacks." }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = 
            if GameVariableManager::get_bool(SMASH_KEY) { on_str() }
            else {  off_str() };
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