use skyline::patching::Patch;
use unity::prelude::*;
use std::ops::Deref;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gameuserdata::GameUserData;
use engage::{gamedata::*, singleton::SingletonClass};
use engage::{gamedata::person::*, gamedata::unit::*};
use engage::force::Force;
use crate::string::*;
//Character mod
pub const CHARACTER_KEY: &str = "G_CHARACTER";
pub static mut setPerson: bool = false;
pub const NameArray : &[&str] = &["MPID_Lueur", "MPID_Vandre", "MPID_Clan", "MPID_Fram", "MPID_Alfred", "MPID_Etie", "MPID_Boucheron", "MPID_Celine", "MPID_Chloe", "MPID_Louis", "MPID_Yunaka", "MPID_Staluke", "MPID_Citrinica", "MPID_Lapis", "MPID_Diamand", "MPID_Umber", "MPID_Jade", "MPID_Ivy", "MPID_Kagetsu", "MPID_Zelkova", "MPID_Fogato", "MPID_Pandoro", "MPID_Bonet", "MPID_Misutira", "MPID_Panetone", "MPID_Merin", "MPID_Hortensia", "MPID_Seadas", "MPID_Rosado", "MPID_Goldmary", "MPID_Linden", "MPID_Saphir", "MPID_Veyre", "MPID_Mauve", "MPID_Anna", "MPID_Jean" ];
pub const OptionArray : &[&str] = &["None", "Vander", "Clanne", "Framme", "Alfred", "Etie", "Boucheron", "Céline", "Chloé", "Louis", "Yunaka", "Alcryst", "Citrinne", "Lapis", "Diamant", "Amber", "Jade", "Ivy", "Kagetsu", "Zelkov", "Fogato", "Pandero", "Bunet", "Timerra", "Panette", "Merrin", "Hortensia", "Seadall", "Rosado", "Goldmary", "Linden", "Saphir", "Veyle", "Mauvier", "Anna", "Jean"];
pub const HelpArray : &[&str] = &["MPID_H_Lueur", "MPID_H_Vandre", "MPID_H_Clan", "MPID_H_Fram", "MPID_H_Alfred", "MPID_H_Etie", "MPID_H_Boucheron", "MPID_H_Celine", "MPID_H_Chloe", "MPID_H_Louis", "MPID_H_Yunaka", "MPID_H_Staluke", "MPID_H_Citrinica", "MPID_H_Lapis", "MPID_H_Diamand", "MPID_H_Umber", "MPID_H_Jade", "MPID_H_Ivy", "MPID_H_Kagetsu", "MPID_H_Zelkova", "MPID_H_Fogato", "MPID_H_Pandoro", "MPID_H_Bonet", "MPID_H_Misutira", "MPID_H_Panetone", "MPID_H_Merin", "MPID_H_Hortensia", "MPID_H_Seadas", "MPID_H_Rosado", "MPID_H_Goldmary", "MPID_H_Linden", "MPID_H_Saphir", "MPID_H_Veyre", "MPID_H_Mauve", "MPID_H_Anna", "MPID_H_Jean" ];
pub const PIDArray : &[&str] = &["PID_リュール", "PID_ヴァンドレ", "PID_クラン", "PID_フラン", "PID_アルフレッド", "PID_エーティエ", "PID_ブシュロン", "PID_セリーヌ", "PID_クロエ", "PID_ルイ", "PID_ユナカ", "PID_スタルーク", "PID_シトリニカ", "PID_ラピス", "PID_ディアマンド", "PID_アンバー", "PID_ジェーデ", "PID_アイビー", "PID_カゲツ", "PID_ゼルコバ", "PID_フォガート", "PID_パンドロ", "PID_ボネ", "PID_ミスティラ", "PID_パネトネ", "PID_メリン", "PID_オルテンシア", "PID_セアダス", "PID_ロサード", "PID_ゴルドマリー", "PID_リンデン", "PID_ザフィーア", "PID_ヴェイル", "PID_モーヴ", "PID_アンナ", "PID_ジャン"];
pub static mut playerIndex: usize = 0;
pub static mut summonStart: usize = 0;
pub static mut summonEnds: usize = 0;

#[skyline::from_offset(0x02616200)]
pub fn Force_Get(forceType:  i32, method_info: OptionalMethod) -> &'static Force;

#[skyline::from_offset(0x01f73e50)]
pub fn set_gender(this: &UnitEdit, gender: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01f73bdc)]
pub fn unit_edit_set_name(this: &UnitEdit, name: &Il2CppString, method_info: OptionalMethod);


//Get and Set Alear Gender
pub fn get_lueur_gender(){
    unsafe {
        if GameVariableManager::get_number(CHARACTER_KEY) == 0 && playerIndex == 0 {
            GameVariableManager::make_entry("G_Lueur_Gender".into(), 0);
            GameVariableManager::make_entry("G_Lueur_Name".into(), 0);
            for f in 0..7 {
                if f == 5 { continue; }
                let force = Force_Get(f, None);
                let mut force_iter = Force::iter(force);
                while let Some(unit) = force_iter.next() {
                    if unit.person.pid.get_string().unwrap() == "PID_リュール" {
                        if unit.edit.name.is_some(){
                            if unit.edit.gender != 0 {
                                if unit.edit.gender > 2 {
                                    set_gender(unit.edit, 1, None);
                                }
                                GameVariableManager::set_number("G_Lueur_Gender".into(), unit.edit.gender);
                                GameVariableManager::set_string("G_Lueur_Name".into(), unit.edit.name.unwrap());
                               // println!("Lueur Name is set! Gender = {}/{}", unit.edit.gender, GameVariableManager::get_number("G_Lueur_Gender".into()));
                               // println!("to {}",  GameVariableManager::get_string("G_Lueur_Name".into()).get_string().unwrap());
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
        return; }
    unsafe {
        let index = GameVariableManager::get_number(CHARACTER_KEY) as usize;
    for f in 0..7 {
        if f == 5 { continue; }
        let force = Force_Get(f, None);
        let mut force_iter = Force::iter(force);
        while let Some(unit) = force_iter.next() {
        if unit.person.pid.get_string().unwrap() == "PID_リュール" {
            if unit.edit.name.is_some(){
                if gender == 0 {
                    set_gender(unit.edit, GameVariableManager::get_number("G_Lueur_Gender".into()), None);
                    let lueurName = GameVariableManager::get_string("G_Lueur_Name".into());
              //      println!("Lueur Name: {} is to {}", unit.person.name.get_string().unwrap(), lueurName .get_string().unwrap());
                  unit_edit_set_name(unit.edit, lueurName, None);
                }
                else { 
                    set_gender(unit.edit, gender, None);
                    let name = Mess_Get( NameArray[index].into(), None);
                    unit_edit_set_name(unit.edit, name, None);
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
    fn init_content(this: & mut ConfigBasicMenuItem){
        let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
        let t_list = &triabolical.list.items;
        let seted = unsafe { setPerson };
        if seted == false { set_Person(); }
        unsafe { setPerson = true; }
        changeCharacters();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(CHARACTER_KEY, 0);
        let toggle =  GameVariableManager::get_number(CHARACTER_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 35, 1);

        if toggle != result {
            GameVariableManager::set_number(CHARACTER_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            changeCharacters();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(CHARACTER_KEY);
        if typeC == 0 { this.help_text = format!("Characters are set their default appearance/growths.").into();  }
        else { this.help_text = format!("Characters are set to the selected character.").into();   }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC: usize = (GameVariableManager::get_number(CHARACTER_KEY)).try_into().unwrap();
        if typeC != 0 {
            unsafe {
            this.command_text = Mess_Get(NameArray[typeC].into(), None);
            }
        }
        else { 
            this.command_text = OptionArray[typeC].into()
        }
    }
}

#[skyline::from_offset(0x02487990)]
pub fn skillarray_find(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> Option<u64>;

#[skyline::from_offset(0x02482850)]
pub fn skillarray_remove(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02482850)]
pub fn skillarray_add(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

pub fn set_Person(){
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    let seted = unsafe { setPerson };
    if seted == false {
        unsafe {
            for x in 0..PIDArray.len() {
                let person = PersonData::get(PIDArray[x]);
                match person {
                    Some(p) => { 
                        set_person(p, t_list[902+x], false); 
                        match skillarray_find(get_CommonSkill(p, None), "SID_主人公".into(), None) {
                            Some(i) => {  playerIndex = x; 
                                println!("Protag Skill found on person #{} - {}", x, NameArray[x]);
                            }
                            None => {}
                        }
                    },
                    None => {}
                }
            }
        }
        set_summons();
        println!("Summons and persons are set");
    }
    unsafe { setPerson = true; }
}
pub fn person_set_Jid(this: &PersonData, value: &str, method_info: OptionalMethod){
    unsafe {
        set_Jid(this, value.into(), method_info);
    }
}

pub fn set_summons(){
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    unsafe {
        for x in 1300..1500 {
            if get_SummonRank(t_list[x], None) != 0{
                let jobName = get_jid(t_list[x], None).get_string().unwrap();
                if jobName == "JID_ソードファイター" { 
                    if get_Gender(t_list[x], None) == 1 {
                        set_person(PersonData::get(PIDArray[7]).unwrap(), t_list[x], true); //Noble Celine
                        person_set_Jid(t_list[x], "JID_フロラージュ下級", None);
                    }
                    else { set_person(PersonData::get(PIDArray[13]).unwrap(), t_list[x], true); } // Lapis
                }
                else if jobName == "JID_ソードペガサス" || jobName == "JID_ランスペガサス" || jobName== "JID_アクスペガサス" || jobName == "JID_グリフォンナイト" { set_person(PersonData::get(PIDArray[8]).unwrap(), t_list[x], true); } // Flier
                else if jobName == "JID_ソードアーマー" { set_person(PersonData::get(PIDArray[14]).unwrap(), t_list[x], true); person_set_Jid(t_list[x], "JID_スュクセサール下級", None); } //Lord Diamant
                else if jobName == "JID_ランスアーマー" { set_person(PersonData::get(PIDArray[9]).unwrap(), t_list[x], true);}     //Lance Armor
                else if jobName == "JID_アクスアーマー" { set_person(PersonData::get(PIDArray[16]).unwrap(), t_list[x], true);}     //Axe Armor
                else if jobName == "JID_アクスファイター" {
                    if get_Gender(t_list[x], None) == 1 { set_person(PersonData::get(PIDArray[6]).unwrap(), t_list[x], true);  }    //Axe Fighter Boucheron
                    else { set_person(PersonData::get("PID_アンナ").unwrap(), t_list[x], true); }     //Axe Figher Female Anna
                }
                if jobName == "JID_ランスファイター" {
                    if get_Gender(t_list[x], None) == 1 {
                        set_person(PersonData::get(PIDArray[4]).unwrap(), t_list[x], true); 
                        person_set_Jid(t_list[x], "JID_アヴニール下級", None);  //Noble Alfred
                    }
                    else {
                        set_person(PersonData::get(PIDArray[23]).unwrap(), t_list[x], true);
                        person_set_Jid(t_list[x], "JID_ピッチフォーク下級", None);  //Setenial Timerra
                    }
                }
                if jobName == "JID_ソードナイト" { set_person(PersonData::get(PIDArray[25]).unwrap(), t_list[x], true);  } //Sword Cav
                if jobName == "JID_ランスナイト" { set_person(PersonData::get(PIDArray[15]).unwrap(), t_list[x], true);  } //Lance Cav
                if jobName == "JID_アクスナイト" { set_person(PersonData::get(PIDArray[1]).unwrap(), t_list[x], true);  } //Axe Cav
                if jobName == "JID_アーチャー" {    //Archer  
                    if get_Gender(t_list[x], None) == 1 {
                        set_person(PersonData::get(PIDArray[11]).unwrap(), t_list[x], true); 
                        person_set_Jid(t_list[x], "JID_ティラユール下級", None);  //Lord Alcryst
                    }
                    else { set_person(PersonData::get(PIDArray[5]).unwrap(), t_list[x], true); }
                }
                if jobName == "JID_マージ" { //mage
                    if get_Gender(t_list[x], None) == 1 { set_person(PersonData::get(PIDArray[2]).unwrap(), t_list[x], true);  }
                    else { set_person(PersonData::get(PIDArray[12]).unwrap(), t_list[x], true); }
                }
            if jobName == "JID_シーフ" { //thief
                if get_SummonRank(t_list[x], None) == 2 {
                    set_person(PersonData::get("PID_ヴェイル").unwrap(), t_list[x], true);
                    person_set_Jid(t_list[x], "JID_邪竜ノ娘", None);    //Fell Dragon Veyle
                }
                else if get_Gender(t_list[x], None) == 1 { set_person(PersonData::get(PIDArray[19]).unwrap(), t_list[x], true);  }
                else { set_person(PersonData::get(PIDArray[10]).unwrap(), t_list[x], true); }
            }
            if jobName == "JID_モンク" { //martial monk
                if get_Gender(t_list[x], None) == 1 { set_person(PersonData::get("PID_ジャン").unwrap(), t_list[x], true);  }
                else { set_person(PersonData::get(PIDArray[3]).unwrap(), t_list[x], true); }
            }
            if jobName == "JID_ロイヤルナイト" { set_person(PersonData::get("PID_モーヴ").unwrap(), t_list[x], true);  }
            if jobName == "JID_スナイパー" {    //Sniper 
                if get_Gender(t_list[x], None) == 1 {
                    set_person(PersonData::get(PIDArray[11]).unwrap(), t_list[x], true); 
                    person_set_Jid(t_list[x], "JID_ティラユール", None);  //Lord Alcryst
                }
                else { set_person(PersonData::get(PIDArray[5]).unwrap(), t_list[x], true); }
            }
            if jobName == "JID_ボウナイト" {    // bowknight
                set_person(PersonData::get(PIDArray[20]).unwrap(), t_list[x], true);  
                person_set_Jid(t_list[x], "JID_クピードー", None);
            }
            if jobName == "JID_ウルフナイト" { set_person(PersonData::get(PIDArray[25]).unwrap(), t_list[x], true);  }    //wolf knight
            if jobName == "JID_セイジ" {    //Sage
                if get_Gender(t_list[x], None) == 1 { set_person(PersonData::get(PIDArray[30]).unwrap(), t_list[x], true); }
                else {
                    set_person(PersonData::get(PIDArray[7]).unwrap(), t_list[x], true); //Vidame Celine
                    person_set_Jid(t_list[x], "JID_フロラージュ", None);
                }
            }
            if jobName == "JID_マージナイト" {
                set_person(PersonData::get(PIDArray[17]).unwrap(), t_list[x], true); //Ivy
                person_set_Jid(t_list[x], "JID_リンドブルム", None);
            }
            if jobName == "JID_ハイプリースト" {    //High Priest
                if get_Gender(t_list[x], None) == 1 { set_person(PersonData::get(PIDArray[21]).unwrap(), t_list[x], true); }
                else {
                    set_person(PersonData::get(PIDArray[26]).unwrap(), t_list[x], true); //Hortensia
                    person_set_Jid(t_list[x], "JID_スレイプニル", None);
                }
            }
            if jobName == "JID_パラディン" { set_person(PersonData::get(PIDArray[1]).unwrap(), t_list[x], true);  }    //paladin
            if jobName == "JID_ドラゴンナイト" { set_person(PersonData::get(PIDArray[28]).unwrap(), t_list[x], true);  } // Wyvern
            if jobName == "JID_ソードマスター" { set_person(PersonData::get(PIDArray[18]).unwrap(), t_list[x], true);  } //Swordmastah
            if jobName == "JID_ベルセルク" { set_person(PersonData::get(PIDArray[24]).unwrap(), t_list[x], true);  } //Besersker
            if jobName == "JID_ウォーリアー" { set_person(PersonData::get(PIDArray[31]).unwrap(), t_list[x], true);  } //Warrior
            if jobName == "JID_ブレイブヒーロー" { set_person(PersonData::get(PIDArray[29]).unwrap(), t_list[x], true);  } //Hero
            if jobName == "JID_ハルバーディア" { set_person(PersonData::get(PIDArray[23]).unwrap(), t_list[x], true); person_set_Jid(t_list[x], "JID_ピッチフォーク", None);  }
            if jobName == "JID_グレートナイト"  { set_person(PersonData::get(PIDArray[22]).unwrap(), t_list[x], true); } // Great Knight
            if jobName == "JID_マスターモンク" { set_person(PersonData::get(PIDArray[27]).unwrap(), t_list[x], true); person_set_Jid(t_list[x], "JID_ダンサー", None);  } //Martial Master
            if jobName == "JID_ジェネラル" { set_person(PersonData::get("PID_マデリーン").unwrap(), t_list[x], true); } //Madeline

        } 
    }
}
}
#[skyline::from_offset(0x1f25e60)]
pub fn person_get_AssetForce(this: &PersonData, method_info: OptionalMethod) -> i32;

pub fn changeCharacters(){
    GameVariableManager::make_entry_norewind(CHARACTER_KEY, 0);
    let result =  GameVariableManager::get_number(CHARACTER_KEY);
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    set_Person();
    if result == 0 {
        unsafe {
            //Default!
            for x in 0..PIDArray.len() {
                let person = PersonData::get(PIDArray[x]);
                match person {
                    Some(p) => {
                        set_person(t_list[902+x], p, false); 
                        set_name(p, NameArray[x].into(), None);
                        set_help(p, HelpArray[x].into(), None);
                    },
                    None => {}
                }
            }
        }
        println!("Characters are set to default.");
        set_lueur_gender(0);
    }
    else {
        let index: usize = (902 + result).try_into().unwrap();
        let index2: usize = (result).try_into().unwrap();
        let name = NameArray[index2];
        let help = HelpArray[index2];
        unsafe {
            for x in 0..PIDArray.len() {
                let person = PersonData::get(PIDArray[x]);
                match person {
                    Some(p) => {
                        set_person(t_list[index], p, false); 
                        set_name(p, name.into(), None);
                        set_help(p, help.into(), None);
                        let sids = get_CommonSkill(p, None);
                        //Remove Protag Skill
                        if x != playerIndex { if skillarray_remove(sids, "SID_主人公".into(), None) { set_CommonSkill(p, sids, None); } }
                    },
                    None => {}
                }
            }
            let skill_array_lueur = get_CommonSkill(t_list[902+playerIndex], None);
            let person = PersonData::get(PIDArray[playerIndex]);
            match person {
                Some(p) => {set_CommonSkill(p, skill_array_lueur, None);}
                None => {}
            }
            println!("Characters are set to {}", name);
            set_lueur_gender(get_Gender(t_list[index], None));
        }
    }
    set_summons();
}

#[unity::from_offset("App", "PersonData", "get_SummonColor")]
pub fn get_SummonColor(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "get_SummonRank")]
pub fn get_SummonRank(this: &PersonData, method_info: OptionalMethod) -> i32;

extern "C" fn char_callback() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CharacterMod>("Single Character Mode")}
pub fn char_install(){ cobapi::install_game_setting(char_callback); }
