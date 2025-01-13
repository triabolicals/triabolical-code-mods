use unity::{prelude::*, system::List,};
use engage::{
    gamedata::{*,animal::*, item::*},
    hub::{hubsequence::*, access::*},
    menu::BasicMenuItem,
    gameuserdata::GameUserData,
    random::Random,
};
use crate::menus::CommonRewardSequence;
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

pub fn get_list_item_class() -> &'static Il2CppClass {
    let common_rewards_sequence = CommonRewardSequence::instantiate().unwrap();
    let methods = common_rewards_sequence.get_class().get_methods();
    let ctor_parameters = methods[0].get_parameters();
    let para = unity::prelude::Il2CppClass::from_il2cpptype( ctor_parameters[2].parameter_type ).unwrap();
    return para;
}

pub fn get_item_count() -> i32 {
    //let kizuna = GameUserData::get_sequence() == 5;
    let mut result = 0;
    let hub_sequence = HubSequence::get_instance();
    let access_manager = hub_sequence.get_current_access_data();
    let access_list = access_manager.access_list;
    let it = get_list_item_class();
    let test = il2cpp::instantiate_class::<List<ItemData>>(it);
    if test.is_err() { return 0; }
    else if it.get_methods().iter().find(|method| method.get_name() == Some(String::from("Add"))).is_none() { return 0; }
    for x in 0..access_list.len() {
        if access_list[x].get_is_done() { continue; }
        if can_adopt(access_list[x]){ result += 1; }
        if let Some(iid) = access_list[x].aid {
            if let Some(item) = ItemData::get(iid) {
                if unsafe { is_null_empty(item.name, None) || is_null_empty(item.help, None) } { continue; }
                else { result += 1; }
            }
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

#[unity::class("App", "FishingFishData")]
pub struct FishingFishData {
    pub parent: StructBaseFields,
    idc: [u8; 0x4c],
    pub catch_time: f32,
    pub catch_time_rnd_add: f32,
    pub escape_time: f32,
    pub hp: f32,
    pub lethal_hp: f32,
    pub regen_per_sec: f32,
}
impl Gamedata for FishingFishData {}

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

pub fn adjust_fish_data() {
    FishingFishData::get_list_mut().unwrap().iter_mut()
        .for_each(|fish|{
            fish.catch_time = 1.0;
            fish.catch_time_rnd_add = 0.5;
            fish.escape_time = 10.0;
            fish.hp = 5.0;
            fish.lethal_hp = 2.0;
        }
    );

}
#[skyline::from_offset(0x02c4ea10)]
pub fn set_game_object_active(this: u64, value: bool, method_info: OptionalMethod);
