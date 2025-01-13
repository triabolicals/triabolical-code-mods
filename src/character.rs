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
pub const NAME_ARRAY : &[&str] = &["MPID_Lueur", "MPID_Vandre", "MPID_Clan", "MPID_Fram", "MPID_Alfred", "MPID_Etie", "MPID_Boucheron", "MPID_Celine", "MPID_Chloe", "MPID_Louis", "MPID_Yunaka", "MPID_Staluke", "MPID_Citrinica", "MPID_Lapis", "MPID_Diamand", "MPID_Umber", "MPID_Jade", "MPID_Ivy", "MPID_Kagetsu", "MPID_Zelkova", "MPID_Fogato", "MPID_Pandoro", "MPID_Bonet", "MPID_Misutira", "MPID_Panetone", "MPID_Merin", "MPID_Hortensia", "MPID_Seadas", "MPID_Rosado", "MPID_Goldmary", "MPID_Linden", "MPID_Saphir", "MPID_Veyre", "MPID_Mauve", "MPID_Anna", "MPID_Jean", "MPID_El", "MPID_Rafale", "MPID_Selestia", "MPID_Gregory", "MPID_Madeline"];
pub const PID_ARRAY : &[&str] = &["PID_リュール", "PID_ヴァンドレ", "PID_クラン", "PID_フラン", "PID_アルフレッド", "PID_エーティエ", "PID_ブシュロン", "PID_セリーヌ", "PID_クロエ", "PID_ルイ", "PID_ユナカ", "PID_スタルーク", "PID_シトリニカ", "PID_ラピス", "PID_ディアマンド", "PID_アンバー", "PID_ジェーデ", "PID_アイビー", "PID_カゲツ", "PID_ゼルコバ", "PID_フォガート", "PID_パンドロ", "PID_ボネ", "PID_ミスティラ", "PID_パネトネ", "PID_メリン", "PID_オルテンシア", "PID_セアダス", "PID_ロサード", "PID_ゴルドマリー", "PID_リンデン", "PID_ザフィーア", "PID_ヴェイル", "PID_モーヴ", "PID_アンナ", "PID_ジャン", "PID_エル", "PID_ラファール", "PID_セレスティア", "PID_グレゴリー", "PID_マデリーン", ];
// DLC Check
#[skyline::from_offset(0x029f4270)]
pub fn has_content(content: i32, method_info: OptionalMethod) -> bool;

#[unity::class("TMPro", "TextMeshProUGUI")]
pub struct TextMeshProUGUI {}

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
           let name = Mess::get_name(unit.person.pid).to_string();
           let record = unit_get_record(unit, None);
           let dead_chapter = unit_record_get_dead_chapter(record, None);
           if dead_chapter.is_some() {
               let dead_chapter_name  = chapter_get_name(dead_chapter.unwrap(), None).to_string();
               let prefix = Mess::get(format!("{}_PREFIX", dead_chapter.unwrap().name.to_string())).to_string();
               if unit_count != 0 { string_dead = format!("{}\n{} - {}: {}", string_dead, name, prefix, dead_chapter_name);}
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