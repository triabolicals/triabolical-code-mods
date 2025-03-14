use crate::utils::off_str;
use super::*;
use engage::{
    reliance::UnitReliance,  
    force::*,
    gamedata::{achieve::AchieveData, unit::Unit},
};
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
        this.help_text = match GameVariableManager::get_number(SUPPORT_KEY) {
            1 => { "Skip bond conversations in the reference menu." },
            2 => { "Skip support conversations in the reference menu." },
            3 => { "Skip bond and support conversations in the reference menu." },
            _ => { "Play bond and support conversations in the reference menu." },
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(SUPPORT_KEY) {
            1 => {  Mess::get("MID_MENU_Recall_GodReliance_Unit") }
            2 => { Mess::get("MID_MENU_Recall_Reliance_Unit") }
            3 => { format!("{} / {}", Mess::get("MID_MENU_Recall_Reliance_Unit"), Mess::get("MID_MENU_Recall_GodReliance_Unit")).into() }
            _ => { off_str() }
        };
    }
}

extern "C" fn supports() -> &'static mut ConfigBasicMenuItem { 
    let label = format!("Skip {} / {}", Mess::get("MID_MENU_Recall_Reliance_Unit"), Mess::get("MID_MENU_Recall_GodReliance_Unit"));
    ConfigBasicMenuItem::new_switch::<SupportMod>(label)
 } 
pub fn support_install(){ cobapi::install_game_setting(supports); }

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

#[skyline::hook(offset=0x01c5a930)]
pub fn reliance_can_level_up_hook(unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod) -> bool {
    let result = call_original!(unit_a, unit_b, method_info);
    if GameVariableManager::get_number(SUPPORT_KEY) < 2 { return result; }
    if result {
        UnitReliance::level_up(unit_a, unit_b);
        UnitReliance::try_get(unit_a, unit_b)
            .map(|reliance_data|{
                if reliance_data.level == 3 { AchieveData::add_count_reliance_a(); }
                else if reliance_data.level == 2 { AchieveData::add_count_reliance_b(); }
                AchieveData::set_clear_reliance(unit_a.person);
                AchieveData::set_clear_reliance(unit_b.person);
            }
        );
    }
    return result;
}

pub fn update_reliances_event(){
    // unit reliance
    if GameVariableManager::get_number(SUPPORT_KEY) < 2 { return; }
    let mut count = 0;
    let units: Vec<&Unit> = Force::get(ForceType::Player).unwrap().iter().chain(  Force::get(ForceType::Absent).unwrap().iter() ).collect();

    for x in 0..units.len() {
        let unit_a = units[x];
        for y in x+1..units.len() {
            if reliance_can_level_up_hook(unit_a, units[y], None) { count += 1; }
        }
    }
    if count != 0 { println!("{} supports are leveled up", count); }
}
