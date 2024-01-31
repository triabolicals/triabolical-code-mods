use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use crate::string::*;
pub const SUPPORT_KEY: &str = "G_SUPPORT_TYPE";

pub fn patchSupport() {
    GameVariableManager::make_entry_norewind(SUPPORT_KEY, 0);
    let active =  GameVariableManager::get_number(SUPPORT_KEY);
    let replaceB = &[0xfd, 0x7b, 0xba, 0xa9];
    let replaceS = &[0xff,0xc3,0x01,0xd1];
    let replaceRig = &[0xC0, 0x03, 0x5F, 0xD6];
    if active == 0{
        Patch::in_text(0x020969b0).bytes(replaceB).unwrap();
        Patch::in_text(0x02097320).bytes(replaceS).unwrap();
    }
    else if active == 1{
        Patch::in_text(0x020969b0).bytes(replaceB).unwrap();
        Patch::in_text(0x02097320).bytes(replaceRig).unwrap();
    }
    else if active == 2{ // Support
        Patch::in_text(0x020969b0).bytes(replaceRig).unwrap();
        Patch::in_text(0x02097320).bytes(replaceS).unwrap();
    }
    else if active == 3{ // Both
        Patch::in_text(0x020969b0).bytes(replaceRig).unwrap();
        Patch::in_text(0x02097320).bytes(replaceRig).unwrap();
    }
}

pub struct SupportMod;
impl ConfigBasicMenuItemSwitchMethods for SupportMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchSupport();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(SUPPORT_KEY);;
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(SUPPORT_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchSupport();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(SUPPORT_KEY);;
        if typeC == 0 {this.help_text = format!("Play bond and support conversations in the reference menu.").into(); }
        else if typeC == 1 { this.help_text = format!("Skip bond conversations in the reference menu.").into(); }
        else if typeC == 2 { this.help_text = format!("Skip support conversations in the reference menu.").into(); }
        else if typeC == 3 { this.help_text = format!("Skip bond and support conversations in the reference menu.").into();  }
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        unsafe {
        let typeC =  GameVariableManager::get_number(SUPPORT_KEY);
        if typeC == 0 {this.command_text = Off_str(); }
        else if typeC == 1 { this.command_text = get_mess_str("MID_MENU_Recall_GodReliance_Unit"); }
        else if typeC == 2 { this.command_text = get_mess_str("MID_MENU_Recall_Reliance_Unit"); }
        else if typeC == 3 { this.command_text = concat_strings3(get_mess_str("MID_MENU_Recall_Reliance_Unit"), " / ".into(), get_mess_str("MID_MENU_Recall_GodReliance_Unit"), None ); }
        else {this.help_text = format!("Unknown").into(); }
    }
}
}

#[no_mangle]
extern "C" fn supports() -> &'static mut ConfigBasicMenuItem { 
    unsafe {
    let label = concat_strings3(get_mess_str("MID_MENU_Recall_Reliance_Unit"), " / ".into(), get_mess_str("MID_MENU_Recall_GodReliance_Unit"), None ); 
    ConfigBasicMenuItem::new_switch::<SupportMod>(concat_strings("Skip ".into(), label, None).get_string().unwrap())
    }
 }

pub fn support_install(){
    cobapi::install_game_setting(supports);
}
