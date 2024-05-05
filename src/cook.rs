use skyline::patching::Patch;
use unity::prelude::*;
use engage::{
    gamevariable::*,gamedata::*,
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    hub::HubUtil,
    gamedata::cook::*,
    mess::*,
};
pub const COOK_KEY: &str = "G_CHEF";
use crate::string::*;

pub fn get_cook_taste(pid: &Il2CppString, kind: i32) -> &'static Il2CppString {
    unsafe {
        let data = CookData::get(&pid.get_string().unwrap());
        let mut taste: Option<&TasteData> = None;
        if data.is_some() {
            if kind == 0 { taste = TasteData::get(&data.unwrap().taste1.get_string().unwrap()); }
            else if kind == 1 {
                taste = TasteData::get(&data.unwrap().taste2.get_string().unwrap());
            }
            else if kind == 2 {
                taste = TasteData::get(&data.unwrap().taste3.get_string().unwrap());
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
    let data = CookData::get(&pid.get_string().unwrap());
    let mut taste: Option<&TasteData> = None;
    if data.is_some() {
        if kind == 0 { taste = TasteData::get(&data.unwrap().taste1.get_string().unwrap()); }
        else if kind == 1 { taste = TasteData::get(&data.unwrap().taste2.get_string().unwrap()); }
        else if kind == 2 { taste = TasteData::get(&data.unwrap().taste3.get_string().unwrap()); }
    }
    if taste.is_some() {
        let tasty = taste.unwrap();
        let condition = TasteConditionData::get(&tasty.cid.get_string().unwrap());
        if condition.is_some() {
            let label = condition.unwrap().get_name();
            if label.is_some() {
                let bonus: &Il2CppString = format!("{},", label.unwrap().get_string().unwrap()).into();
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
        else if mode == 5 { this.command_text = Mess::get("MID_FOODRANK_DefinitelySuffer").into(); } 
        else if mode == 4 { this.command_text = Mess::get("MID_FOODRANK_Normal").into();  } 
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
        ConfigBasicMenuItem::new_switch::<CookMod>(str12.get_string().unwrap())
    }
 }

pub fn cook_install(){ cobapi::install_game_setting(cook);}
