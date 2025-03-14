use unity::prelude::*;
use engage::{
    force::{Force, ForceType}, 
    gamevariable::*, menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuItemAttribute, BasicMenuResult}, mess::*,
};
const GENDER_KEY: &str = "G_Lueur_Gender2";
//Character mod
pub const NAME_ARRAY : &[&str] = &["MPID_Lueur", "MPID_Vandre", "MPID_Clan", "MPID_Fram", "MPID_Alfred", "MPID_Etie", "MPID_Boucheron", "MPID_Celine", "MPID_Chloe", "MPID_Louis", "MPID_Yunaka", "MPID_Staluke", "MPID_Citrinica", "MPID_Lapis", "MPID_Diamand", "MPID_Umber", "MPID_Jade", "MPID_Ivy", "MPID_Kagetsu", "MPID_Zelkova", "MPID_Fogato", "MPID_Pandoro", "MPID_Bonet", "MPID_Misutira", "MPID_Panetone", "MPID_Merin", "MPID_Hortensia", "MPID_Seadas", "MPID_Rosado", "MPID_Goldmary", "MPID_Linden", "MPID_Saphir", "MPID_Veyre", "MPID_Mauve", "MPID_Anna", "MPID_Jean", "MPID_El", "MPID_Rafale", "MPID_Selestia", "MPID_Gregory", "MPID_Madeline"];
pub const PID_ARRAY : &[&str] = &["PID_リュール", "PID_ヴァンドレ", "PID_クラン", "PID_フラン", "PID_アルフレッド", "PID_エーティエ", "PID_ブシュロン", "PID_セリーヌ", "PID_クロエ", "PID_ルイ", "PID_ユナカ", "PID_スタルーク", "PID_シトリニカ", "PID_ラピス", "PID_ディアマンド", "PID_アンバー", "PID_ジェーデ", "PID_アイビー", "PID_カゲツ", "PID_ゼルコバ", "PID_フォガート", "PID_パンドロ", "PID_ボネ", "PID_ミスティラ", "PID_パネトネ", "PID_メリン", "PID_オルテンシア", "PID_セアダス", "PID_ロサード", "PID_ゴルドマリー", "PID_リンデン", "PID_ザフィーア", "PID_ヴェイル", "PID_モーヴ", "PID_アンナ", "PID_ジャン", "PID_エル", "PID_ラファール", "PID_セレスティア", "PID_グレゴリー", "PID_マデリーン", ];

pub struct HeroNameGenderMod {}
impl ConfigBasicMenuItemSwitchMethods for HeroNameGenderMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number(GENDER_KEY);
        if toggle == 0 {
            this.help_text = format!("Cannot change {}'s gender", Mess::get("MPID_Lueur")).into();
            this.update_text();
            return BasicMenuResult::new();
        }
        let male = toggle == 1;
        let result = ConfigBasicMenuItem::change_key_value_b(male);
        if male != result {
            let gender = if result { 1 } else { 2 };
            GameVariableManager::set_number(GENDER_KEY, gender);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            change_lueur_gender();
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = format!("Change {}'s gender", Mess::get("MPID_Lueur")).into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = if GameVariableManager::get_number(GENDER_KEY) == 1 {  "Male"}  else { "Female" }.into();
    }
}

pub fn hero_gender_build_attr(_this: &ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if GameVariableManager::get_number(GENDER_KEY) == 0 { BasicMenuItemAttribute::Hide } else { BasicMenuItemAttribute::Enable }
}

extern "C" fn gender_switch() -> &'static mut ConfigBasicMenuItem { 
    let new_switch = ConfigBasicMenuItem::new_switch::<HeroNameGenderMod>(format!("{} Gender", Mess::get("MPID_Lueur")));
    new_switch.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = hero_gender_build_attr as _);
    new_switch
}

 pub fn gender_install(){ cobapi::install_game_setting(gender_switch); }

// DLC Check
pub fn get_lueur_name_gender(){
    if GameVariableManager::exist(GENDER_KEY) { return; }
    GameVariableManager::make_entry(GENDER_KEY, 0);
    let f_type: [ForceType; 5] = [ForceType::Player, ForceType::Enemy, ForceType::Absent, ForceType::Dead, ForceType::Lost];
    for f in f_type {
        let force = Force::get(f).unwrap();
        let mut force_iter = Force::iter(force);
        while let Some(unit) = force_iter.next() {
            if unit.person.pid.to_string() == "PID_リュール" {
                if unit.edit.name.is_some(){
                    if unit.edit.gender != 0 {
                        if unit.edit.gender > 2 { unit.edit.set_gender(1); }
                        GameVariableManager::set_number(GENDER_KEY, unit.edit.gender);
                        return;
                    }
                }
            }
        }
    }
}

pub fn change_lueur_gender(){
    if GameVariableManager::get_number(GENDER_KEY) == 0 { return; }
    [ForceType::Player, ForceType::Enemy, ForceType::Absent, ForceType::Dead, ForceType::Lost].into_iter().for_each(|ty|
        if let Some(force) = Force::get(ty) {
            force.iter().for_each(|unit|{
                if unit.person.pid.to_string() == "PID_リュール" || unit.person.get_flag().value & 128 != 0 {
                    if unit.edit.gender != 0 { unit.edit.set_gender(GameVariableManager::get_number(GENDER_KEY) ) }
                }
            });
        }
    )
}

