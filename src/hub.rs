use unity::{prelude::*, system::List,};
use engage::{
    gamedata::{*,animal::*, item::*},
    hub::{hubsequence::*, access::*},
    menu::BasicMenuItem,
    gameuserdata::GameUserData,
    random::Random,
};
//functions for hub related stuff

#[unity::from_offset("App", "WellSequence", "CalcItemExchange")]
pub fn well_get_item(this: &HubSequence, level: i32, random: &Random, method_info: OptionalMethod) -> &'static mut List<ItemData>;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubAccessData", "TryGetPID")]
pub fn access_data_try_get_pid(this: &HubAccessData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

fn try_get_pid(this: &HubAccessData) -> Option<&'static Il2CppString> {
    unsafe { access_data_try_get_pid(this, None)}
}
pub fn can_adopt(access: &HubAccessData) -> bool { 
    if GameUserData::get_sequence() != 5  {return false; }  //If not Exploration
    if !HubFacilityData::get("AID_動物小屋").unwrap().is_complete() { return false; }   // If farm isn't unlocked
    if !access.is_animal() { return false; }
    if access.get_is_done() { return false; }

    let animal_pid = try_get_pid(access);
    if animal_pid.is_none() { return false; }

    let animal = AnimalData::get_by_pid(animal_pid.unwrap());
    if animal.is_none() { return false; }

    if animal.unwrap().can_capture() { return true; }
    return false;
}

pub fn try_capture_animal(access: &HubAccessData) -> bool {
    if can_adopt(access){
        let animal = AnimalData::get_by_pid(try_get_pid(access).unwrap()).unwrap();
        animal.increment_capture();
        return access.done();
    }
    return false;
}

pub fn get_item_count() -> i32 {
    //let kizuna = GameUserData::get_sequence() == 5;
    let mut result = 0;
    let hub_sequence = HubSequence::get_instance();
    let access_manager = hub_sequence.get_current_access_data();
    let access_list = access_manager.access_list;
    for x in 0..access_list.len() {
        let item = access_list[x].aid;
        let is_done = access_list[x].get_is_done();
        if can_adopt(access_list[x]){ result += 1; }
        if item.is_some() && !is_done {
            let item_data = ItemData::get_mut(&item.unwrap().get_string().unwrap());
            if item_data.is_some() { result += 1; }
        }
    }
    result
}

#[unity::class("App", "AchievementMenu")]
pub struct AchievementMenu {}

#[unity::from_offset("App","AchievementMenu", "GetAllRewardNum")]
pub fn get_achieve_reward_num(this: &AchievementMenu, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x0225ff30)]
pub fn investment_item_acall(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x0225fea0)]
pub fn investment_item_get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x0225ff20)]
pub fn investment_item_build(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x0225fb10)]
pub fn achieve_item_acall(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x0225fa80)]
pub fn achieve_item_get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &Il2CppString;

#[unity::class("App", "JukeboxData")]
pub struct JukeBoxData {
    pub parent: StructBaseFields,
    pub event_name: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub condition: &'static Il2CppString,
}
impl Gamedata for JukeBoxData {}

#[unity::class("App", "MusicData")]
pub struct MusicData {
    pub parent: StructBaseFields,
    pub event_name: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
}
impl Gamedata for MusicData {}
#[unity::from_offset("App", "JukeboxData", ".ctor")]
pub fn juke_box_data_ctor(this: &JukeBoxData, method_info: OptionalMethod);

#[unity::from_offset("App", "JukeboxData", "set_Condition")]
pub fn juke_box_data_set_condition(this: &JukeBoxData, value: &Il2CppString,method_info: OptionalMethod);

#[unity::from_offset("App", "JukeboxData", "set_EventName")]
pub fn juke_box_data_set_event(this: &JukeBoxData, value: &Il2CppString,method_info: OptionalMethod);

#[unity::from_offset("App", "JukeboxData", "set_Name")]
pub fn juke_box_data_set_name(this: &JukeBoxData, value: &Il2CppString, method_info: OptionalMethod);
pub fn add_to_juke_box(){
    let music_list = MusicData::get_list().unwrap();
    unsafe {
        if JukeBoxData::get_count() > 10 { return; }
        let jukebox_list = JukeBoxData::get_list_mut().unwrap();
        for x in 0..music_list.len() {
        let new_juke_box = JukeBoxData::instantiate().unwrap();
            juke_box_data_ctor(new_juke_box, None);
            juke_box_data_set_event(new_juke_box, music_list[x as usize].event_name, None);
            juke_box_data_set_name(new_juke_box, music_list[x as usize].name, None);
            jukebox_list.add(new_juke_box);
        }
    }
}



