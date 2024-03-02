use skyline::patching::Patch;
use unity::prelude::*;
use crate::level::*;
use engage::gamedata::{item::*, skill::{SkillData, SkillArray}, unit::*};
use engage::{gamevariable::*,gamedata::*};

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

#[skyline::from_offset(0x025c923c)]
pub fn Mess_Get(label: &Il2CppString, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x025d4410)]
pub fn Mess_Get2(label: &Il2CppString, arg0: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;
#[skyline::from_offset(0x03784a20)]
pub fn to_lower(this: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x025d77c0)]
pub fn Mess_SetArgument(index: i32, value: &Il2CppString, method_info: OptionalMethod); 

#[skyline::from_offset(0x025d3e40)]
pub fn Mess_Load(value: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x028316d0)]
pub fn TMP_Text_get_text(this: &TextMeshProUGUI, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x028317d0)]
pub fn TMP_Text_set_text(this: &TextMeshProUGUI, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x0215a8b0)]
pub fn HelpParamSetter_SetFixedText(this: &HelpParamSetter, frame: u64, text: &Il2CppString, method_info: OptionalMethod);

// MID_SYS_Mt
#[unity::class("App", "HelpParamSetter")]
pub struct HelpParamSetter {
    junk : [u8; 0x50],
    pub m_TitleAtk: &'static TextMeshProUGUI,
    pub m_ValueAtk: &'static TextMeshProUGUI,
    junk2 : [u64; 18],
    pub m_ContextsText: &'static TextMeshProUGUI,
}

pub fn check_effectiveness(item: &UnitItem) -> i32 {
    unsafe {
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
    unsafe {
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
    }
    return 0;
}

#[unity::hook("App","HelpParamSetter", "SetItemData")]
pub fn HelpParamSetter_SetItemData(this: &HelpParamSetter, frame: u64, data: Option<&ItemData>, unit: &Unit, god: &GodUnit, ring: u64, endurance: i32, item: Option<&UnitItem>, isUseEnchant: bool, method_info: OptionalMethod){
    call_original!(this, frame, data, unit, god, ring, endurance, item, isUseEnchant, method_info);
    if data.is_some() && item.is_some() {
        if data.unwrap().usetype == 1  {
            unsafe {
                let power = item.unwrap().get_power();
                let power0: i32  = data.unwrap().power.into();
                let text = TMP_Text_get_text(&this.m_ContextsText, None);
                let mut power_string: &Il2CppString = format!(": {}\n", power0).into();
                if power < power0 { power_string = format!(": {} ({})\n", power, power-power0).into();  }
                else if power0 < power { power_string = format!(": {} (+{})\n", power, power-power0).into(); }

                let might_str = concat_strings(get_mess_str("MID_SYS_Mt"), power_string, None);
                let mut eff = check_effectiveness(item.unwrap());
                let mut eff2 = check_effectiveness_unit(unit, item.unwrap()); 
                if eff == 0 && (eff2 != 0 && eff2 < 4) {
                    eff = eff2;
                }
                if eff == 0 && eff2 >= 4 {
                    eff = eff2 - 2;
                }
                if eff != 0 {
                    let atk_type = TMP_Text_get_text(&this.m_TitleAtk, None);
                    let atk_value = TMP_Text_get_text(&this.m_ValueAtk, None).get_string().unwrap();
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
                        }
                        
                        let final_str = concat_strings4(might_str, get_mess_str("MID_SYS_Eff"), power_string, text, None);
                        TMP_Text_set_text(&this.m_ContextsText, final_str, None);
                        return;
                    }
                }
                let final_str = concat_strings(might_str, text, None);
                TMP_Text_set_text(&this.m_ContextsText, final_str, None);
            }
        }
    }
}
pub fn get_mess_str(string: &str) -> &Il2CppString {
    unsafe { Mess_Get(string.into(), None) }
}
pub fn On_str() -> &'static Il2CppString {
    unsafe { Mess_Get("MID_CONFIG_TUTORIAL_ON".into(), None) }
}
pub fn Off_str() -> &'static Il2CppString {
    unsafe { Mess_Get("MID_CONFIG_TUTORIAL_OFF".into(), None) }
}
// Jan English (US), Spanish(LA), English (EUR), Spanish (Eur), Franch, Itlalian, German, Simply Chinese, Traditional Chinese, Korean
pub fn setting_str(string: &str) -> &'static Il2CppString  {
    unsafe {
        let lang = get_lang(None);
        let mess = Mess_Get(string.into(), None);
        let config = Mess_Get("MID_MENU_CONFIG".into(), None);
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
            let stat_str = get_mess_str(MID_STATS[index]);
            return concat_strings(stat_str, value_str, None);
        }
        else {
            let value_str: &Il2CppString = format!("+{}", value).into();
            let stat_str = get_mess_str(MID_STATS[index]);
            return concat_strings(stat_str, value_str, None);

        }
    }  
}
pub fn pid_to_name(pid: &Il2CppString ) -> &'static Il2CppString {

    let person = PersonData::get(&pid.get_string().unwrap());
    if person.is_some() {
        unsafe { return Mess_Get(person.unwrap().name, None); }
    }
    else { return "???".into(); } 
}