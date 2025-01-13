use skyline::patching::Patch;
use unity::prelude::*;
use engage::{
    force::*,
    gamedata::{cook::*, *}, gameuserdata::GameUserData, gamevariable::*, hub::HubUtil, menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuItemAttribute, BasicMenuResult}, mess::*
};
pub const COOK_KEY: &str = "G_CHEF";
use crate::string::*;

pub fn get_cook_taste(pid: &Il2CppString, kind: i32) -> &'static Il2CppString {
    unsafe {
        let data = CookData::get(&pid.to_string());
        let mut taste: Option<&TasteData> = None;
        if data.is_some() {
            if kind == 0 { taste = TasteData::get(&data.unwrap().taste1.to_string()); }
            else if kind == 1 {
                taste = TasteData::get(&data.unwrap().taste2.to_string());
            }
            else if kind == 2 {
                taste = TasteData::get(&data.unwrap().taste3.to_string());
            }
        }
        if taste.is_some() {
            let grade = taste.unwrap().get_grade();
            let name = taste.unwrap().get_name();
            return concat_strings3(grade, " - ".into(), name, None);
        }
        return "N/A".into();
    }
}
// Need to rewrite this. so bad :(
pub fn get_cook_taste_description(pid: &Il2CppString, kind: i32) -> &'static Il2CppString {
    unsafe {
    let data = CookData::get(&pid.to_string());
    let mut taste: Option<&TasteData> = None;
    if data.is_some() {
        if kind == 0 { taste = TasteData::get(&data.unwrap().taste1.to_string()); }
        else if kind == 1 { taste = TasteData::get(&data.unwrap().taste2.to_string()); }
        else if kind == 2 { taste = TasteData::get(&data.unwrap().taste3.to_string()); }
    }
    if taste.is_some() {
        let tasty = taste.unwrap();
        let condition = TasteConditionData::get(&tasty.cid.to_string());
        if condition.is_some() {
            let label = condition.unwrap().get_name();
            if label.is_some() {
                let bonus: &Il2CppString = format!("{},", label.unwrap().to_string()).into();
                if tasty.augment > 0 {
                    let aug1: &Il2CppString = format!(" +{} dish stats", tasty.augment).into();
                    if tasty.other_enhance > 0 {
                        let aug2: &Il2CppString = format!("/+{} one non-dish Stat", tasty.other_enhance).into();
                        return concat_strings3(bonus, aug1, aug2, None);
                    }
                    else if tasty.other_enhance < 0 {
                        let aug2: &Il2CppString = format!("/{}", tasty.other_enhance).into();
                        return concat_strings3(bonus, aug1, aug2, None);
                    }
                    else { return concat_strings(bonus, aug1, None); }
                }
                else if tasty.augment < 0 {
                    let aug1: &Il2CppString = format!(" {} dish stats", tasty.augment).into();
                    if tasty.other_enhance > 0 {
                        let aug2: &Il2CppString = format!("/+{} one non-dish Stat", tasty.other_enhance).into();
                        return concat_strings3(bonus, aug1, aug2, None);
                    }
                    else if tasty.other_enhance < 0 {
                        let aug2: &Il2CppString = format!("/{} one non-dish Stat", tasty.other_enhance).into();
                        return concat_strings3(bonus, aug1, aug2, None);
                    }
                    else { return concat_strings(bonus, aug1, None); }
                }
                if tasty.other_enhance > 0 {
                    let aug1: &Il2CppString = format!(" +{} one non-dish Stat", tasty.other_enhance).into();
                    return concat_strings(bonus, aug1, None);
                }
                else if tasty.other_enhance < 0 {
                    let aug1: &Il2CppString = format!(" {} one non-dish Stat", tasty.other_enhance).into();
                    return concat_strings(bonus, aug1, None);
                }
                else {
                    let mut count = 0;
                    let mut stat_str_total: &Il2CppString = " ".into();
                    for i in 0..8 {
                        if tasty.enhanced[i] != 0 {
                            let stat = get_stat_with_value(i, tasty.enhanced[i]);
                            if count == 0 { stat_str_total = stat; }
                            else { stat_str_total = concat_strings3(stat_str_total, ", ".into(), stat, None); }
                            count += 1;
                        }
                    }
                    return concat_strings3(bonus, " ".into(), stat_str_total, None);
                }
            }
            else {
                if tasty.augment > 0 {
                    let aug1: &Il2CppString = format!("+{} dish stats", tasty.augment).into();
                    if tasty.other_enhance > 0 {
                        let aug2: &Il2CppString = format!("/+{} one non-dish Stat", tasty.other_enhance).into();
                        return concat_strings(aug1, aug2, None);
                    }
                    else if tasty.other_enhance < 0 {
                        let aug2: &Il2CppString = format!("/{} one non-dish Stat", tasty.other_enhance).into();
                        return concat_strings(aug1, aug2, None);
                    }
                    else { return aug1; }
                }
                else if tasty.augment < 0 {
                    let aug1: &Il2CppString = format!(" {} dish stats", tasty.augment).into();
                    if tasty.other_enhance > 0 {
                        let aug2: &Il2CppString = format!("/+{} one non-dish Stat", tasty.other_enhance).into();
                        return concat_strings(aug1, aug2, None);
                    }
                    else if tasty.other_enhance < 0 {
                        let aug2: &Il2CppString = format!("/{} one non-dish Stat", tasty.other_enhance).into();
                        return concat_strings(aug1, aug2, None);
                    }
                    else { return aug1; }
                }
                if tasty.other_enhance > 0 {
                    let aug1: &Il2CppString = format!(" +{} one non-dish Stat", tasty.other_enhance).into();
                    return aug1;
                }
                else if tasty.other_enhance < 0 {
                    let aug1: &Il2CppString = format!(" {} one non-dish Stat", tasty.other_enhance).into();
                    return aug1;
                }
                else {
                    let mut count = 0;
                    let mut stat_str_total: &Il2CppString = " ".into();
                    for i in 0..8 {
                        if tasty.enhanced[i] != 0 {
                            let stat = get_stat_with_value(i, tasty.enhanced[i]);
                            if count == 0 { stat_str_total = stat; }
                            else { stat_str_total = concat_strings3(stat_str_total, ", ".into(), stat, None);     }
                            count += 1;
                        }
                    }
                    if tasty.flag.value & 2 == 2 {
                        return concat_strings("No dish stats. ".into(), stat_str_total, None);
                    } 
                    return stat_str_total;
                }
            }
        }
    }
    if kind == 2 { return "Dish will have the Chef's 3rd dish title.".into();  }
        else if kind == 1 { return "Dish will have the Chef's 2nd dish title.".into();  }
        else {  return "Dish will have the Chef's 1st dish title.".into(); }
    }
}

pub fn patch_cook(){
    let result =  GameVariableManager::get_number(COOK_KEY);
    let mut replace = &[0xe1, 0x03, 0x00, 0x2a];
    if result == 0 { replace = &[0xe1, 0x03, 0x00, 0x2a];}
    else if result == 5 { replace = &[0x81, 0x00, 0x80, 0x52];}
    else if result == 4 { replace = &[0x61, 0x00, 0x80, 0x52];}
    else if result == 3 { replace = &[0x41, 0x00, 0x80, 0x52];}
    else if result == 2 { replace = &[0x21, 0x00, 0x80, 0x52];}
    else if result == 1 { replace = &[0x01, 0x00, 0x80, 0x52];}
    Patch::in_text(0x02544808).bytes(replace).unwrap();
    Patch::in_text(0x02544edc).bytes(&[0x21, 0x00, 0x80, 0x52]).unwrap();
}

pub fn is_pid_available(pid: &str) -> bool {
    if let Some(force) = Force::get(ForceType::Absent) {
        if force.iter().any(|f| f.person.pid.to_string().contains(pid)) { return true;}
    }
    return false;
}
fn get_pid_index(index: i32) -> i32 {
    if let Some(force) = Force::get(ForceType::Absent) {
        if let Some(unit) = force.iter().nth(index as usize) {
            let unit_pid = unit.person.pid.to_string();
            if let Some(pos) = crate::character::PID_ARRAY.iter().position(|&pid| unit_pid == pid){
                return pos as i32;
            }
        }
    }
    return 0;
}
fn get_chef_index() -> i32 {
    if let Some(pid) = HubUtil::get_current_cooking_pid() {
        if let Some(force) = Force::get(ForceType::Absent) {
            if let Some(pos) = force.iter().position(|x| x.person.pid == pid) { return pos as i32 + 1;  }
            else { return 0; }
        }
    }
    0
}

pub struct ChefMod;
impl ConfigBasicMenuItemSwitchMethods for ChefMod {
    fn init_content(_this: & mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        unsafe {
            let count = Force::get(ForceType::Absent).and_then(|f| Some(f.get_count()) ).unwrap_or(0);
            if count == 0 { return BasicMenuResult::new(); }
            let toggle = get_chef_index();
            if toggle == 0 { return BasicMenuResult::new();}
            let result = ConfigBasicMenuItem::change_key_value_i(toggle, 1, count, 1);
            let person_index = get_pid_index(result - 1);
            if toggle != result && person_index != 0 {
                println!("Result: {} / {} Person Index: {}", result, count, person_index);
                set_cooking_pid( crate::character::PID_ARRAY[ person_index as usize].into(), None);
                Self::set_command_text(this, None);
                Self::set_help_text(this, None);
                this.update_text();
                return BasicMenuResult::se_cursor();
            } else {return BasicMenuResult::new(); }
        }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
       this.help_text = "The assigned chef.".into(); 
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if let Some(pid) = HubUtil::get_current_cooking_pid() {
            let pcheck = pid.to_string();
            if let Some(pos) = crate::character::PID_ARRAY.iter().position(|&p| pcheck == p){
                this.command_text = Mess::get(crate::character::NAME_ARRAY[pos]);
                return;
            }
        }
        this.command_text = "No Chef".into();  
    }
}
#[skyline::from_offset(0x02a644c0)]
fn set_cooking_pid(value: &Il2CppString, method_info: OptionalMethod);




pub struct CookMod;
impl ConfigBasicMenuItemSwitchMethods for CookMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_cook(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(COOK_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 5, 1);
        if toggle != result {
            GameVariableManager::set_number(COOK_KEY, result);
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            patch_cook();
            return BasicMenuResult::se_cursor();
        } 
        else { return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let mode =  GameVariableManager::get_number(COOK_KEY);
        if mode == 0 {this.help_text = "Dish titles are determined randomly.".into(); }
        else if mode == 5 { this.help_text = "Dish will have the 'Failure' title.".into(); } 
        else if mode == 4 { this.help_text = "Dish will have the 'Ordinary' title.".into(); }
        else if mode == 3 { this.help_text = "Dish will have the Chef's 3rd dish title.".into();  }
        else if mode == 2 { this.help_text = "Dish will have the Chef's 2nd dish title.".into();  }
        else if mode == 1 { this.help_text = "Dish will have the Chef's 1st dish title.".into();  }
        unsafe {
            Mess::load("Cook".into());
            let chef = HubUtil::get_current_cooking_pid();
            if chef.is_some() {
                if mode == 0  { this.help_text = concat_strings("Dish titles are determined randomly. Chef ".into(), pid_to_name(chef.unwrap()), None); return; }
                else if mode > 3 { return; }
                this.help_text = get_cook_taste_description(chef.unwrap(), mode - 1);
            }
        }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let mode =  GameVariableManager::get_number(COOK_KEY);
        Mess::load("Cook".into());
        if mode == 0 { this.command_text = format!("Default").into();  }
        else if mode == 5 { this.command_text = Mess::get("MID_FOODRANK_DefinitelySuffer") } 
        else if mode == 4 { this.command_text = Mess::get("MID_FOODRANK_Normal") } 
        else if mode == 3 { this.command_text = "Dish Title 3".into(); }
        else if mode == 2 { this.command_text = "Dish Title 2".into();  }
        else if mode == 1 { this.command_text = "Dish Title 1".into();  }
        let chef = HubUtil::get_current_cooking_pid();
        if chef.is_some(){
            if mode == 0 || mode > 3 { return; }
            this.command_text = get_cook_taste(chef.unwrap(), mode - 1);
            this.help_text = get_cook_taste_description(chef.unwrap(), mode - 1);
        }
    }
}
#[no_mangle]
extern "C" fn cook() -> &'static mut ConfigBasicMenuItem { 
    unsafe {
        let str12 = concat_strings3( Mess::get("MID_Hub_CafeTerrace_CookMenu"), " ".into(), Mess::get("MID_Hub_CafeTerrace_Cook_Workmanship"), None);
        let menu_item = ConfigBasicMenuItem::new_switch::<CookMod>(str12.to_string());
        menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = cook_build_attribute as _);
        menu_item
    }
 }
 #[no_mangle]
 extern "C" fn chef() -> &'static mut ConfigBasicMenuItem { 
    let menu_item = ConfigBasicMenuItem::new_switch::<ChefMod>("Cooking Chef");
    menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = cook_build_attribute as _);
    menu_item
}
pub fn cook_install(){ 
    cobapi::install_game_setting(chef);
    cobapi::install_game_setting(cook);
}

fn cook_build_attribute(_this: &ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute  {
    if GameUserData::get_sequence() == 2  { return BasicMenuItemAttribute::Hide }
    if GameVariableManager::get_bool("G_Cleared_M005") {  BasicMenuItemAttribute::Enable }
    else { BasicMenuItemAttribute::Hide }

}