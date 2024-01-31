use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::*;
use engage::gameuserdata::GameUserData;
use engage::gameuserdata::*;
use engage::gamevariable::GameVariableManager;
static mut WELL_ITEMS: bool = false;
static mut WELL_MENU: bool = false;
use crate::string::*;
pub const WELL_KEY: &str = "G_WELL";
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};

pub struct WellMod;
impl ConfigBasicMenuItemSwitchMethods for WellMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        GameVariableManager::make_entry_norewind(WELL_KEY, 0);
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry_norewind(WELL_KEY, 0);
        let toggle = GameVariableManager::get_bool(WELL_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(WELL_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_bool(WELL_KEY) {this.help_text = format!("Well menu will appear when leaving the somniel from the outside.").into(); }
        else {this.help_text = format!("Well related settings are off.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_bool(WELL_KEY) { this.command_text = On_str(); }
        else { this.command_text = Off_str(); }
    }
}
extern "C" fn well() -> &'static mut ConfigBasicMenuItem { 
    unsafe {
    let well_string_label = setting_str("MID_Hub_Well");
    ConfigBasicMenuItem::new_switch::<WellMod>(well_string_label.get_string().unwrap())
    }
 }

pub fn well_install(){
    cobapi::install_game_setting(well);
}

#[skyline::from_offset(0x293a700)]
pub fn get_IsItemReturn(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2939950)]
pub fn get_well_useFlag(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "get_ExchangeLevel")]
pub fn get_well_exchangeLevel(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "get_Seed")]
pub fn get_well_seed(method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x2939a80)]
pub fn set_well_flag(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x2939dc0)]
pub fn set_well_level(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x293a100)]
pub fn set_seed(value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "GetItem")]
pub fn well_get_item(this: &HubSequence, method_info: OptionalMethod);

#[skyline::from_offset(0x023efab0)]
pub fn well_CreateBind(this: &HubSequence, method_info: OptionalMethod);

#[unity::class("App", "HubAccessManager")]
pub struct HubAccessManager {}

#[unity::class("App", "HubSequence")]
pub struct HubSequence {
    proc: [u8; 0x78],
    pub m_ScriptFuncName: &'static Il2CppString,
    pub m_FastTravelID: &'static Il2CppString,
    m_TalkAccess: u64,
    pub m_IsBackgroundBind: bool,
    pub m_IsKeyHelp: bool,
    pub m_IsCave: bool,
    pub m_SceneName: &'static Il2CppString,
    pub m_StartName: &'static Il2CppString,
    m_junk: [u64; 18],
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

#[unity::from_offset("App", "HubSequence", "get_CurrentAccessData")]
pub fn hub_get_access_data(this: &HubSequence, method_info: OptionalMethod) -> &HubAccessManager;

#[skyline::from_offset(0x21733a0)]
pub fn GetNotTakenPieceOfBond(this: &HubAccessManager, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;


#[skyline::from_offset(0x02a719d0)]
pub fn HubVariable_CurrentStartName(method_info: OptionalMethod) -> &'static Il2CppString;
//Kizana Exit
// 023ed420 - MapEndingEvent, 0x23ed2c0 - KizanaExit

#[skyline::hook(offset = 0x23f1bb0)]
pub fn hub_next_map(sceneName: &Il2CppString, startName: &Il2CppString, method_info: OptionalMethod){
    unsafe {
        let sequence = GameUserData::get_sequence();
        if sequence == 5 {
            WELL_ITEMS = false;
            WELL_MENU = true;
        }
        let startname = HubVariable_CurrentStartName(method_info);
    }
    call_original!(sceneName, startName, method_info);
}


#[skyline::hook(offset = 0x23ed2c0)]
pub fn well_items(this: & mut HubSequence, method_info: OptionalMethod){
    unsafe {
        let sequence = GameUserData::get_sequence();
        let instance = GameUserData::get_instance();
        let well_available = GameVariableManager::get_number("G_拠点_裏武器イベント") > 2;
        let hubacess = hub_get_access_data(this, method_info);
        println!("well item function called {}, {}", well_available,GameVariableManager::get_bool(WELL_KEY) );
        if sequence == 5 {
            let count = GetNotTakenPieceOfBond(hubacess, None) + get_PieceOfBond(instance, None);
            set_PieceOfBond(instance, count, None);
        }
        let put_in_items = get_well_useFlag(None);
        let fast = is_null_empty(this.m_FastTravelID, None);
        if well_available && GameVariableManager::get_bool(WELL_KEY) {
            let can_get_items = get_IsItemReturn(None);
            if sequence == 5 {
                if can_get_items { well_get_item(this, method_info); }
                else { call_original!(this, method_info); }
            }
            else if (sequence == 4 && put_in_items == 0) && (WELL_ITEMS && this.m_SceneName.get_string().unwrap() == "Hub_Solanel".to_string()) { 
                println!("Well Binding for Somniel -> GMAP");
                println!("HubSequence: {}", this.m_SceneName.get_string().unwrap());
                this.is_EnterEvent_k_BackingField = false;
                well_CreateBind(this, method_info);
             }
        }
        else { 
           // println!("Calling Kizana Sequence {}, {}, {}", sequence, WELL_ITEMS, put_in_items == 0);
            call_original!(this, method_info);
          }
        WELL_ITEMS = false;
        WELL_MENU = false;
    }
}

#[skyline::hook(offset = 0x23f1f10)]
pub fn NextGmap(method_info: OptionalMethod){

    unsafe {
    let sequence = GameUserData::get_sequence();
    println!("NextMap Called on sequence = {}", sequence);
        if sequence == 5 {
            WELL_ITEMS = true;
            WELL_MENU = false;
        }
        else if sequence == 4 { WELL_ITEMS = true;}
    }

    call_original!(method_info);
}
