use unity::prelude::*;
use crate::il2cpp::object::Array;
use crate::level::*;
use engage::{
    mess::*,
    gamedata::{*, person::PersonDataFlag, item::*, skill::*, unit::*},
};

pub const MID_STATS : &[&str] = &["MID_SYS_HP", "MID_SYS_Str", "MID_SYS_Tec", "MID_SYS_Spd", "MID_SYS_Lck", "MID_SYS_Def", "MID_SYS_Mag", "MID_SYS_Res"];
pub const EFFECTIVE_SIDS : &[&str] = &["SID_馬特効", "SID_鎧特効", "SID_飛行特効", "SID_竜特効", "SID_邪竜特効", "SID_異形特効" ];

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
#[unity::class("App", "UnitAI")]
pub struct UnitAI {
    pub flags: &'static PersonDataFlag,
    pub band: u8,
    pub active: u8,
    pub priority: u8,
    pub heal_rate_a: u8,
    pub heal_rate_b: u8,
    pub battle_rate_type: u8,
    pub prohibit_engage_attack: u8,
    pub prohibit_rod: u8,
    pub prohibit_overlap: u8,
    pub rerewarap_count: u8,
    pub rerewarp_count_max: u8,
    pub rerwarp_last_x: u8,
    pub rerewrap_last_z: u8, 
    __ : i32,
    pub rerewarp_event_flag: &'static Il2CppString,
    pub random_flag: &'static PersonDataFlag,
    pub move_limit: &'static MoveLimitRange,
}

#[unity::class("App", "UnitAI.MoveLimitRange")]
pub struct MoveLimitRange {
    pub m_type: u8,
    pub x: i8,
    pub z: i8,
    pub w: i8,
    pub h: i8,
}

#[skyline::from_offset(0x01f5f570)]
pub fn unit_ai_get_sequence(this: &UnitAI, order: i32, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x01a522a0)]
pub fn unit_get_ai(this: &Unit, method_info: OptionalMethod) -> &'static UnitAI;

#[skyline::from_offset(0x01f5f490)]
pub fn unit_ai_move_limit(this: &UnitAI, method_info: OptionalMethod) -> &'static MoveLimitRange; 

#[unity::from_offset("App", "Unit", "get_X")]
pub fn unit_get_x(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "get_Z")]
pub fn unit_get_z(this: &Unit, method_info: OptionalMethod) -> i32;

fn print_flags(flags: &PersonDataFlag) -> String {
    let mut out = "".into();
    if flags.value & 512 != 0 { out = format!("{}\tReject0Power", out); }
    if flags.value & 8192 != 0 { out = format!("{}\tMoveWithAtk", out); }
    if flags.value & 1024 != 0 { out = format!("{}\tMoveBreak", out); }
    if flags.value & 16384 != 0 { out = format!("{}\t Break", out); }
    if flags.value & 32768 != 0 { out = format!("{}\t Chain", out); }
    if flags.value & 65536 != 0 { out = format!("{}\tLongToShortRange", out); }
    if flags.value & 131072 != 0 { out = format!("{}\tBandActive", out); }
    if flags.value & 262144 != 0 { out = format!("{}\tBandMove", out); }
    if flags.value & 524288 != 0 { out = format!("{}\tBandAttack", out); }
    if flags.value & 16777216 != 0 { out = format!("{}\tTargetAttack", out); }
    if flags.value & 33554432 != 0 { out = format!("{}\tTargetHeal", out); }
    if flags.value & 67108864 != 0 { out = format!("{}\tTargetInterfer", out); }
    return out;
}



#[skyline::hook(offset=0x0215a660)]
pub fn help_param_setter_set_person(this: &HelpParamSetter, frame: u64, unit: &Unit, method_info: OptionalMethod){
    call_original!(this, frame, unit, method_info);

    if unit.force.is_none() { return;}
    if unit.force.unwrap().force_type == 1 || unit.force.unwrap().force_type == 2 {
        unsafe {
            let ai = unit_get_ai(unit, None);
            let mut ai_string = format!("Position: x = {}, z = {}\nSequence:", unit_get_x(unit, None), unit_get_z(unit, None));
            for x in 0..4 {
                let value = unit_ai_get_sequence(ai, x as i32, None).get_string().unwrap();
                ai_string = format!("{}\n{}", ai_string, value);
            }
            ai_string = format!("{}\nFlags ({}): {}", ai_string, ai.flags.value, print_flags(ai.flags));
            ai_string = format!("{}\nBand: {}\tActive: {}\tPriority: {}", ai_string, ai.band, ai.active, ai.priority);
            let move_limit = unit_ai_move_limit(ai, None);
            if move_limit .m_type != 0 {
                let limit_str; 
                if move_limit.m_type == 1 { limit_str = "Move"; }
                else if move_limit.m_type == 2 { limit_str = "Distance"; }
                else if move_limit.m_type == 3 { limit_str = "Rectangle"; }
                else if move_limit.m_type == 4 { limit_str = "Interference Rod"; }
                else { limit_str = "None"; }
                ai_string = format!("{}\nMove Limit Type: {} (x = {}, z = {}, W = {}, H = {})", ai_string, limit_str, move_limit.x, move_limit.z, move_limit.w, move_limit.h)
            }
            let text = tmp_text_get_text(&this.contexts_text, None);
            //let expand_text = concat_strings(text, ai_string.into(), None);
            tmp_text_set_text(&this.contexts_text, ai_string.into(), None);
        }
    }


}
pub fn check_effectiveness(item: &UnitItem) -> i32 {
    let equipped_skills = item.get_equipped_skills();
    if equipped_skills.is_some() {
        let skills = equipped_skills.unwrap();
        for sid in EFFECTIVE_SIDS {
            if skills.find_sid(sid.into()).is_some() {
                let eff = skills.find_sid(sid.into()).unwrap().get_efficacy_value();
                return eff;
            }
        }
    }
    return 0;
}
pub fn get_position_stack(unit: &Unit) -> i32 {
    if unit.has_sid("SID_劇毒".into()) { return 5; }
    else if unit.has_sid("SID_猛毒".into()) { return 3; }
    else if unit.has_sid("SID_毒".into()) { return 1; }    
    return 0;
}
pub fn check_effectiveness_unit(unit: &Unit, item: &UnitItem) -> i32 {
    let item_equip_skills = item.get_equipped_skills();
    if item_equip_skills.is_some() {
        let item_skills = item_equip_skills.unwrap();
            //
        if unit.has_sid("SID_邪竜特効変化".into()) && unit.has_sid("SID_邪竜特効".into()) {
            let mut other_effective = 0; 
            println!("I have Holy Aura");
                //Pure Water Enchant
            if unit.has_sid("SID_異形特効".into()) && unit.has_sid("SID_EN_聖水_発動演出".into()) {
                println!("I have Holy Aura and Pure Water Enchant");
                return SkillData::get("SID_邪竜特効").unwrap().get_efficacy_value() + SkillData::get("SID_異形特効").unwrap().get_efficacy_value();
            }
            for sid in EFFECTIVE_SIDS {
                if *sid == "SID_邪竜特効" { continue; }
                if item_skills.find_sid(sid.into()).is_some() {
                    other_effective = SkillData::get(sid).unwrap().get_efficacy_value();
                    println!("I have Holy Aura and Weapon Effectiviness");
                }
            }
            return SkillData::get("SID_邪竜特効").unwrap().get_efficacy_value() + other_effective;
        }
        if unit.has_sid("SID_異形特効".into()) && unit.has_sid("SID_EN_聖水_発動演出".into()) {
            return SkillData::get("SID_異形特効").unwrap().get_efficacy_value();
        }
        for sid in EFFECTIVE_SIDS {
            if item_skills.find_sid(sid.into()).is_some() && unit.has_sid(sid.into()) {
                return SkillData::get(sid).unwrap().get_efficacy_value();
            }
        }
    }
    else {
        if unit.has_sid("SID_邪竜特効変化".into()) {
            return SkillData::get("SID_邪竜特効").unwrap().get_efficacy_value();
        }
        if unit.has_sid("SID_異形特効".into()) && unit.has_sid("SID_EN_聖水_発動演出".into()) {
            return SkillData::get("SID_異形特効").unwrap().get_efficacy_value();
        }
    }
    return 0;
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
        let link_god = GodData::get(&link_gid.unwrap().get_string().unwrap() ).unwrap().mid;
        let link_engage = SkillData::get(&link_sid.unwrap().get_string().unwrap() ).unwrap().name.unwrap();
        let link = format!("\nLink: {}  ({}) ", Mess::get(link_engage).get_string().unwrap(), Mess::get(link_god).get_string().unwrap());
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
                let mut eff = check_effectiveness(item.unwrap());
                let eff2 = check_effectiveness_unit(unit, item.unwrap()); 
                if eff == 0 && (eff2 != 0 && eff2 < 4) {
                    eff = eff2;
                }
                if eff == 0 && eff2 >= 4 {
                    eff = eff2 - 2;
                }
                if eff != 0 {
                    let atk_type = tmp_text_get_text(&this.title_atk, None);
                    let atk_value = tmp_text_get_text(&this.value_atk, None).get_string().unwrap();
                    let atk: Result<i32, _> = atk_value.parse();
                    if atk.is_ok() {
                        let base_atk = atk.unwrap() as i32;
                        let effective_atk = (eff-1)*power + base_atk;

                        if eff2 >= 4 {
                            let effective_atk2 = (eff2-eff-1)*power + base_atk;
                            power_string = format!(" {}: {} / {}\n", atk_type.get_string().unwrap(), effective_atk, effective_atk2 ).into();
                        }
                        else {
                            power_string = format!(" {}: {}\n", atk_type.get_string().unwrap(), effective_atk).into();
                            let new_atk_str: &Il2CppString = format!("{}", effective_atk).into();
                          //  tmp_text_set_text(this.efficacy_none, new_atk_str, None);
                           // try_set_active(this.efficacy_none, true, None);
                        }
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
    let person = PersonData::get(&pid.get_string().unwrap());
    if person.is_some() { return Mess::get(person.unwrap().name); }
    else { return "???".into(); } 
}