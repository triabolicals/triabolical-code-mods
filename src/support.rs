use skyline::patching::Patch;
use unity::{prelude::*, system::List,};
use engage::{
    gamevariable::*,
    gamedata::{*, unit::Unit},
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    mess::*,
};
use crate::string::*;
pub const SUPPORT_KEY: &str = "G_SUPPORT_TYPE";

pub fn patch_support() {
    let replace_bond = &[0xc0, 0x24, 0x11, 0x14];
    let replace_support = &[0x51,0x24,0x011,0x94];
    let replace_rig = &[0xC0, 0x03, 0x5F, 0xD6];

    let duration_0 = &[0xE0, 0x03, 0x27, 0x1E];
    let duration_25 = &[0x00, 0x10, 0x2a, 0x1e];
    
    match GameVariableManager::get_number(SUPPORT_KEY) {
        1 => {
        //Bonds
            Patch::in_text(0x02096d7c).bytes(duration_0).unwrap();
            Patch::in_text(0x02097028).bytes(duration_0).unwrap();
        //Supports normal
            Patch::in_text(0x02097c90).bytes(replace_bond).unwrap();
            Patch::in_text(0x020987ec).bytes(&[0x45, 0x61, 0x07, 0x94]).unwrap();
            Patch::in_text(0x02096d88).bytes(&[0xAE, 0x14, 0x1E, 0x94]).unwrap(); // remove black fade

            Patch::in_text(0x02097e4c).nop().unwrap();
            Patch::in_text(0x020988a4).bytes(replace_rig).unwrap();
            Patch::in_text(0x02098a60).bytes(replace_rig).unwrap();
        }
        2 => { // Support
            Patch::in_text(0x02096d7c).bytes(duration_0).unwrap();
            Patch::in_text(0x02097028).bytes(duration_0).unwrap();
            Patch::in_text(0x02096d88).bytes(&[0xAE, 0x14, 0x1E, 0x94]).unwrap(); // remove black fade
        // 02096d88 FadeOut 02096d88 at 0281c040
        //bonds normal
            Patch::in_text(0x02097e4c).bytes(replace_support).unwrap();
            Patch::in_text(0x020988a4).bytes(&[0x67, 0x22, 0x05, 0x14]).unwrap();
            Patch::in_text(0x02098a60).bytes(&[0xf8, 0x21, 0x05, 0x14]).unwrap();

            Patch::in_text(0x02097c90).bytes(replace_rig).unwrap();
            Patch::in_text(0x020987ec).nop().unwrap();
        }
        3 => {
            Patch::in_text(0x02096d7c).bytes(duration_0).unwrap();
            Patch::in_text(0x02097028).bytes(duration_0).unwrap();

            Patch::in_text(0x02097e4c).nop().unwrap();
            Patch::in_text(0x020988a4).bytes(replace_rig).unwrap();
            Patch::in_text(0x02098a60).bytes(replace_rig).unwrap();

            Patch::in_text(0x02097c90).bytes(replace_rig).unwrap();
            Patch::in_text(0x020987ec).nop().unwrap(); 
            Patch::in_text(0x02096d88).bytes(&[0xAE, 0x14, 0x1E, 0x94]).unwrap(); // remove black fade
        }
        _ => {
            Patch::in_text(0x02097c90).bytes(replace_bond).unwrap();
            Patch::in_text(0x020987ec).bytes(&[0x45, 0x61, 0x07, 0x94]).unwrap();
            Patch::in_text(0x02096d7c).bytes(duration_25).unwrap();
            Patch::in_text(0x02097028).bytes(duration_25).unwrap();

            Patch::in_text(0x02097e4c).bytes(replace_support).unwrap();
            Patch::in_text(0x020988a4).bytes(&[0x67, 0x22, 0x05, 0x14]).unwrap();
            Patch::in_text(0x02098a60).bytes(&[0xf8, 0x21, 0x05, 0x14]).unwrap();
            Patch::in_text(0x02096d88).bytes(&[0x36, 0xf3, 0x32, 0x94]).unwrap(); // 36 f3 32 94
        }
    }
}

pub struct SupportMod;
impl ConfigBasicMenuItemSwitchMethods for SupportMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_support(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(SUPPORT_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(SUPPORT_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_support();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let mode =  GameVariableManager::get_number(SUPPORT_KEY);
        if mode == 0 {this.help_text = format!("Play bond and support conversations in the reference menu.").into(); }
        else if mode == 1 { this.help_text = format!("Skip bond conversations in the reference menu.").into(); }
        else if mode == 2 { this.help_text = format!("Skip support conversations in the reference menu.").into(); }
        else if mode == 3 { this.help_text = format!("Skip bond and support conversations in the reference menu.").into();  }
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let mode =  GameVariableManager::get_number(SUPPORT_KEY);
        if mode == 0 {this.command_text = off_str(); }
        else if mode == 1 { this.command_text = Mess::get("MID_MENU_Recall_GodReliance_Unit"); }
        else if mode == 2 { this.command_text = Mess::get("MID_MENU_Recall_Reliance_Unit"); }
        else if mode == 3 { unsafe { this.command_text = concat_strings3(Mess::get("MID_MENU_Recall_Reliance_Unit"), " / ".into(), Mess::get("MID_MENU_Recall_GodReliance_Unit"), None ); } }
        else {this.help_text = format!("Unknown").into(); }
    }
}

#[no_mangle]
extern "C" fn supports() -> &'static mut ConfigBasicMenuItem { 
    unsafe {
        let label = concat_strings3(Mess::get("MID_MENU_Recall_Reliance_Unit"), " / ".into(), Mess::get("MID_MENU_Recall_GodReliance_Unit"), None ); 
        ConfigBasicMenuItem::new_switch::<SupportMod>(concat_strings("Skip ".into(), label, None).get_string().unwrap())
    }
 } 
pub fn support_install(){ cobapi::install_game_setting(supports); }

#[unity::class("App", "UnitRelianceData")]
pub struct UnitRelianceData {
    reliance: u64,
    pub level: i32,
    pub exp: i8,
    pub score: i8,
}

#[skyline::from_offset(0x01c5abf0)]
pub fn reliance_level_up( unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod);

#[unity::from_offset("App", "AchieveData","AddCountRelianceA")]
pub fn add_a_reliance_count(method_info: OptionalMethod);

#[unity::from_offset("App", "AchieveData","AddCountRelianceB")]
pub fn add_b_reliance_count(method_info: OptionalMethod);

#[unity::from_offset("App", "PersonData", "IsHero")]
pub fn person_is_hero(this: &PersonData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x027c8910)]
pub fn set_clear_reliance(this: &PersonData, method_info: OptionalMethod);

#[skyline::from_offset(0x01c5a930)]
pub fn reliance_can_level_up( unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "MyRoomRelianceSelect","GetUnitList")]
pub fn my_room_reliance_select_get_unit_list(method_info: OptionalMethod) -> &'static List<Unit>;

#[skyline::from_offset(0x01c5a040)]
pub fn unit_reliance_try_get(unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod) -> Option<&'static mut UnitRelianceData>;

#[skyline::from_offset(0x01c5c450)]
pub fn unit_get_exp_next_level(this: & UnitRelianceData, current_level: i32, method_info: OptionalMethod ) -> i32;

pub fn update_reliances(){
    // unit reliance
    println!("Support Key: {}", GameVariableManager::get_number(SUPPORT_KEY));
    if GameVariableManager::get_number(SUPPORT_KEY) < 2 { return; }
    let mut count = 0;
    unsafe {
        let unit_list = my_room_reliance_select_get_unit_list(None);
        for x in 0..unit_list.len() {
            let unit_a = &unit_list[x];
            for y in x+1..unit_list.len(){
                let unit_b = &unit_list[y];
                let reliance = unit_reliance_try_get(unit_a, unit_b, None);
                if reliance.is_some(){
                    let reliance_data = reliance.unwrap();
                    if reliance_can_level_up(unit_a, unit_b, None) {
                        count += 1;
                        reliance_level_up(unit_a, unit_b, None);
                        if reliance_data.level == 3 { add_a_reliance_count(None); }
                        else if reliance_data.level == 2 { add_b_reliance_count(None); }
        
                        if person_is_hero(unit_a.person, None) { set_clear_reliance(unit_b.person, None); }
                        else { set_clear_reliance(unit_a.person, None);
                            if !person_is_hero(unit_b.person, None) { set_clear_reliance(unit_b.person, None);  }
                        }
                        println!("{} and {} leveled up support", Mess::get(unit_a.person.get_name().unwrap()).get_string().unwrap(), Mess::get(unit_b.person.get_name().unwrap()).get_string().unwrap());
                    }
                }
            }
        }
    }
    if count != 0 { println!("{} supports are leveled up", count); }
}

