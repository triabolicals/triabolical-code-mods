use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;

pub const GIFT_KEY: &str = "G_GIFT";

pub fn patch_gift(){
    Patch::in_text(0x023f3f18).bytes(&[0x3a,0xff,0xff,0x17]);   //DLC0
    Patch::in_text(0x023f3fc8).bytes(&[0x0e,0xff,0xff,0x17]);   //Patch0
    Patch::in_text(0x023f4088).bytes(&[0xde,0xfe,0xff,0x17]);   //DLC1
    Patch::in_text(0x023f4138).bytes(&[0xb2,0xfe,0xff, 0x17]);  //Patch3
    let ret = &[0xC0, 0x03, 0x5F, 0xD6];
    if GameVariableManager::get_number(GIFT_KEY) == 1 {
        Patch::in_text(0x023f3f18).bytes(ret);
        Patch::in_text(0x023f4088).bytes(ret);
    }
    else if GameVariableManager::get_number(GIFT_KEY) == 2 {
        Patch::in_text(0x023f3fc8).bytes(ret);
        Patch::in_text(0x023f4138).bytes(ret);
    }
    else if GameVariableManager::get_number(GIFT_KEY) == 3 {
        Patch::in_text(0x023f3f18).bytes(ret);
        Patch::in_text(0x023f4088).bytes(ret);
        Patch::in_text(0x023f3fc8).bytes(ret);
        Patch::in_text(0x023f4138).bytes(ret);
    }
}

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
            patch_gift();
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
extern "C" fn gift() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<Giftmod>("Gift Options") }

pub fn gift_install(){ cobapi::install_game_setting(gift); }
