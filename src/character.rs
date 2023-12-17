use skyline::patching::Patch;
use unity::prelude::*;
use std::ops::Deref;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gameuserdata::GameUserData;
use engage::{gamedata::*, singleton::SingletonClass};
use engage::{gamedata::person::*, gamedata::unit::*};

//Character mod
pub const CHARACTER_KEY: &str = "G_CHARACTER";
pub static mut setPerson: bool = false;
pub const NameArray : &[&str] = &["MPID_Lueur", "MPID_Vandre", "MPID_Clan", "MPID_Fram", "MPID_Alfred", "MPID_Etie", "MPID_Boucheron", "MPID_Celine", "MPID_Chloe", "MPID_Louis", "MPID_Yunaka", "MPID_Staluke", "MPID_Citrinica", "MPID_Lapis", "MPID_Diamand", "MPID_Umber", "MPID_Jade", "MPID_Ivy", "MPID_Kagetsu", "MPID_Zelkova", "MPID_Fogato", "MPID_Pandoro", "MPID_Bonet", "MPID_Misutira", "MPID_Panetone", "MPID_Merin", "MPID_Hortensia", "MPID_Seadas", "MPID_Rosado", "MPID_Goldmary", "MPID_Linden", "MPID_Saphir", "MPID_Veyre", "MPID_Mauve", "MPID_Anna", "MPID_Jean" ];
pub const OptionArray : &[&str] = &["None", "Vander", "Clanne", "Framme", "Alfred", "Etie", "Boucheron", "Céline", "Chloé", "Louis", "Yunaka", "Alcryst", "Citrinne", "Lapis", "Diamant", "Amber", "Jade", "Ivy", "Kagetsu", "Zelkov", "Fogato", "Pandero", "Bunet", "Timerra", "Panette", "Merrin", "Hortensia", "Seadall", "Rosado", "Goldmary", "Linden", "Saphir", "Veyle", "Mauvier", "Anna", "Jean"];
pub const HelpArray : &[&str] = &["MPID_H_Lueur", "MPID_H_Vandre", "MPID_H_Clan", "MPID_H_Fram", "MPID_H_Alfred", "MPID_H_Etie", "MPID_H_Boucheron", "MPID_H_Celine", "MPID_H_Chloe", "MPID_H_Louis", "MPID_H_Yunaka", "MPID_H_Staluke", "MPID_H_Citrinica", "MPID_H_Lapis", "MPID_H_Diamand", "MPID_H_Umber", "MPID_H_Jade", "MPID_H_Ivy", "MPID_H_Kagetsu", "MPID_H_Zelkova", "MPID_H_Fogato", "MPID_H_Pandoro", "MPID_H_Bonet", "MPID_H_Misutira", "MPID_H_Panetone", "MPID_H_Merin", "MPID_H_Hortensia", "MPID_H_Seadas", "MPID_H_Rosado", "MPID_H_Goldmary", "MPID_H_Linden", "MPID_H_Saphir", "MPID_H_Veyre", "MPID_H_Mauve", "MPID_H_Anna", "MPID_H_Jean" ];
pub const PIDArray : &[&str] = &["PID_リュール", "PID_ヴァンドレ", "PID_クラン", "PID_フラン", "PID_アルフレッド", "PID_エーティエ", "PID_ブシュロン", "PID_セリーヌ", "PID_クロエ", "PID_ルイ", "PID_ユナカ", "PID_スタルーク", "PID_シトリニカ", "PID_ラピス", "PID_ディアマンド", "PID_アンバー", "PID_ジェーデ", "PID_アイビー", "PID_カゲツ", "PID_ゼルコバ", "PID_フォガート", "PID_パンドロ", "PID_ボネ", "PID_ミスティラ", "PID_パネトネ", "PID_メリン", "PID_オルテンシア", "PID_セアダス", "PID_ロサード", "PID_ゴルドマリー", "PID_リンデン", "PID_ザフィーア", "PID_ヴェイル", "PID_モーヴ", "PID_アンナ", "PID_ジャン"];
pub static mut playerIndex: usize = 0;

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
        this.command_text = OptionArray[typeC].into();
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
                        set_person(p, t_list[902+x]); 
                        match skillarray_find(get_CommonSkill(p, None), "SID_主人公".into(), None) {
                            Some(i) => { 
                                playerIndex = x; 
                                println!("Protag Skill found on person #{} - {}", x, NameArray[x]);
                            }
                            None => {}
                        }
                    },
                    None => {}
                }
            }
        }
    }
    unsafe { setPerson = true; }
}
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
                        set_person(t_list[902+x], p); 
                        set_name(p, NameArray[x].into(), None);
                        set_help(p, HelpArray[x].into(), None);
                    },
                    None => {}
                }
            }
        }
        println!("Characters are set to default.");
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
                        set_person(t_list[index], p); 
                        set_name(p, name.into(), None);
                        set_help(p, help.into(), None);
                        let sids = get_CommonSkill(p, None);
                        //Remove Protag Skill
                        if x != playerIndex {
                            if skillarray_remove(sids, "SID_主人公".into(), None) { set_CommonSkill(p, sids, None); }
                        }
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
        }
    }
    
}

extern "C" fn char_callback() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CharacterMod>("Single Character Mode")}
pub fn char_install(){
    cobapi::install_game_setting(char_callback);
}
