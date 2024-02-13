use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, item::*};
use engage::gameuserdata::GameUserData;
use engage::gameuserdata::*;
use engage::proc::*;
use unity::il2cpp::object::Array;
use engage::menu::BasicMenu;

use unity::system::List;
use engage::menu::BasicMenuItem;
use crate::rng::*;
use engage::gamevariable::GameVariableManager;
static mut WELL_ITEMS: bool = false;
static mut WELL_MENU: bool = false;
use crate::string::*;
pub const WELL_KEY: &str = "G_WELL";
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};


#[skyline::from_offset(0x293a100)]
pub fn set_seed(value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "CalcItemExchange")]
pub fn well_get_item(this: &HubSequence, level: i32, random: &Random, method_info: OptionalMethod) -> &'static mut List<ItemData>;

#[skyline::from_offset(0x023efab0)]
pub fn well_CreateBind(this: &HubSequence, method_info: OptionalMethod);

#[unity::class("App", "HubDisposData")]
pub struct HubDisposData {}

#[unity::class("App", "HubAccessData")]
pub struct HubAccessData {
    pub aid: Option<&'static Il2CppString>,
    pub dispos_data: &'static HubDisposData,
}

#[unity::class("App", "AnimalData")]
pub struct AnimalData {}

#[unity::class("App", "AnimalMenuSequence")]
pub struct AnimalMenuSequence {
    pub proc: ProcInstFields,
}
impl Bindable for AnimalMenuSequence {}

#[skyline::from_offset(0x02174d70)]
pub fn animal_menu_create_bind<P: Bindable>( this: &P, shop: i32, method_info: OptionalMethod);

#[unity::class("App", "HubAccessManager")]
pub struct HubAccessManager {
    pub scene_name: &'static Il2CppString,
    pub access_list: &'static List<HubAccessData>,
    pub dispos_list: &'static List<HubDisposData>,
    pub dispos_item_list: &'static List<HubDisposData>,
    talk_limit: * const u8,
    pub animal_list: &'static List<AnimalData>, 
}


#[unity::class("App", "HubMiniMap")]
pub struct HubMiniMap {}

#[unity::class("App", "HubSequence")]
pub struct HubSequence {
    proc: [u8; 0x78],
    pub m_ScriptFuncName: &'static Il2CppString,
    pub m_FastTravelID: &'static Il2CppString,
    m_TalkAccess: u64,
    pub m_IsBackgroundBind: bool,
    pub m_IsKeyHelp: bool,
    pub m_IsCave: bool,
    junk: [u8; 5],
    pub m_SceneName: &'static Il2CppString,
    pub m_StartName: &'static Il2CppString,
    m_junk: [u64; 15],
    pub mini_map: &'static HubMiniMap,
    pub _ForceNonStopBGM_k_BackingField: bool,
    _accessData_k_BackingField: u64,
    pub m_IsShutdown: bool,
    pub is_EnterEvent_k_BackingField: bool,
    m_numPieceOfBond : i32,
    EndRollDisableList: u64,
    ArgConfirmMid: &'static Il2CppString,
    ArgScriptName: &'static Il2CppString,
    //

}
#[unity::from_offset("App", "HubMiniMap", "Hide")]
pub fn hub_mini_map_hide(this: &HubMiniMap, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "IsShow")]
pub fn hub_mini_map_Is_Show(this: &HubMiniMap, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubSequence", "get_MiniMap")]
pub fn hub_get_mini_map(this: &HubSequence, method_info: OptionalMethod) -> &HubMiniMap;
#[unity::from_offset("App", "HubMiniMap", "HideSystemMenu")]
pub fn hub_mini_map_system_hide(this: &HubMiniMap, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "SetMode")]
pub fn hub_mini_map_set_mode(this: &HubMiniMap, mode: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "HubSequence", "get_CurrentAccessData")]
pub fn hub_get_access_data(this: &HubSequence, method_info: OptionalMethod) -> &HubAccessManager;

#[unity::from_offset("App", "HubAccessData", "get_TalkItem")]
pub fn hub_access_get_talk_item(this: &HubAccessData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[skyline::from_offset(0x21733a0)]
pub fn GetNotTakenPieceOfBond(this: &HubAccessManager, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02a719d0)]
pub fn HubVariable_CurrentStartName(method_info: OptionalMethod) -> &'static Il2CppString;
//Kizana Exit
// 023ed420 - MapEndingEvent, 0x23ed2c0 - KizanaExit
pub fn get_hub_sequence() -> &'static HubSequence{
    let mut method = HubSequence::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("get_Instance")));
    if method.is_none() { method = HubSequence::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("get_Instance"))); }
    let get_instance = unsafe { std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> &'static HubSequence>( method.unwrap().method_ptr, ) };
    get_instance(method.unwrap())
}
#[unity::from_offset("App", "HubDisposData", "get_Locator")]
pub fn dispos_hub_get_locator(this: &HubDisposData, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "HubAccessData", "get_IsDone")]
pub fn access_data_is_done(this: &HubAccessData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubAccessData", "DoneAccess")]
pub fn access_data_done(this: &HubAccessData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubAccessData", "get_ItemCount")]
pub fn access_data_item_count(this: &HubAccessData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "HubAccessData", "get_IsAnimal")]
pub fn access_data_is_animal(this: &HubAccessData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubUtil", "GetItemCountWithBonus")]
pub fn Hub_Get_Item_Count_Bonus(item: &ItemData, base_count: i32, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "HubAccessData", "TryGetPID")]
pub fn access_data_try_get_pid(this: &HubAccessData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[unity::from_offset("App", "AnimalData", "GetByPID")]
pub fn get_animal_by_PID(pid: &Il2CppString, method_info: OptionalMethod) -> Option<&AnimalData>;

#[unity::from_offset("App", "HubUtil", "IsCaptureAnimal")]
pub fn is_capture_animal(animial: &AnimalData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubUtil", "IncAnimalCaptureNum")]
pub fn inc_animal_capture_num(animal: &AnimalData, number: i32, method_info: OptionalMethod);

pub fn can_adopt(access: &HubAccessData) -> bool { 
unsafe {
    if GameUserData::get_sequence() != 5  {return false; }  //If not Exploration
    if !HubFacilityData::get("AID_動物小屋").unwrap().is_complete() { return false; }   // If farm isn't unlocked
    if !access_data_is_animal(access, None) { return false; }
    if access_data_is_done(access, None) { return false; }

    let animal_pid = access_data_try_get_pid(access, None);
    if animal_pid.is_none() { return false; }

    let animal = get_animal_by_PID(animal_pid.unwrap(), None);
    if animal.is_none() { return false; }

    if is_capture_animal(animal.unwrap(), None) { return true; }
    return false;
}
}

pub fn try_capture_animal(access: &HubAccessData) -> bool {
    unsafe {
        if GameUserData::get_sequence() != 5  {return false; }  //If not Exploration
        if !HubFacilityData::get("AID_動物小屋").unwrap().is_complete() { return false; }   // If farm isn't unlocked
        if !access_data_is_animal(access, None) { return false; }
        if access_data_is_done(access, None) { return false; }

        let animal_pid = access_data_try_get_pid(access, None);
        if animal_pid.is_none() { return false; }

        let animal = get_animal_by_PID(animal_pid.unwrap(), None);
        if animal.is_none() { return false; }

        if is_capture_animal(animal.unwrap(), None) {
            inc_animal_capture_num(animal.unwrap(), 1, None);
            return access_data_done(access, None);  //  un-marked this location
        }

        return false;
    }
}
pub fn get_item_count() -> i32 {
    unsafe {
        let kizuna = (GameUserData::get_sequence() == 5 );
        let mut result = 0;
        let hub_sequence = get_hub_sequence();
        let access_manager = hub_get_access_data(hub_sequence, None);
        let access_list = access_manager.access_list;
        for x in 0..access_list.len() {
            let item = access_list[x].aid;
            let locator = dispos_hub_get_locator(access_list[x].dispos_data, None);
            let is_done = access_data_is_done(access_list[x], None);
            let item_count = access_data_item_count(access_list[x], None);
            if can_adopt(access_list[x]){
                result += item_count;
            }
            if item.is_some() && !is_done {
                let item_data = ItemData::get_mut(&item.unwrap().get_string().unwrap());
                if item_data.is_some() {
                    result += item_count;
                }
            }
        }
        result
    }
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

#[unity::class("App", "HubAccessoryRoom")]
pub struct HubAccessoryRoom {}
#[skyline::from_offset(0x02173790)]
pub fn get_return_accesssory_name(this: &HubAccessoryRoom, method_info: OptionalMethod) -> &'static Il2CppString;
#[skyline::hook(offset=0x02173d50)]
pub fn accessory_hook(this: &HubAccessoryRoom, shop: i32, method_info: OptionalMethod){
    call_original!(this, shop, method_info);
    unsafe {
        let mut scene_name = get_return_accesssory_name(this, None);
        let scen = get_hub_sequence().m_SceneName.get_string().unwrap();
        scene_name = scen.into();
    }
}