use unity::prelude::*;
use engage::{
    gamevariable::*,
    gamedata::{*, dispos::*, unit::*, person::*},
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    mess::*,
    force::{ForceType, Force},
};
use unity::il2cpp::object::Array;

//Character mod
pub const CHARACTER_KEY: &str = "G_CHARACTER";
pub static mut SET_PERSON: bool = false;
pub const NAME_ARRAY : &[&str] = &["MPID_Lueur", "MPID_Vandre", "MPID_Clan", "MPID_Fram", "MPID_Alfred", "MPID_Etie", "MPID_Boucheron", "MPID_Celine", "MPID_Chloe", "MPID_Louis", "MPID_Yunaka", "MPID_Staluke", "MPID_Citrinica", "MPID_Lapis", "MPID_Diamand", "MPID_Umber", "MPID_Jade", "MPID_Ivy", "MPID_Kagetsu", "MPID_Zelkova", "MPID_Fogato", "MPID_Pandoro", "MPID_Bonet", "MPID_Misutira", "MPID_Panetone", "MPID_Merin", "MPID_Hortensia", "MPID_Seadas", "MPID_Rosado", "MPID_Goldmary", "MPID_Linden", "MPID_Saphir", "MPID_Veyre", "MPID_Mauve", "MPID_Anna", "MPID_Jean", "MPID_El", "MPID_Rafale", "MPID_Selestia", "MPID_Gregory", "MPID_Madeline"];
pub const OPTION_ARRAY : &[&str] = &["None", "Vander", "Clanne", "Framme", "Alfred", "Etie", "Boucheron", "Céline", "Chloé", "Louis", "Yunaka", "Alcryst", "Citrinne", "Lapis", "Diamant", "Amber", "Jade", "Ivy", "Kagetsu", "Zelkov", "Fogato", "Pandero", "Bunet", "Timerra", "Panette", "Merrin", "Hortensia", "Seadall", "Rosado", "Goldmary", "Linden", "Saphir", "Veyle", "Mauvier", "Anna", "Jean"];
pub const HELP_ARRAY : &[&str] = &["MPID_H_Lueur", "MPID_H_Vandre", "MPID_H_Clan", "MPID_H_Fram", "MPID_H_Alfred", "MPID_H_Etie", "MPID_H_Boucheron", "MPID_H_Celine", "MPID_H_Chloe", "MPID_H_Louis", "MPID_H_Yunaka", "MPID_H_Staluke", "MPID_H_Citrinica", "MPID_H_Lapis", "MPID_H_Diamand", "MPID_H_Umber", "MPID_H_Jade", "MPID_H_Ivy", "MPID_H_Kagetsu", "MPID_H_Zelkova", "MPID_H_Fogato", "MPID_H_Pandoro", "MPID_H_Bonet", "MPID_H_Misutira", "MPID_H_Panetone", "MPID_H_Merin", "MPID_H_Hortensia", "MPID_H_Seadas", "MPID_H_Rosado", "MPID_H_Goldmary", "MPID_H_Linden", "MPID_H_Saphir", "MPID_H_Veyre", "MPID_H_Mauve", "MPID_H_Anna", "MPID_H_Jean", "MPID_H_El", "MPID_H_Rafale", "MPID_H_Selestia", "MPID_H_Gregory", "MPID_H_Madeline"];
pub const PID_ARRAY : &[&str] = &["PID_リュール", "PID_ヴァンドレ", "PID_クラン", "PID_フラン", "PID_アルフレッド", "PID_エーティエ", "PID_ブシュロン", "PID_セリーヌ", "PID_クロエ", "PID_ルイ", "PID_ユナカ", "PID_スタルーク", "PID_シトリニカ", "PID_ラピス", "PID_ディアマンド", "PID_アンバー", "PID_ジェーデ", "PID_アイビー", "PID_カゲツ", "PID_ゼルコバ", "PID_フォガート", "PID_パンドロ", "PID_ボネ", "PID_ミスティラ", "PID_パネトネ", "PID_メリン", "PID_オルテンシア", "PID_セアダス", "PID_ロサード", "PID_ゴルドマリー", "PID_リンデン", "PID_ザフィーア", "PID_ヴェイル", "PID_モーヴ", "PID_アンナ", "PID_ジャン", "PID_エル", "PID_ラファール", "PID_セレスティア", "PID_グレゴリー", "PID_マデリーン", ];
pub static mut ID: &[&str; 41] = &["001Lueur", "500Vandre", "501Clan", "550Fram", "100Alfred", "152Etie", "101Boucheron", "150Celine", "153Chloe", "102Louis", "253Yunaka", "201Staluke", "252Citrinica", "251Lapis", "200Diamand", "203Umber", "250Jade", "350Ivy", "302Kagetsu", "301Zelkova", "400Fogato", "401Pandoro","402Bonet", "450Misutira", "453Panetone", "452Merin", "351Hortensia", "403Seadas", "303Rosado", "352Goldmary", "304Linden", "254Saphir", "551Veyre", "502Mauve", "552Anna", "103Jean", "099El", "049Il", "553Selestia", "503Gregory", "554Madeline"];
pub static mut PLAYER_INDEX: usize = 0;
pub static mut LIMIT: [i8; 484] = [0; 484];
pub static mut GROW: [u8; 484] = [0; 484];
pub static mut ATTRS: [i32; 125] = [0; 125];
// DLC Check
#[skyline::from_offset(0x029f4270)]
pub fn has_content(content: i32, method_info: OptionalMethod) -> bool;
/*
//Get and Set Alear Gender
pub fn get_lueur_gender(){
    unsafe {
        if GameVariableManager::get_number(CHARACTER_KEY) == 0 && PLAYER_INDEX == 0 {
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
                                GameVariableManager::set_string("G_Lueur_Name".into(), &unit.edit.name.unwrap().get_string().unwrap());
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
                        let name = Mess::get(NAME_ARRAY[index] );
                        unit.edit.set_name( name );
                    }
                    return;
                }
            }   
        }
    }
}
pub struct CharacterMod;
impl ConfigBasicMenuItemSwitchMethods for CharacterMod {
    fn init_content(_this: & mut ConfigBasicMenuItem){ GameVariableManager::make_entry(CHARACTER_KEY, 0);}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(CHARACTER_KEY);
        let result: i32;
        unsafe {
            if has_content(0, None) { result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 40, 1); }
            else { result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 35, 1); }
            if toggle != result {
                GameVariableManager::set_number(CHARACTER_KEY, result);
                Self::set_command_text(this, None);
                Self::set_help_text(this, None);
                this.update_text();
                change_characters();
                return BasicMenuResult::se_cursor();
            } else {return BasicMenuResult::new(); }
        }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(CHARACTER_KEY) == 0 { this.help_text = "Characters are set their default appearance/growths.".into();  }
        else { this.help_text = "Characters are set to the selected character.".into();   }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_c: usize = (GameVariableManager::get_number(CHARACTER_KEY)).try_into().unwrap();
        if type_c != 0 { this.command_text = Mess::get(NAME_ARRAY[type_c]); }
        else {  this.command_text = OPTION_ARRAY[type_c].into() }
    }
}
pub fn set_person(){
    let seted = unsafe { SET_PERSON };
    if seted == false {
        unsafe {
            for x in 0..PID_ARRAY.len() {
                let person = PersonData::get(PID_ARRAY[x]).unwrap();
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
                    Some(_i) => {  
                        PLAYER_INDEX = x; 
                        let name = Mess::get(NAME_ARRAY[PLAYER_INDEX]).get_string().unwrap();
                        println!("Protag Skill found on person #{} - {}: {}", x, NAME_ARRAY[x], name);
                    }
                    None => {}
                }
            }
        }
    }
    unsafe { SET_PERSON = true; }
}

pub fn change_characters(){
    GameVariableManager::make_entry(CHARACTER_KEY, 0);
    let result =  GameVariableManager::get_number(CHARACTER_KEY);
    set_person();
    if result == 0 {
        unsafe {
            for x in 0..PID_ARRAY.len() {
                let person = PersonData::get(PID_ARRAY[x]).unwrap();
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
                person.set_name(NAME_ARRAY[x].into());
                person.set_ascii_name(substring(NAME_ARRAY[x].into(), 5, None ));
                person.set_help(HELP_ARRAY[x].into());
                flag.value = ATTRS[3*x+2];
                person.set_fid(PID_ARRAY[x].into());
                person.on_complete();
            }
        }
        println!("Characters are set to default.");
        set_lueur_gender(0);
    }
    else {
        let index = result as usize;
        let name = NAME_ARRAY[index];
        let help = HELP_ARRAY[index];
        let current_person = PersonData::get(PID_ARRAY[index]).unwrap();
        current_person.set_name(name.into());
        current_person.set_help(help.into());
        current_person.on_complete();
        let current_person_skills = current_person.get_common_skills();
        unsafe {
            for x in 0..PID_ARRAY.len() {
                if x == index { continue; }
                let person = PersonData::get(PID_ARRAY[x]).unwrap();
                let caps = person.get_limit();
                let grow = person.get_grow();
                person.set_name(name.into());
                person.set_help(help.into());
                person.set_ascii_name(substring(name.into(), 5, None ));
                person.set_fid(PID_ARRAY[index].into());
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
                if x != PLAYER_INDEX {
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
*/

#[unity::class("TMPro", "TextMeshProUGUI")]
pub struct TextMeshProUGUI {
}

#[unity::class("App", "LoadingLogo")]
pub struct LoadingLogo {
   __: [u8; 0x60],
   pub title_text: &'static mut TextMeshProUGUI,
   pub tips_text: &'static mut TextMeshProUGUI,
}


impl TextMeshProUGUI {
    pub fn set_text(&mut self, source_text: &Il2CppString, sync_text_input_box: bool) {
        unsafe { tmptext_settext(self, source_text, sync_text_input_box, None) };
    }
}

#[skyline::from_offset(0x2837690)]
fn tmptext_settext(this: &mut TextMeshProUGUI, source_text: &Il2CppString, sync_text_input_box: bool, method_info: OptionalMethod);

#[unity::class("App", "UnitRecord")]
pub struct UnitRecord {
   pub values: &'static Array<i32>,
}

#[unity::hook("App", "LoadingLogo", "SetTipsData")]
pub fn set_tip_text(this: &mut LoadingLogo, tips: u64, method_info: OptionalMethod){
   let force = Force::get(ForceType::Dead);
   call_original!(this, tips, method_info);
   let rng = engage::random::Random::get_game();
   if rng.get_value(100) < 50 { return; }
   if force.is_none() { return; }
   let dead_force = force.unwrap();
   let count = dead_force.get_count();
   if count == 0 { return; }
   let mut string_dead = format!("{} Dead Units", count);
   this.title_text.set_text( format!("{} Dead Units", count).into(), true);
   let mut force_iter = Force::iter(dead_force);
   let mut unit_count = 0;
   while let Some(unit) = force_iter.next() {
       unsafe {
           let name = Mess::get(unit.person.get_name().unwrap()).get_string().unwrap();
           let record = unit_get_record(unit, None);
           let dead_chapter = unit_record_get_dead_chapter(record, None);
           if dead_chapter.is_some() {
               println!("Dead Chapter for {}: {}", name, dead_chapter.unwrap().name.get_string().unwrap());
               let dead_chapter_name  = chapter_get_name(dead_chapter.unwrap(), None).get_string().unwrap();
               let prefix = Mess::get(format!("{}_PREFIX", dead_chapter.unwrap().name.get_string().unwrap())).get_string().unwrap();
               if unit_count != 0 { string_dead = format!("{}\n{} in {}: {}", string_dead, name, prefix, dead_chapter_name);}
               else { string_dead = format!("{} in {}: {}", name, prefix, dead_chapter_name); }
           }
           else {
               if unit_count != 0 {
                   if unit_count % 2 == 0 { format!("{} \n {}", string_dead, name); }
                   else { string_dead = format!("{} - {}", string_dead, name); }
               }
               else {
                   string_dead = name;
               }
           }
           unit_count += 1;
       }
   }
   this.tips_text.set_text(string_dead.into(), true);
}
#[skyline::from_offset(0x01a57fb0)]
fn unit_get_record(this: &Unit, method_info: OptionalMethod) -> &UnitRecord;

#[skyline::from_offset(0x01c57f30)]
fn unit_record_get_dead_chapter(this: &UnitRecord, method_info: OptionalMethod) -> Option<&'static ChapterData>;

#[skyline::from_offset(0x02af9a40)]
fn chapter_get_name(this: &ChapterData,method_info: OptionalMethod) -> &'static Il2CppString;