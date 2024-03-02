use skyline::patching::Patch;
use unity::prelude::*;
use std::ops::Deref;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gameuserdata::GameUserData;
use engage::{gamedata::*, singleton::SingletonClass};
use engage::{gamedata::person::*, gamedata::unit::*};
use engage::force::{ForceType, Force};
use crate::string::*;

//Character mod
pub const CHARACTER_KEY: &str = "G_CHARACTER";
pub static mut setPerson: bool = false;
pub const NameArray : &[&str] = &["MPID_Lueur", "MPID_Vandre", "MPID_Clan", "MPID_Fram", "MPID_Alfred", "MPID_Etie", "MPID_Boucheron", "MPID_Celine", "MPID_Chloe", "MPID_Louis", "MPID_Yunaka", "MPID_Staluke", "MPID_Citrinica", "MPID_Lapis", "MPID_Diamand", "MPID_Umber", "MPID_Jade", "MPID_Ivy", "MPID_Kagetsu", "MPID_Zelkova", "MPID_Fogato", "MPID_Pandoro", "MPID_Bonet", "MPID_Misutira", "MPID_Panetone", "MPID_Merin", "MPID_Hortensia", "MPID_Seadas", "MPID_Rosado", "MPID_Goldmary", "MPID_Linden", "MPID_Saphir", "MPID_Veyre", "MPID_Mauve", "MPID_Anna", "MPID_Jean", "MPID_El", "MPID_Rafale", "MPID_Selestia", "MPID_Gregory", "MPID_Madeline"];
pub const OptionArray : &[&str] = &["None", "Vander", "Clanne", "Framme", "Alfred", "Etie", "Boucheron", "Céline", "Chloé", "Louis", "Yunaka", "Alcryst", "Citrinne", "Lapis", "Diamant", "Amber", "Jade", "Ivy", "Kagetsu", "Zelkov", "Fogato", "Pandero", "Bunet", "Timerra", "Panette", "Merrin", "Hortensia", "Seadall", "Rosado", "Goldmary", "Linden", "Saphir", "Veyle", "Mauvier", "Anna", "Jean"];
pub const HelpArray : &[&str] = &["MPID_H_Lueur", "MPID_H_Vandre", "MPID_H_Clan", "MPID_H_Fram", "MPID_H_Alfred", "MPID_H_Etie", "MPID_H_Boucheron", "MPID_H_Celine", "MPID_H_Chloe", "MPID_H_Louis", "MPID_H_Yunaka", "MPID_H_Staluke", "MPID_H_Citrinica", "MPID_H_Lapis", "MPID_H_Diamand", "MPID_H_Umber", "MPID_H_Jade", "MPID_H_Ivy", "MPID_H_Kagetsu", "MPID_H_Zelkova", "MPID_H_Fogato", "MPID_H_Pandoro", "MPID_H_Bonet", "MPID_H_Misutira", "MPID_H_Panetone", "MPID_H_Merin", "MPID_H_Hortensia", "MPID_H_Seadas", "MPID_H_Rosado", "MPID_H_Goldmary", "MPID_H_Linden", "MPID_H_Saphir", "MPID_H_Veyre", "MPID_H_Mauve", "MPID_H_Anna", "MPID_H_Jean", "MPID_H_El", "MPID_H_Rafale", "MPID_H_Selestia", "MPID_H_Gregory", "MPID_H_Madeline"];
pub const PIDArray : &[&str] = &["PID_リュール", "PID_ヴァンドレ", "PID_クラン", "PID_フラン", "PID_アルフレッド", "PID_エーティエ", "PID_ブシュロン", "PID_セリーヌ", "PID_クロエ", "PID_ルイ", "PID_ユナカ", "PID_スタルーク", "PID_シトリニカ", "PID_ラピス", "PID_ディアマンド", "PID_アンバー", "PID_ジェーデ", "PID_アイビー", "PID_カゲツ", "PID_ゼルコバ", "PID_フォガート", "PID_パンドロ", "PID_ボネ", "PID_ミスティラ", "PID_パネトネ", "PID_メリン", "PID_オルテンシア", "PID_セアダス", "PID_ロサード", "PID_ゴルドマリー", "PID_リンデン", "PID_ザフィーア", "PID_ヴェイル", "PID_モーヴ", "PID_アンナ", "PID_ジャン", "PID_エル", "PID_ラファール", "PID_セレスティア", "PID_グレゴリー", "PID_マデリーン", ];
pub static mut ID: &[&str; 41] = &["001Lueur", "500Vandre", "501Clan", "550Fram", "100Alfred", "152Etie", "101Boucheron", "150Celine", "153Chloe", "102Louis", "253Yunaka", "201Staluke", "252Citrinica", "251Lapis", "200Diamand", "203Umber", "250Jade", "350Ivy", "302Kagetsu", "301Zelkova", "400Fogato", "401Pandoro","402Bonet", "450Misutira", "453Panetone", "452Merin", "351Hortensia", "403Seadas", "303Rosado", "352Goldmary", "304Linden", "254Saphir", "551Veyre", "502Mauve", "552Anna", "103Jean", "099El", "049Il", "553Selestia", "503Gregory", "554Madeline"];
pub static mut playerIndex: usize = 0;
pub static mut summonStart: usize = 0;
pub static mut summonEnds: usize = 0;
pub static mut LIMIT: [i8; 484] = [0; 484];
pub static mut GROW: [u8; 484] = [0; 484];
pub static mut ATTRS: [i32; 125] = [0; 125];

// DLC Check
#[skyline::from_offset(0x029f4270)]
pub fn has_content(content: i32, method_info: OptionalMethod) -> bool;

//Get and Set Alear Gender
pub fn get_lueur_gender(){
    unsafe {
        if GameVariableManager::get_number(CHARACTER_KEY) == 0 && playerIndex == 0 {
            GameVariableManager::make_entry("G_Lueur_Gender".into(), 0);
            GameVariableManager::make_entry("G_Lueur_Name".into(), 0);
            let f_type: [ForceType; 5] = [ForceType::Player, ForceType::Enemy, ForceType::Absent, ForceType::Dead, ForceType::Lost];
            for f in f_type {
                let force = Force::get(f).unwrap();
                let mut force_iter = Force::iter(force);
                while let Some(unit) = force_iter.next() {
                    if unit.person.pid.get_string().unwrap() == "PID_リュール" {
                        if unit.edit.name.is_some(){
                            if unit.edit.gender != 0 {
                                if unit.edit.gender > 2 {
                                    unit.edit.set_gender(1);
                                }
                                GameVariableManager::set_number("G_Lueur_Gender".into(), unit.edit.gender);
                                GameVariableManager::set_string("G_Lueur_Name".into(), unit.edit.name.unwrap());
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub fn set_lueur_gender(gender: i32){
    if GameVariableManager::get_number("G_Lueur_Gender".into()) == 0 { 
        get_lueur_gender();
        return; 
    }
    unsafe {
        let index = GameVariableManager::get_number(CHARACTER_KEY) as usize;
        let f_type: [ForceType; 4] = [ForceType::Player, ForceType::Enemy, ForceType::Absent, ForceType::Dead];
        for f in f_type {
            let force = Force::get(f).unwrap();
            let mut force_iter = Force::iter(force);
            while let Some(unit) = force_iter.next() {
                if unit.person.pid.get_string().unwrap() == "PID_リュール" {
                    if unit.edit.name.is_some(){
                        if gender == 0 {
                            unit.edit.set_gender(GameVariableManager::get_number("G_Lueur_Gender".into()));
                            unit.edit.set_name(GameVariableManager::get_string("G_Lueur_Name".into()));
                        }
                        else { 
                            unit.edit.set_gender(gender);
                            let name = Mess_Get( NameArray[index].into(), None);
                            unit.edit.set_name( name );
                        }
                        return;
                    }
                }
            }   
        }
    }
}
pub struct CharacterMod;
impl ConfigBasicMenuItemSwitchMethods for CharacterMod {
    fn init_content(this: & mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(CHARACTER_KEY, 0);
        let toggle =  GameVariableManager::get_number(CHARACTER_KEY);
        let mut result: i32 = 0;
        unsafe {
            if has_content(0, None) { result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 40, 1); }
            else { result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 35, 1); }
            if toggle != result {
                GameVariableManager::set_number(CHARACTER_KEY, result);
                Self::set_command_text(this, None);
                Self::set_help_text(this, None);
                this.update_text();
                changeCharacters();
                return BasicMenuResult::se_cursor();
            } else {return BasicMenuResult::new(); }
        }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(CHARACTER_KEY);
        if typeC == 0 { this.help_text = "Characters are set their default appearance/growths.".into();  }
        else { this.help_text = "Characters are set to the selected character.".into();   }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC: usize = (GameVariableManager::get_number(CHARACTER_KEY)).try_into().unwrap();
        if typeC != 0 {
            unsafe { this.command_text = Mess_Get(NameArray[typeC].into(), None); }
        }
        else {  this.command_text = OptionArray[typeC].into() }
    }
}
pub fn set_Person(){
    let seted = unsafe { setPerson };
    if seted == false {
        unsafe {
            for x in 0..PIDArray.len() {
                let person = PersonData::get(PIDArray[x]).unwrap();
                let caps = person.get_limit();
                let grow = person.get_grow();
                let flag = person.get_flag();
                ATTRS[3*x] = person.get_gender();
                ATTRS[3*x+1] = person.get_attrs();
                ATTRS[3*x+2] = flag.value;
                for i in 0..11 {
                    LIMIT[11*x+i] = caps[i];
                    GROW[11*x+i] = grow[i];
                }
                match person.get_common_skills().find_sid("SID_主人公".into()) {
                    Some(i) => {  
                        playerIndex = x; 
                        let name = Mess_Get(NameArray[playerIndex].into(), None).get_string().unwrap();
                        println!("Protag Skill found on person #{} - {}: {}", x, NameArray[x], name);
                    }
                    None => {}
                }
            }
        }
    }
    unsafe { setPerson = true; }
}

pub fn changeCharacters(){
    GameVariableManager::make_entry(CHARACTER_KEY, 0);
    let result =  GameVariableManager::get_number(CHARACTER_KEY);
    set_Person();
    if result == 0 {
        unsafe {
            for x in 0..PIDArray.len() {
                let person = PersonData::get(PIDArray[x]).unwrap();
                let caps = person.get_limit();
                let grow = person.get_grow();
                let flag: &mut PersonDataFlag = person.get_flag();
                for i in 0..11 {
                    caps[i] = LIMIT[11*x+i];
                    grow[i] = GROW[11*x+i];
                }
                person.set_unit_icon_id(ID[x].into());
                person.set_gender(ATTRS[3*x]);
                person.set_attrs(ATTRS[3*x+1]);
                person.set_name(NameArray[x].into());
                person.set_ascii_name(substring(NameArray[x].into(), 5, None ));
                person.set_help(HelpArray[x].into());
                flag.value = ATTRS[3*x+2];
                person.set_fid(PIDArray[x].into());
                person.on_complete();
            }
        }
        println!("Characters are set to default.");
        set_lueur_gender(0);
    }
    else {
        let index = result as usize;
        let name = NameArray[index];
        let help = HelpArray[index];
        let current_person = PersonData::get(PIDArray[index]).unwrap();
        current_person.set_name(name.into());
        current_person.set_help(help.into());
        current_person.on_complete();
        let current_person_skills = current_person.get_common_skills();
        unsafe {
            for x in 0..PIDArray.len() {
                if x == index { continue; }
                let person = PersonData::get(PIDArray[x]).unwrap();
                let caps = person.get_limit();
                let grow = person.get_grow();
                person.set_name(name.into());
                person.set_help(help.into());
                person.set_ascii_name(substring(name.into(), 5, None ));
                person.set_fid(PIDArray[index].into());
                person.set_unit_icon_id(ID[index].into());
                current_person.on_complete();
                for i in 0..11 {
                    caps[i] = LIMIT[11*index+i];
                    grow[i] = GROW[11*index+i];
                }
                person.set_gender(ATTRS[3*index]);
                person.set_attrs(ATTRS[3*index+1]);
                person.get_flag().value = ATTRS[3*index+2];
                let skill_array = person.get_common_skills();
                // Protag does not change skills
                if x != playerIndex {
                    skill_array.clear();
                    skill_array.copy(current_person_skills); 
                    skill_array.remove_sid("SID_主人公".into()); // remove protag skill from anyone that isn't the protag
                }
            }
            println!("Characters are set to {}", name);
            set_lueur_gender(current_person.get_gender());
        }
    }
}
#[skyline::from_offset(0x037815b0)]
pub fn substring(this: &Il2CppString, start_index: i32, method_info: OptionalMethod) -> &Il2CppString;

extern "C" fn char_callback() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CharacterMod>("Single Character Mode")}
pub fn char_install(){ cobapi::install_game_setting(char_callback); }
