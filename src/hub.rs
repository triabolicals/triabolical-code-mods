use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use unity::system::List;
use unity::il2cpp::object::Array;
use engage::{
    gamedata::{*,animal::*, item::*},
    hub::{hubsequence::*, access::*},
    menu::{BasicMenuItem, BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    proc::*,
    gameuserdata::GameUserData,
    gamevariable::GameVariableManager,
    random::Random,
};
use crate::string::*;

#[unity::from_offset("App", "WellSequence", "CalcItemExchange")]
pub fn well_get_item(this: &HubSequence, level: i32, random: &Random, method_info: OptionalMethod) -> &'static mut List<ItemData>;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02a719d0)]
pub fn HubVariable_CurrentStartName(method_info: OptionalMethod) -> &'static Il2CppString;
//Kizana Exit
// 023ed420 - MapEndingEvent, 0x23ed2c0 - KizanaExit

pub fn can_adopt(access: &HubAccessData) -> bool { 
    if GameUserData::get_sequence() != 5  {return false; }  //If not Exploration
    if !HubFacilityData::get("AID_動物小屋").unwrap().is_complete() { return false; }   // If farm isn't unlocked
    if !access.is_animal() { return false; }
    if access.get_is_done() { return false; }

    let animal_pid = access.try_get_pid();
    if animal_pid.is_none() { return false; }

    let animal = AnimalData::get_by_pid(animal_pid.unwrap());
    if animal.is_none() { return false; }

    if animal.unwrap().can_capture() { return true; }
    return false;
}

pub fn try_capture_animal(access: &HubAccessData) -> bool {

    if can_adopt(access){
        let animal = AnimalData::get_by_pid(access.try_get_pid().unwrap()).unwrap();
        animal.increment_capture();
        return access.done();
    }
    return false;
}

pub fn get_item_count() -> i32 {
    let kizuna = (GameUserData::get_sequence() == 5 );
    let mut result = 0;
    let hub_sequence = HubSequence::get_instance();
    let access_manager = hub_sequence.get_current_access_data();
    let access_list = access_manager.access_list;
    for x in 0..access_list.len() {
        let item = access_list[x].aid;
        let locator = access_list[x].dispos_data.get_locator();
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

pub fn set_achieve_menu_status() -> bool {
    let achieve = AchievementMenu::instantiate().unwrap();
    unsafe {
        if get_achieve_reward_num(achieve, None) == 0 {
            Patch::in_text(0x0225fb00).bytes(&[0x80,0x00,0x80, 0x52]);
            return false;
        }
        else {
            Patch::in_text(0x0225fb00).bytes(&[0x20,0x00,0x80, 0x52]);
            return true;
        }
    }
}

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

