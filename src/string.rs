use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::level::*;

use engage::{    
    mess::*,
    gamevariable::*,
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamedata::{*, item::*, skill::*, unit::*},
};

pub const MID_STATS : &[&str] = &["MID_SYS_HP", "MID_SYS_Str", "MID_SYS_Tec", "MID_SYS_Spd", "MID_SYS_Lck", "MID_SYS_Def", "MID_SYS_Mag", "MID_SYS_Res"];
pub const EFFECTIVE_SIDS : Vec<String> = Vec::new();

pub fn gather_effective_sids() {
    if unsafe { EFFECTIVE_SIDS.len() > 0 } { return; }
    SkillData::get_list().unwrap().iter()
        .filter(|skill| skill.get_efficacy_value() > 1 )
        .for_each(|skill|{
            unsafe { EFFECTIVE_SIDS.push( skill.sid.to_string() ) };
        }
    );
}

#[skyline::from_offset(0x01bdbc80)]
pub fn get_lang(method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x037713e0)]
pub fn concat_strings(str0: &Il2CppString, str1: &Il2CppString,method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x037721c0)]
pub fn concat_strings3(str0: &Il2CppString, str1: &Il2CppString, str2: &Il2CppString,method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x037727b0)]
pub fn concat_strings4(str0: &Il2CppString, str1: &Il2CppString, str2: &Il2CppString, str3: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x03784a20)]
pub fn to_lower(this: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x028316d0)]
pub fn tmp_text_get_text(this: &TextMeshProUGUI, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x028317d0)]
pub fn tmp_text_set_text(this: &TextMeshProUGUI, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x0215a8b0)]
pub fn help_param_setter_set_fixed_text(this: &HelpParamSetter, frame: u64, text: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x0290f870)]
pub fn try_set_active(c: &TextMeshProUGUI, is_active: bool, method_info: OptionalMethod);

// MID_SYS_Mt
#[unity::class("App", "HelpParamSetter")]
pub struct HelpParamSetter {
    junk : [u8; 0x50],
    pub title_atk: &'static TextMeshProUGUI,
    pub value_atk: &'static TextMeshProUGUI,
    junk2 : [u64; 13],
    pub efficacy_none: &'static TextMeshProUGUI,
    pub efficacy_icons: &'static Array<u64>,
    pub title_weapon_level: &'static TextMeshProUGUI,
    pub value_weapon_level: &'static TextMeshProUGUI,
    junk3: u64,
    pub contexts_text: &'static TextMeshProUGUI,
}


pub fn check_effectiveness(item: &UnitItem) -> i32 {
    let mut result = 0;
    if let Some(item_equip_skills) = item.get_equipped_skills() {
        item_equip_skills.list.item.iter()
            .for_each(|skill_entity| {
                if let Some(skill) = skill_entity.get_skill() {
                    if skill.get_efficacy_value() > 1 {
                        result =  skill.get_efficacy_value();
                    }
                }
            }
        );
    }
    return result;
}
pub fn get_position_stack(unit: &Unit) -> i32 {
    if unit.has_sid("SID_劇毒".into()) { return 5; }
    else if unit.has_sid("SID_猛毒".into()) { return 3; }
    else if unit.has_sid("SID_毒".into()) { return 1; }    
    return 0;
}
pub fn check_effectiveness_unit(unit: &Unit, item: &UnitItem) -> i32 {
    let mut result = 0;
    let effective = unsafe {&EFFECTIVE_SIDS};
    if let Some(item_equip_skills) = item.get_equipped_skills() {
        item_equip_skills.list.item.iter()
            .for_each(|skill_entity| {
                if let Some(skill) = skill_entity.get_skill() {
                    let eff = skill.get_efficacy_value();
                    if eff > 1 { result |= 1 << eff; }
                }
            }
        );
    }
    effective.iter()
        .for_each(|sid|{
            if unit.has_sid(sid.into()) {
                if let Some(skill) = SkillData::get(sid) {
                    let eff = skill.get_efficacy_value();
                    if eff > 1 { result |= 1 << eff; }
                }
            }
        }
    );
    result 
}
#[unity::from_offset("App", "GodData", "get_LinkGid")]
fn god_data_set_link_gid(this: &GodData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "GodData", "get_EngageAttackLink")]
fn god_data_get_link_engage(this: &GodData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[skyline::hook(offset=0x021621a0)]
pub fn help_param_setter_set_god(this: &HelpParamSetter, frame: u64, god: &GodData, method_info: OptionalMethod){
    call_original!(this, frame, god, method_info);
    unsafe {
        let link_gid = god_data_set_link_gid(god, None);
        if link_gid.is_none() { return; }
        let link_sid = god_data_get_link_engage(god, None);
        if link_sid.is_none() { return; }
        let link_god = GodData::get(&link_gid.unwrap().to_string() ).unwrap().mid;
        let link_engage = SkillData::get(&link_sid.unwrap().to_string() ).unwrap().name.unwrap();
        let link = format!("\nLink: {}  ({}) ", Mess::get(link_engage).to_string(), Mess::get(link_god).to_string());
        let text = tmp_text_get_text(&this.contexts_text, None);
        let expand_text = concat_strings(text, link.into(),None);
        tmp_text_set_text(&this.contexts_text, expand_text, None);
    }
}

#[unity::hook("App","HelpParamSetter", "SetItemData")]
pub fn help_param_setter_set_item_data_hook(this: &HelpParamSetter, frame: u64, data: Option<&ItemData>, unit: &Unit, god: &GodUnit, ring: u64, endurance: i32, item: Option<&UnitItem>, is_use_enchant: bool, method_info: OptionalMethod){
    call_original!(this, frame, data, unit, god, ring, endurance, item, is_use_enchant, method_info);
    if data.is_some() && item.is_some() {
        if data.unwrap().usetype == 1  {
            unsafe {
                let power = item.unwrap().get_power();
                let power0: i32  = data.unwrap().power.into();
                let text = tmp_text_get_text(&this.contexts_text, None);
                let mut power_string: &Il2CppString = format!(": {}\n", power0).into();
                if power < power0 { power_string = format!(": {} ({})\n", power, power-power0).into();  }
                else if power0 < power { power_string = format!(": {} (+{})\n", power, power-power0).into(); }

                let might_str = concat_strings(Mess::get("MID_SYS_Mt"), power_string, None);
                //let mut eff = check_effectiveness(item.unwrap());
                let eff = check_effectiveness_unit(unit, item.unwrap()); 
                if eff > 1 {
                    let atk_type = tmp_text_get_text(&this.title_atk, None);
                    let atk_value = tmp_text_get_text(&this.value_atk, None).to_string();
                    let atk: Result<i32, _> = atk_value.parse();
                    if atk.is_ok() {
                        let base_atk = atk.unwrap() as i32;
                        let mut count = 0;
                        let mut eff_atk_str = "".to_string();
                        for x in 2..6 {
                            if eff & (1 << x) != 0 {
                                let eff_atk = base_atk + (x - 1)*power;
                                if count == 0 {  eff_atk_str = format!("{} ({}x)", eff_atk, x); }
                                else { eff_atk_str = format!("{} / {} ({}x)", eff_atk_str, eff_atk, x); }
                                count +=1;
                            }
                        }
                        power_string = format!(" {}: {} \n", atk_type, eff_atk_str ).into();
                        let final_str = concat_strings4(might_str, Mess::get("MID_SYS_Eff"), power_string, text, None);
                        tmp_text_set_text(&this.contexts_text, final_str, None);
                        return;
                    }
                }
                let final_str = concat_strings(might_str, text, None);
                tmp_text_set_text(&this.contexts_text, final_str, None);
            }
        }
    }
}
pub fn on_str() -> &'static Il2CppString { Mess::get("MID_CONFIG_TUTORIAL_ON") }
pub fn off_str() -> &'static Il2CppString { Mess::get("MID_CONFIG_TUTORIAL_OFF") }

// Jan English (US), Spanish(LA), English (EUR), Spanish (Eur), Franch, Itlalian, German, Simply Chinese, Traditional Chinese, Korean
pub fn setting_str(string: &str) -> &'static Il2CppString  {
    unsafe {
        let lang = get_lang(None);
        let mess = Mess::get(string);
        let config = Mess::get("MID_MENU_CONFIG");
        match lang {
            //English (US)
            1|3|10 => { return concat_strings3(mess, " ".into(),  config, None); },
            //Spanish (AM)
            2|4|5 => { return concat_strings3(config, " de ".into(), mess, None); },
            //Italian
            6 => { return concat_strings3(config, " del ".into(), mess, None); }
            //German
            7 => { return  concat_strings(mess, to_lower(config, None), None); }
            //Sim Chinese
            _ => { return concat_strings3(mess, " ".into(),  config, None); }
        }
    }
}
pub fn get_stat_with_value(index: usize, value: i8) -> &'static Il2CppString {
    unsafe {
        if value < 0 {
            let value_str: &Il2CppString = format!("{}", value).into();
            let stat_str = Mess::get(MID_STATS[index]);
            return concat_strings(stat_str, value_str, None);
        }
        else {
            let value_str: &Il2CppString = format!("+{}", value).into();
            let stat_str = Mess::get(MID_STATS[index]);
            return concat_strings(stat_str, value_str, None);
        }
    }  
}
pub fn pid_to_name(pid: &Il2CppString ) -> &'static Il2CppString {
    let person = PersonData::get(&pid.to_string());
    if person.is_some() { return Mess::get(person.unwrap().get_name().unwrap()); }
    else { return "???".into(); } 
}

#[unity::class("App", "ClassChangeJobMenuContent")]
pub struct ClassChangeJobMenuContent {
    junk: [u8; 0x108],
    pub help_text: &'static TextMeshProUGUI,
}

#[unity::class("App", "ClassChangeChangeJobData")]
pub struct ClassChangeChangeJobData {
    pub job: &'static JobData,
}
#[skyline::hook(offset=0x01ea52d0)]
pub fn set_job_details(this: &ClassChangeJobMenuContent, data: &ClassChangeChangeJobData, method_info: OptionalMethod) {
    call_original!(this, data, method_info);
    match GameVariableManager::get_number("G_JobGrowth") {
        1 => {
            let stats = create_job_growth_string(data.job);
             unsafe {
                let name = format!("{} {}", Mess::get(data.job.name), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
                let final_str = concat_strings3(name.into(), "\n".into(), stats.into(), None);
                tmp_text_set_text(&this.help_text, final_str, None);
            }
        },
        2 => {
            let old_unit = unsafe { class_change_get_unit(None) };
            old_unit.class_change(data.job);
            let stats = unit_total_growths(old_unit);
            unsafe {
                let name = format!("{} {} {}", Mess::get(old_unit.get_job().name), Mess::get_name(old_unit.person.pid), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
                let final_str = concat_strings3(name.into(), "\n".into(), stats.into(), None);
                tmp_text_set_text(&this.help_text, final_str, None);
            }
        },
        3 => {
            let new_unit = unsafe { class_change_get_unit(None) };
            new_unit.class_change(data.job);
            let old_unit = unsafe { class_change_get_unit(None) };
            let stats = unit_diff_growths(old_unit, new_unit);
            unsafe {
                let name = format!("{}: {} -> {}",  Mess::get_name(old_unit.person.pid), Mess::get(old_unit.get_job().name), Mess::get(data.job.name));
                let final_str = concat_strings3(name.into(), "\n".into(), stats.into(), None);
                tmp_text_set_text(&this.help_text, final_str, None);
            }
        },
        _ => {},
    }
}

fn create_job_growth_string(job: &JobData) -> String {
    let mut out = "".to_string();
    let diff = job.get_diff_grow();
    let mut count = 0;
    let stat_order = [0, 1, 6, 2, 3, 4, 5, 7, 8];
    for x in stat_order {
        let stat = if diff[x] != 0 {  format!("{}: {}%", get_stat_label(x), diff[x]) }
            else { format!("{}: -", get_stat_label(x))};
            out = 
                if count == 0 { stat }
                else if count == 4 { format!("{}\n{}", out, stat) }
                else { format!("{}, {}", out, stat) };
            count += 1;
        //}
    }
    return out;
}

fn unit_total_growths(unit: &Unit) -> String {
    let mut out = "".to_string();
    let mut count = 0;
    let stat_order = [0, 1, 6, 2, 3, 4, 5, 7, 8];
    for x in stat_order {
        let stat = format!("{}: {}%", get_stat_label(x as usize), unit.get_capability_grow(x, false));
            out = 
                if count == 0 { stat }
                else if count == 4 { format!("{}\n{}", out, stat) }
                else { format!("{}, {}", out, stat) };
            count += 1;
        //}
    }
    return out;
}

fn unit_diff_growths(before: &Unit, after: &Unit) -> String {
    let mut out = "".to_string();
    let mut count = 0;
    let stat_order = [0, 1, 6, 2, 3, 4, 5, 7, 8];
    for x in stat_order {
        let value = after.get_capability_grow(x, false) - before.get_capability_grow(x, false);
        let stat = format!("{}: {}%", get_stat_label(x as usize), value);
            out = 
                if count == 0 { stat }
                else if count == 4 { format!("{}\n{}", out, stat) }
                else { format!("{}, {}", out, stat) };
            count += 1;
        //}
    }
    return out;

}
pub fn get_stat_label(index: usize) -> String {
    match index {
        0 => { return Mess::get("MID_SYS_HP").to_string();}
        1 => { return Mess::get("MID_SYS_Str").to_string();}
        2 => { return Mess::get("MID_SYS_Tec").to_string();}
        3 => { return Mess::get("MID_SYS_Spd").to_string();}
        4 => { return Mess::get("MID_SYS_Lck").to_string();}
        5 => { return Mess::get("MID_SYS_Def").to_string();}
        6 => { return Mess::get("MID_SYS_Mag").to_string();}
        7 => { return Mess::get("MID_SYS_Res").to_string();}
        8 => { return Mess::get("MID_SYS_Phy").to_string();}
        9 => { return Mess::get("MID_SYS_Vis").to_string();}
        10 => { return Mess::get("MID_SYS_Mov").to_string();}
        11 => { return Mess::get("MID_SYS_Avo").to_string(); }
        12 => { return Mess::get("MID_SYS_Crit").to_string();}
        13 => { return Mess::get("MID_SYS_Hit").to_string();}
        14 => { return  Mess::get("MID_SYS_Mt").to_string(); }
        15 => { return Mess::get("MID_SYS_Secure").to_string(); }
        16 => { return Mess::get("MID_SYS_Weight").to_string(); } 
        _ => { return "".to_string(); }
    }
}

// 0 - Job Help Text
// 1 - Job Growths
// 2 - Unit New Total
// 3 - Net Change


#[unity::class("App", "ClassChangeJobMenuItem")]
pub struct ClassChangeJobMenuItem {
    pub menu: &'static ClassChangeJobMenu,
    junk: [u8; 0x50],
    pub job_data: &'static ClassChangeChangeJobData,
}
#[unity::class("App", "ClassChangeJobMenu")]
pub struct ClassChangeJobMenu {
    menu_stuff: [u8; 0x60],
    pub menu_item_content: &'static mut ClassChangeJobMenuContent,
}
pub fn job_menu_item_x_call(this: &mut ClassChangeJobMenuItem, method_info: OptionalMethod) -> i32 {
    let current_mode = GameVariableManager::get_number("G_JobGrowth");
    let new_mode = ( current_mode + 1 ) % 4;
    GameVariableManager::set_number("G_JobGrowth", new_mode);
    match new_mode {
        1 => {
            let stats = create_job_growth_string(this.job_data.job);
             unsafe {
                let name = format!("{} {}", Mess::get(this.job_data.job.name), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
                let final_str = concat_strings3(name.into(), "\n".into(), stats.into(), None);
                tmp_text_set_text(&this.menu.menu_item_content.help_text, final_str, None);
            }
        },
        2 => {
            let old_unit = unsafe { class_change_get_unit(None) };
            old_unit.class_change(this.job_data.job);
            let stats = unit_total_growths(old_unit);
            unsafe {
                let name = format!("{} {} {}",  Mess::get(old_unit.get_job().name), Mess::get_name(old_unit.person.pid), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
                let final_str = concat_strings3(name.into(), "\n".into(), stats.into(), None);
                tmp_text_set_text(&this.menu.menu_item_content.help_text, final_str, None);
            }
        },
        3 => {
            let new_unit = unsafe { class_change_get_unit(None) };
            new_unit.class_change(this.job_data.job);
            let old_unit = unsafe { class_change_get_unit(None) };
            let stats = unit_diff_growths(old_unit, new_unit);
            unsafe {
                let name = format!("{}: {} -> {}", Mess::get_name(old_unit.person.pid), Mess::get(old_unit.get_job().name), Mess::get(this.job_data.job.name));
                let final_str = concat_strings3(name.into(), "\n".into(), stats.into(), None);
                tmp_text_set_text(&this.menu.menu_item_content.help_text, final_str, None);
            }
        },
        _ => {
            let help_text = Mess::get(this.job_data.job.help);
            unsafe { tmp_text_set_text(&this.menu.menu_item_content.help_text, help_text, None) };
        },
    }
    return 0x80;
}
pub fn job_menu_item_selected(this: &mut ClassChangeJobMenuItem, method_info: OptionalMethod) {
    unsafe { job_menu_item_on_selected(this, method_info); }
    let old_unit = unsafe { class_change_get_unit(None) };
    old_unit.class_change(this.job_data.job);
    crate::accessory::reload_unit_info(old_unit);
}

#[skyline::from_offset(0x019c7ac0)]
pub fn job_menu_item_on_selected(this: &mut ClassChangeJobMenuItem, method_info: OptionalMethod);

#[skyline::from_offset(0x01ea4680)]
pub fn class_change_get_unit(method_info: OptionalMethod) -> &'static Unit;

pub fn add_x_call() {
    let cooking_menu = Il2CppClass::from_name("App", "ClassChangeJobMenu").unwrap().get_nested_types().iter().find(|x| x.get_name() == "ClassChangeJobMenuItem").unwrap();
    let cooking_menu_mut = Il2CppClass::from_il2cpptype(cooking_menu.get_type()).unwrap();
    cooking_menu_mut.get_virtual_method_mut("XCall").map(|method| method.method_ptr = job_menu_item_x_call as _);
    cooking_menu_mut.get_virtual_method_mut("OnSelect").map(|method| method.method_ptr = job_menu_item_selected as _);
    println!("Replaced Virtual Method of ClassChangeJobMenuItem");
}
