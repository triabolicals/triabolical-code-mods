use super::*;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    battle::BattleCalculator,
    random::*,
};
use std::sync::OnceLock;
pub static HYBRID_TABLE: OnceLock<Vec<(i32, i32)>> = OnceLock::new();

pub struct RNGMod;
impl ConfigBasicMenuItemSwitchMethods for RNGMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry_norewind(RNG_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(RNG_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 7, 1);
        if toggle != result {
            GameVariableManager::set_number(RNG_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
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

extern "C" fn rng() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<RNGMod>("RNG Mode") }
pub fn rng_install(){ cobapi::install_game_setting(rng); }

pub fn intialize_hybrid_table() {
    HYBRID_TABLE.get_or_init(||{
        let mut vec: Vec<(i32, i32)> = Vec::new();
        for x in 0..101 {
            let true_ratio = unsafe { get_hit_ratio_10000(x, None) };
            vec.push( (x, true_ratio) );
        }
        vec
    });

}

pub fn get_hybrid_to_display(hybrid: i32) -> i32 {
    let table = HYBRID_TABLE.get().unwrap();
    let found = table.iter().find(|&&x| x.1 == hybrid);
    if found.is_some() { found.unwrap().0 }
    else { hybrid }
}


#[skyline::hook(offset=0x01e8d0e0)]
pub fn battle_math_hook(ratio: i32, method_info: OptionalMethod) -> bool {
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
}
#[skyline::hook(offset=0x01e8d0b0)]
pub fn probability_100_hook(this: i32, method_info: OptionalMethod) -> bool {
    match GameVariableManager::get_number(RNG_KEY) {
        1|3 => { this > 0 }
        _ => { call_original!(this, method_info) }
    }
}

#[skyline::hook(offset=0x02470d60)]
pub fn calc_action_hook(this: &BattleCalculator, side_type: i32, method_info: OptionalMethod) -> bool {
    if GameVariableManager::get_number(rng::RNG_KEY) == 4 { 
        Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]).unwrap();
        Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap();
        if let Some(unit) =  this.info.get_unit(side_type) {
            if unit.force.is_some() {
                if unit.person.get_asset_force() == 0 || unit.force.unwrap().force_type == 0 { 
                    Patch::in_text(0x01e8d12c).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();   // Hybrid Rng set to 1
                    Patch::in_text(0x02375510).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();   // 1 RN Rng set to 1
                }
                else { 
                    Patch::in_text(0x01e8d12c).bytes(&[0xE0,0xE1, 0x84, 0x52]).unwrap();    // Hybrid RNG set to 9999
                    Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap();   // 1 RN RNG set to normal
                }
            }
        }
        let result = call_original!(this, side_type, method_info);
        Patch::in_text(0x01e8d12c).bytes(&[0x11, 0xa0, 0x13, 0x94]).unwrap();
        Patch::in_text(0x02375510).bytes(&[0xe0, 0xd7, 0x9f, 0x1a]).unwrap();
        return result;
    }
    call_original!(this, side_type, method_info)
}

#[skyline::from_offset(0x01e8d200)]
fn get_hit_ratio_10000(ratio: i32, method_info: OptionalMethod) -> i32;