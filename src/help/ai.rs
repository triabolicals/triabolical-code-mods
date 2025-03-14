use unity::prelude::*;
use engage::gamevariable::*;
use engage::{
    mess::*,
    gamedata::{*, unit::*, ai::*},
};
use super::*;

fn print_flags(flags: &WeaponMask) -> String {
    let mut out = "".into();
    if flags.value & 256 != 0 { out = format!("{}\tThinkNoMove", out)}
    if flags.value & 512 != 0 { out = format!("{}\tReject0Power", out); }
    if flags.value & 2048 != 0 { out = format!("{}\tMoveThrough", out); }
    if flags.value & 8192 != 0 { out = format!("{}\tMoveWithAtk", out); }
    if flags.value & 1024 != 0 { out = format!("{}\tMoveBreak", out); }
    if flags.value & 4096 != 0 { out = format!("{}\tMoveSlow", out); }
    if flags.value & 16384 != 0 { out = format!("{}\t Break", out); }
    if flags.value & 32768 != 0 { out = format!("{}\t Chain", out); }
    if flags.value & 1048576 != 0 { out = format!("{}\tAskHealA", out); }
    if flags.value & 2097152 != 0 { out = format!("{}\tAskHealB", out); }
    if flags.value & 65536 != 0 { out = format!("{}\tLongToShortRange", out); }
    if flags.value & 131072 != 0 { out = format!("{}\tNotActivateAtk", out); }
    if flags.value & 262144 != 0 { out = format!("{}\tBandMove", out); }
    if flags.value & 524288 != 0 { out = format!("{}\tBandAttack", out); }
    if flags.value & 4194304 != 0 { out = format!("{}\tIdle", out); }
    if flags.value & 16777216 != 0 { out = format!("{}\tTargetAttack", out); }
    if flags.value & 33554432 != 0 { out = format!("{}\tTargetHeal", out); }
    if flags.value & 67108864 != 0 { out = format!("{}\tTargetInterfer", out); }
    if flags.value & 536870912 != 0 { out = format!("{}\tEngageAtkOnce", out); }
    if flags.value & 1073741824 != 0 { out = format!("{}\tEngageAtkOnceDone", out); }
    return out;
}

fn print_ai_value(this: &UnitAI, order: i32) -> String {
    let mut out = "".into();
    let values = this.value;
    let sequence = this.sequence[order as usize].to_string();
    if sequence.contains("AI_AC_Flag") {
        for y in 0..4 {
            let index: usize = ( order * 4 + y ) as usize;
            let ai_value = values[ index ]; 
            let nvalue = values[ index ].get_value();
            if y == 0 && nvalue != -1 {
                let flag_value = ai_value.get_flag_value();
                if flag_value == 0 { out = format!("{}\tFlag #{}: False", out, nvalue);  }
                else if flag_value == 1 { out = format!("{}\tFlag #{}: True", out, nvalue);  }
                else { out = format!("{}\tFlag #{}: value={}", out, nvalue, flag_value);  }
            }
            else if y == 1 && sequence.contains("Turn") { out = format!("{}\tTurn={}", out, nvalue);  }
            else if nvalue != -1 { out = format!("{}\t{}", out, nvalue); }
            else { out = format!("{}\t-", out); }
        }
    }
    else if sequence.contains("Force") {
        for y in 0..4 {
            let index: usize = ( order * 4 + y ) as usize;
            let ai_value = values[ index ]; 
            let nvalue = values[ index ].get_value();
            if y == 0 {
                if nvalue == 0 { out = format!("{}\tPlayer", out); }
                if nvalue == 1 { out = format!("{}\tEnemy", out); } 
                if nvalue == 2 { out = format!("{}\tAlly/Other", out); }
            }
            else if sequence.contains("Person") && y != 0 {
                if nvalue != -1 {
                    if let Some(person) = ai_value.get_person() {
                        let person_name = Mess::get_name(person.pid);
                        let index = person.parent.index;
                        out = format!("{}\t#{}: {}", out, index, person_name);

                    } 
                }
            }
        }
    }
    else if sequence.contains("Person") {
        for y in 0..4 {
            let index: usize = ( order * 4 + y ) as usize;
            let ai_value = values[ index ]; 
            let nvalue = ai_value.get_value();
            if nvalue > 0 {
                if let Some(person) = ai_value.get_person() {
                    let person_name = Mess::get_name(person.pid);
                    let index = person.parent.index;
                    out = format!("{}\t#{}: {}", out, index, person_name);
                } 
            }
        }
    }
    else if order == 3 {    //movement
        for y in 0..4 {
            let index: usize = ( order * 4 + y ) as usize;
            let ai_value = values[ index ]; 
            let nvalue = ai_value.get_value();
            if nvalue == -1 { out = format!("{}\t-", out); }
            else if ai_value.v8_2 > 0 { 
                if sequence == "AI_MV_TreasureToEscape" && y == 3 { out = format!("{}\tEscape({}, {})", out, ai_value.v8_1, ai_value.v8_2); }
                else { out = format!("{}\tPos({}, {})", out, ai_value.v8_1, ai_value.v8_2); }
            }
            else { out = format!("{}\t{}", out, nvalue); }
        }
    }
    else {
        for y in 0..4 {
            let index: usize = ( order * 4 + y ) as usize;
            let ai_value = values[ index ]; 
            let nvalue = ai_value.get_value();
            if nvalue != -1 {
                if ai_value.v8_2 > 0 { 
                    if sequence.contains("Treasure") || sequence.contains("Escape") { out = format!("{}\tPos({}, {})", out, ai_value.v8_1, ai_value.v8_2); }
                    else { out = format!("{}\t({}, {})", out, ai_value.v8_1, ai_value.v8_2); }   
                }
                else {
                    if y == 0 && sequence.contains("AC_Turn") { out = format!("{}\tTurn={}", out, nvalue);  }
                    else { out = format!("{}\t{}", out, nvalue); }
                }
            }
            else { out = format!("{}\t-", out); }
        }
    }
    return out;
}

#[skyline::hook(offset=0x0215a660)]
pub fn help_param_setter_set_person(this: &mut HelpParamSetter, frame: u64, unit: &Unit, method_info: OptionalMethod){
    call_original!(this, frame, unit, method_info);
    if GameVariableManager::get_number("G_AI") == 1  { 
        if unit.force.is_none() { return; }
        let force = unit.force.unwrap().force_type;
        match force {
            0 => {
                let text = this.contexts_text.get_text();
                let support_cat = unsafe { get_support_category(unit.person, None) };
                let support_data = SupportData::get(support_cat).unwrap();
                if support_cat.to_string() == "デフォルト" {
                    let hit_string = format!("{} {}: {}", Mess::get("MID_MENU_Recall_Reliance_Unit"), Mess::get("MID_SYS_Hit"), support_data[0].hit);
                    this.contexts_text.set_text(format!("{}\n{}", text.to_string(), hit_string).into(), true);
                }  
                else {
                    let rank = ["C", "B", "A", "S"];
                    let mut support_text = format!("{} {} / {} / {} {}", Mess::get("MID_SYS_Hit"), Mess::get("MID_SYS_Crit"), Mess::get("MID_SYS_Avo"), Mess::get("MID_SYS_Secure"), Mess::get("MID_H_INFO_Param_Correction_Support"));
                    for x in 0..support_data.len() {
                        support_text = format!("{}\n{}: {} / {} / {} / {}", support_text, rank[x], support_data[x].hit, support_data[x].crit, support_data[x].avo, support_data[x].secure);
                    }
                    if let Some(god) = unsafe { get_person_data_link_god(unit.person, None) } {
                        support_text  = format!("{}\n{}: {} ", support_text, Mess::get("MID_MENU_ENGAGE_LINK"), Mess::get(god.mid));
                    }
                    this.contexts_text.set_text(format!("{}\n\n{}", text.to_string(), support_text).into(), true);
                } 
            }
            1|2 => {
                let ai = &unit.ai;
                let mut ai_string = format!("PID: #{}, Pos: {}, {}", unit.person.parent.index, unit.x, unit.z);
                for x in 0..4 { ai_string = format!("{}\n{}: {}", ai_string, ai.sequence[x as usize], print_ai_value(ai, x as i32));  }
                ai_string = format!("{}\nAI Flags: {}", ai_string, print_flags(ai.flag));
                ai_string = format!("{}\nBand: {}\tActive: {}\tPriority: {}", ai_string, ai.band, ai.active, ai.priority);
                let move_limit = ai.move_limit;
                if move_limit.m_type != 0 {
                    let limit_str =
                    match move_limit.m_type {
                        1 => { "Move" },
                        2 => { "Distance" },
                        3 => { "Rectance" },
                        4 => { "Interference Rod"},
                        _ => { "None" },
                    };
                    ai_string = format!("{}\nMove Limit Type: {} (x = {}, z = {}, W = {}, H = {})", ai_string, limit_str, move_limit.x, move_limit.z, move_limit.w, move_limit.h)
                }
                if let Some(god) = unsafe { get_person_data_link_god(unit.person, None) } {
                    ai_string = format!("{}\n{}: {} ", ai_string, Mess::get("MID_MENU_ENGAGE_LINK"), Mess::get(god.mid));
                }
                this.contexts_text.set_text(ai_string.into(), true);
                return;
            }
            _ => {}
        }
    }
    add_link_god_text(this, unit);
    // Support Data
}
pub fn add_link_god_text(help: &mut HelpParamSetter, unit: &Unit) {
    if let Some(god) = unsafe { get_person_data_link_god(unit.person, None) } {
        let text = help.contexts_text.get_text().to_string();
        let new_text = format!("{}\n{}: {} ", text, Mess::get("MID_MENU_ENGAGE_LINK").to_string(), Mess::get(god.mid).to_string());
        help.contexts_text.set_text(new_text.into(), true);
    }
}

#[skyline::from_offset(0x02c4ea10)]
pub fn set_active_object(this: u64, value: bool, method_info: OptionalMethod);

#[skyline::from_offset(0x01f26180)]
fn get_person_data_link_god(this: &PersonData, method_info: OptionalMethod) -> Option<&'static GodData>;

#[skyline::from_offset(0x01f25e80)]
fn get_support_category(this: &PersonData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::class("App", "SupportData")]
pub struct SupportData {
    parent: [u8; 0x18],
    pub level: i8,
    pub hit: i8,
    pub crit: i8,
    pub avo: i8,
    pub secure: i8,
}

impl SupportData {
    fn get(name: &Il2CppString) -> Option<&'static mut StructList<Self>> {
        let method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        if method.is_none() {
            return None;
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static mut StructList<Self>>>(
                method.unwrap().method_ptr,
            )
        };
        get_list(name, method.unwrap())
    }
}