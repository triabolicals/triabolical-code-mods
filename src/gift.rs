use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;

pub const GIFT_KEY: &str = "G_GIFT";

pub struct Giftmod;
impl ConfigBasicMenuItemSwitchMethods for Giftmod {
    fn init_content(this: &mut ConfigBasicMenuItem){  GameVariableManager::make_entry_norewind(GIFT_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number(GIFT_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(GIFT_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_number(GIFT_KEY);
        if toggle == 0 { this.help_text = format!("Accept both patch and DLC gift items.").into(); } 
        else if toggle == 1 { this.help_text = format!("Accept only patch update items.").into(); } 
        else if toggle == 2 { this.help_text = format!("Accept only paid DLC items.").into(); }
        else if toggle == 3 { this.help_text = format!("Accept no gift items.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_number(GIFT_KEY);
        if toggle == 0 { this.command_text = format!("All Gifts").into();} 
        else if toggle == 1 { this.command_text = format!("Patch Update Gifts").into(); }
        else if toggle == 2 { this.command_text = format!("Expansion Pass Gifts").into(); }
        else if toggle == 3 { this.command_text = format!("No Gifts").into(); }
    }
}
#[skyline::hook(offset= 0x023f3c00)]
pub fn TryGiftEvent(this: u64, flagName: &Il2CppString, rewardID: &Il2CppString, messageID: &Il2CppString, method_info: OptionalMethod){
    let toggle = GameVariableManager::get_number(GIFT_KEY);
    if toggle == 0{ call_original!(this, flagName, rewardID, messageID, method_info); }
    else if toggle == 1 {
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_DLC_Accessory1" {return; }
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_DLC_Accessory2" {return; }
        call_original!(this, flagName, rewardID, messageID, method_info);
    }
    else if toggle == 2 {
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_Patch0" {return; }
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_Patch3" {return; }
        call_original!(this, flagName, rewardID, messageID, method_info);
    }
    else { return;  }
}
extern "C" fn gift() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<Giftmod>("Gift Options") }

pub fn gift_install(){ cobapi::install_game_setting(gift); }
