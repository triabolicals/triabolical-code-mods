use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::level::*;
use crate::string::*;
use crate::skill::*;
use skyline::patching::Patch;
use engage::gamevariable::*;
use engage::{
    mess::*,
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamedata::{*, person::PersonDataFlag, item::*, unit::*},
};

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
    junk: [u8; 3],
    pub rerewrap_event_flag: &'static Il2CppString,
    pub random_flag: &'static PersonDataFlag,
    pub move_limit: &'static MoveLimitRange,
    bleh: u64,  //vstype and bulletPattern u8s
    pub sequence: &'static Array<&'static Il2CppString>,
    pub value: &'static Array<&'static AIValue>,
    pub unit: &'static Unit,
    vs_think: i32,
}

#[unity::class("App", "UnitAI.MoveLimitRange")]
pub struct MoveLimitRange {
    pub m_type: u8,
    pub x: i8,
    pub z: i8,
    pub w: i8,
    pub h: i8,
}

#[unity::class("App", "AIValue")]
pub struct AIValue {
    pub v8_1: i8,
    pub v8_2: i8,
    pub v16: i16,
}

#[unity::from_offset("System", "String", "Contains")]
pub fn string_contains(this: &Il2CppString, value: &Il2CppString, method_info: OptionalMethod) -> bool;

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
#[skyline::from_offset(0x027b2d50)]
pub fn ai_value_get_flag_value(this: &AIValue, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x027b2850)]
pub fn ai_value_get_person(this: &AIValue, method_info: OptionalMethod) -> &'static PersonData;

#[unity::from_offset("App", "Unit","get_PrivateSkill")]
pub fn unit_get_private_skill(this: &Unit, method_info: OptionalMethod) -> &'static SkillArray;

#[unity::from_offset("App", "Unit","get_MaskSkill")]
pub fn unit_get_mask_skill(this: &Unit, method_info: OptionalMethod) -> &'static SkillArray;

fn print_ai_value(this: &UnitAI, order: i32) -> String {
    unsafe {
        let mut out = "".into();
        let values = this.value;
        let seq =  unit_ai_get_sequence(this, order, None);
        let sequence = unit_ai_get_sequence(this, order, None).to_string();
        if string_contains(unit_ai_get_sequence(this, order, None), "AI_AC_Flag".into(), None) {
            for y in 0..4 {
                let index: usize = ( order * 4 + y ) as usize;
                let ai_value = values[ index ]; 
                let nvalue = ai_value_get_value(ai_value, None);
                if y == 0 && nvalue != -1 {
                    let flag_value = ai_value_get_flag_value(ai_value, None);
                    if flag_value == 0 { out = format!("{}\tFlag #{}: False", out, nvalue);  }
                    else if flag_value == 1 { out = format!("{}\tFlag #{}: True", out, nvalue);  }
                    else { out = format!("{}\tFlag #{}: value={}", out, nvalue, flag_value);  }
                }
                else if y == 1 && string_contains(seq, "Turn".into(), None) {
                    out = format!("{}\tTurn={}", out, nvalue); 
                }
                else if nvalue != -1 { out = format!("{}\t{}", out, nvalue); }
                else { out = format!("{}\t-", out); }
            }
        }
        else if string_contains(seq, "Force".into(), None) {
            for y in 0..4 {
                let index: usize = ( order * 4 + y ) as usize;
                let ai_value = values[ index ]; 
                let nvalue = ai_value_get_value(ai_value, None);
                if y == 0 {
                    if nvalue == 0 { out = format!("{}\tPlayer", out); }
                    if nvalue == 1 { out = format!("{}\tEnemy", out); } 
                    if nvalue == 2 { out = format!("{}\tAlly/Other", out); }
                }
                else if string_contains(seq, "Person".into(), None) && y != 0 {
                    if nvalue != -1 {
                        let person =  ai_value_get_person(ai_value, None);
                        let person_name = Mess::get(person.get_name().unwrap()).to_string();
                        let index = person.parent.index;
                        out = format!("{}\t#{}: {}", out, index, person_name);
                    }
                }
            }
        }
        else if string_contains(seq, "Person".into(), None){
            for y in 0..4 {
                let index: usize = ( order * 4 + y ) as usize;
                let ai_value = values[ index ]; 
                let nvalue = ai_value_get_value(ai_value, None);
                if nvalue > 0 {
                    let person =  ai_value_get_person(ai_value, None);
                    let person_name = Mess::get(person.get_name().unwrap()).to_string();
                    let index = person.parent.index;
                    out = format!("{}\t#{}: {}", out, index, person_name);
                }
            }
        }
        else if order == 3 {    //movement
            for y in 0..4 {
                let index: usize = ( order * 4 + y ) as usize;
                let ai_value = values[ index ]; 
                let nvalue = ai_value_get_value(ai_value, None);
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
                let nvalue = ai_value_get_value(ai_value, None);
                if nvalue != -1 {
                    if ai_value.v8_2 > 0 { 
                        if string_contains(seq, "Treasure".into(), None) || string_contains(seq, "Escape".into(), None)  { out = format!("{}\tPos({}, {})", out, ai_value.v8_1, ai_value.v8_2); }
                        else { out = format!("{}\t({}, {})", out, ai_value.v8_1, ai_value.v8_2); }   
                    }
                    else {
                        if y == 0 && string_contains(seq, "AC_Turn".into(), None) { out = format!("{}\tTurn={}", out, nvalue);  }
                        else { out = format!("{}\t{}", out, nvalue); }
                    }
                }
                else { out = format!("{}\t-", out); }
            }
        }
        return out;
    }
}

const HIDDEN_SIDS: &[&str] = &["SID_死亡回避", "SID_命中０","SID_命中１００", "SID_必殺０", "SID_相手の命中０", "SID_相手の命中１００", "SID_相手の必殺０",  "SID_相手の取得経験値０", "SID_必殺０_オフェンス時", "SID_撃破経験加算５０", "SID_王族", "SID_不動_隠蔽", "SID_命中回避－１０", "SID_命中回避－２０", "SID_相手の防御力無視"];
const HIDDEN_NAMES: &[&str] = &["Avoid Death", "Hit 0", "Hit 100", "Crit 0","Foe Hit 0", "Foe Hit 100", "Foe Crit 0", "0 Exp Earned", "O Crit Offense", "+50 Exp", "Royal", "Anchor", "Hit/Avo-10", "Hit/Avo-20", "Ignore Def/Res"];

pub fn skill_array_hidden_string(skills: &SkillArray) -> String {
    let n_skills = skills.list.size;
    let mut n_print = 0;
    let mut out: String = "".to_string();
    for x in 0..n_skills {
        let skill = skills.list.item[x as usize].get_skill().unwrap();
        if skill.get_flag() & 1 == 0 { continue; }
        let sid = skill.sid.to_string();
        if x > 0 && x % 5 == 0 { out = format!("{}\n", out); }
        if skill.name.is_some() { continue; }
        else {
            let index = HIDDEN_SIDS.iter().position(|&r| r == sid);
            if index.is_none() { out = format!("{}\t{}", out, sid); }
            else {
                let ind = index.unwrap();
                out = format!("{}\t{} ({})", out, sid, HIDDEN_NAMES[ind as usize]);
            }
        }
    }
    return out;
}
pub fn has_hidden_skills(skills: &SkillArray) -> bool {
    let n_skills = skills.list.size;
    let mut n_print = 0;
    let mut out: String = "".to_string();
    for x in 0..n_skills {
        let skill = skills.list.item[x as usize].get_skill().unwrap();
        if skill.get_flag() & 1 == 0 { continue; }
        let sid = skill.sid.to_string();
        if skill.name.is_some() { continue; }
        else {
            let index = HIDDEN_SIDS.iter().position(|&r| r == sid);
            if index.is_none() { continue; }
            else {
                return true;
            }
        }
    }
    return false;
}

#[skyline::from_offset(0x027b2820)]
pub fn ai_value_get_value(this: &AIValue, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01f5f880)]
pub fn unit_ai_get_value(this: &UnitAI, order: i32, index: i32, method_info: OptionalMethod) -> &'static AIValue;

fn mess_to_str(mid: &str) -> String {
    Mess::get(mid).to_string()
}


#[skyline::hook(offset=0x0215a660)]
pub fn help_param_setter_set_person(this: &HelpParamSetter, frame: u64, unit: &Unit, method_info: OptionalMethod){
    call_original!(this, frame, unit, method_info);
    if GameVariableManager::get_number("G_AI") != 1  { 
        unsafe {
            if let Some(god) = get_person_data_link_god(unit.person, None) {
                let text = tmp_text_get_text(&this.contexts_text, None);
                let link = format!("\n{}: {} ", Mess::get("MID_MENU_ENGAGE_LINK"), Mess::get(god.mid));
                let expand_text = concat_strings(text, link.into(), None);
                tmp_text_set_text(&this.contexts_text, expand_text.into(), None);
            }
        }
        return;
    }
    if unit.force.is_none() { return;}
    if unit.force.unwrap().force_type == 1 || unit.force.unwrap().force_type == 2 {
        unsafe {
            let ai = unit_get_ai(unit, None);
            let mut ai_string = format!("PID: #{}, Pos: {}, {}", unit.person.parent.index, unit_get_x(unit, None), unit_get_z(unit, None));
            for x in 0..4 {
                let value = unit_ai_get_sequence(ai, x as i32, None).to_string();
                ai_string = format!("{}\n{}: {}", ai_string, value, print_ai_value(ai, x as i32));
            }
            ai_string = format!("{}\nAI Flags: {}", ai_string, print_flags(ai.flags));
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
            let private = unit_get_private_skill(unit, None);
            let mask = unit_get_mask_skill(unit, None);
            let skill_array = SkillArray::instantiate().unwrap();
            skill_array.ctor(mask);
            skill_array.add_array(private);
            if has_hidden_skills(skill_array) { ai_string = format!("{}\nHidden:{}", ai_string, skill_array_hidden_string(skill_array)); }
            let text = tmp_text_get_text(&this.contexts_text, None);
            //let expand_text = concat_strings(text, ai_string.into(), None);
            if let Some(god) = get_person_data_link_god(unit.person, None) {
                ai_string = format!("{}\n{}: {} ", ai_string, Mess::get("MID_MENU_ENGAGE_LINK"), Mess::get(god.mid));
            }
            tmp_text_set_text(&this.contexts_text, ai_string.into(), None);
        }
    }
    // Support Data
    else if unit.force.unwrap().force_type == 0 {
        unsafe {
            let text = tmp_text_get_text(&this.contexts_text, None);
            let support_cat = get_support_category(unit.person, None);
            let support_data = SupportData::get(support_cat).unwrap();
            if support_cat.to_string() == "デフォルト" {
                let hit_string = format!("{} {}: {}", mess_to_str("MID_MENU_Recall_Reliance_Unit"), mess_to_str("MID_SYS_Hit"), support_data[0].hit);
                tmp_text_set_text(&this.contexts_text, format!("{}\n{}", text.to_string(), hit_string).into(), None);
            }  
            else {
                let rank = ["C", "B", "A", "S"];
                let mut support_text = format!("{} {} / {} / {} {}", mess_to_str("MID_SYS_Hit"), mess_to_str("MID_SYS_Crit"), mess_to_str("MID_SYS_Avo"), mess_to_str("MID_SYS_Secure"), mess_to_str("MID_H_INFO_Param_Correction_Support"));
                for x in 0..support_data.len() {
                    support_text = format!("{}\n{}: {} / {} / {} / {}", support_text, rank[x], support_data[x].hit, support_data[x].crit, support_data[x].avo, support_data[x].secure);
                }
                if let Some(god) = get_person_data_link_god(unit.person, None) {
                    support_text  = format!("{}\n{}: {} ", support_text, Mess::get("MID_MENU_ENGAGE_LINK"), Mess::get(god.mid));
                }
                tmp_text_set_text(&this.contexts_text, format!("{}\n\n{}", text.to_string(), support_text).into(), None);
            } 
        }
    }
}
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
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
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
#[unity::class("App", "MapTerrainInfoSingle")]
pub struct MapTerrainInfoSingle {
    object: u64,
    object_array: u64,
    pub title_text_mesh: &'static Array<&'static TextMeshProUGUI>,
    pub value_text_mesh: &'static Array<&'static TextMeshProUGUI>,
}

#[unity::class("App", "MapTerrainInfoSingle")]
pub struct MapBattleInfoParamSetter {
    junk: [u8; 0x110],
    pub battle_hit: &'static TextMeshProUGUI,
    pub battle_crit: &'static TextMeshProUGUI,
    pub chain_atk: &'static TextMeshProUGUI,
    chain_atk_hit_root: u64,
    pub chain_atk_hit: &'static TextMeshProUGUI,
}

#[skyline::from_offset(0x02c4ea10)]
pub fn set_active_object(this: u64, value: bool, method_info: OptionalMethod);

#[skyline::from_offset(0x01e8d4a0)]
pub fn get_hit_real_ratio(ratio: i32, method_info: OptionalMethod) -> f32;

#[skyline::hook(offset=0x01f05f20)]
pub fn map_battle_info_hook(this: &MapBattleInfoParamSetter, method_info: OptionalMethod){
    call_original!(this, method_info);
    unsafe {
        if GameVariableManager::get_number("G_RNG_TYPE") != 5 { return;  }
        let hit_value = tmp_text_get_text(this.battle_hit, None).to_string();
        let hit: Result<i32, _> = hit_value.parse();
        if hit.is_ok() {
            let base_hit = hit.unwrap() as i32;
            let real_hit = (get_hit_real_ratio(base_hit, None)*100.0 ) as i32;
            let new_hit_str = format!("{}", real_hit);
            tmp_text_set_text(this.battle_hit, new_hit_str.into(), None);
        }
        let hit_value2 = tmp_text_get_text(this.chain_atk_hit, None).to_string();
        let hit2: Result<i32, _> = hit_value2.parse();
        if hit2.is_ok() {
            let base_hit2 = hit2.unwrap() as i32;
            let real_hit2 = ( get_hit_real_ratio(base_hit2, None)*100.0 ) as i32;
            let new_hit_str2 = format!("{}", real_hit2);
            tmp_text_set_text(this.chain_atk_hit, new_hit_str2.into(), None);
        }
    }
}

pub struct AIMod;
impl ConfigBasicMenuItemSwitchMethods for AIMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number("G_AI");
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 2, 1);
        if toggle != result {
            GameVariableManager::set_number("G_AI", result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_ignorance();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = match GameVariableManager::get_number("G_AI") {
            1 => { "Displays unit's AI/support bonuses in unit description text."},
            2 => { "Displays as little information as possible."},
            _ => { "Unit description text will not contain AI/support data." }
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number("G_AI") {
            1 => { "AI/Support"},
            2 => { "Ignorance"},
            _ => { "Normal" },
        }.into();
    }
}

extern "C" fn ai() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<AIMod>("Unit Data Mode")
}

pub fn ai_install(){ cobapi::install_game_setting(ai); }

pub fn patch_ignorance() {
    //return;
    if GameVariableManager::get_number("G_AI") == 2 {
        let w2_0 = &[0x02, 0x00, 0x80, 0x52];
        let ret = &[0xc0, 0x03, 0x5f, 0xd6];
        let pop_up_offsets = [ 0x01f47c90, 0x02997f80, ];
        for x in pop_up_offsets {
            Patch::in_text(x).bytes(ret).unwrap();
        }
        //Patch::in_text(0x02089968).bytes(&[0x00, 0x80, 0x52, 0x20]).unwrap();
        Patch::in_text(0x01c6576c).nop().unwrap();
        Patch::in_text(0x01f9e120).nop().unwrap();
        Patch::in_text(0x023584a0).nop().unwrap();
        Patch::in_text(0x0207b520).bytes(w2_0).unwrap();
        Patch::in_text(0x0207bb7c).bytes(w2_0).unwrap();
        Patch::in_text(0x0207c08c).bytes(w2_0).unwrap();
        Patch::in_text(0x02089830).bytes(&[0x00, 0x00, 0x80, 0x52]).unwrap();
        Patch::in_text(0x02089834).bytes(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    }

    else {
        let w2_0_revert = &[0xe2, 0x17, 0x9f, 0x1a];
        Patch::in_text(0x01f47c90).bytes(&[0xff, 0x03, 0x01, 0xd1]).unwrap();
        Patch::in_text(0x02997f80).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();


        Patch::in_text(0x01c6576c).bytes(&[0xa0, 0x1a, 0x00, 0x54]).unwrap();
        Patch::in_text(0x01f9e120).bytes(&[0xe0, 0x01, 0x00, 0x54]).unwrap();
        Patch::in_text(0x023584a0).bytes(&[0x20, 0x01, 0x00, 0x54]).unwrap();
        Patch::in_text(0x0207b520).bytes(w2_0_revert).unwrap();
        Patch::in_text(0x0207b520).bytes(w2_0_revert).unwrap();
        Patch::in_text(0x0207c08c).bytes(w2_0_revert).unwrap();
        Patch::in_text(0x02089830).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();
        Patch::in_text(0x02089834).bytes(&[0xf7, 0x0b, 0x00, 0xf9]).unwrap();
        //Patch::in_text(0x02089968).bytes(&[0x20, 0x80, 0x52, 0x20]).unwrap();

    }
}
#[skyline::from_offset(0x01f26180)]
fn get_person_data_link_god(this: &PersonData, method_info: OptionalMethod) -> Option<&'static GodData>;
