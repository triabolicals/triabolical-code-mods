use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use crate::string::*;
pub const SUPPORT_KEY: &str = "G_SUPPORT_TYPE";

pub fn patchSupport() {
    GameVariableManager::make_entry_norewind(SUPPORT_KEY, 0);
    let active =  GameVariableManager::get_number(SUPPORT_KEY);
    let replaceB = &[0xc0, 0x24, 0x11, 0x14];
    let replaceS = &[0x51,0x24,0x011,0x94];
    let replaceRig = &[0xC0, 0x03, 0x5F, 0xD6];

    let duration_0 = &[0xE0, 0x03, 0x27, 0x1E];
    let duration_25 = &[0x00, 0x10, 0x2a, 0x1e];

    // Support 
   // Patch::in_text(0x02097af0).bytes(replaceRig);   // 0x08, 0x32, 0x40, 0xf9
    /*
    Patch::in_text(0x02097c90).bytes(replaceRig).unwrap();
    Patch::in_text(0x020987ec).nop();   //0x45, 0x61, 0x07, 0x94 
    Patch::in_text(0x02096d7c).bytes(duration_0); 
    Patch::in_text(0x02097028).bytes(duration_0);

    Patch::in_text(0x02097c90).bytes(replaceB);
    Patch::in_text(0x020987ec).bytes(&[0x45, 0x61, 0x07, 0x94]);
    Patch::in_text(0x02096d7c).bytes(duration_25);
    Patch::in_text(0x02097028).bytes(duration_25);


    Patch::in_text(0x02097e4c).bytes(replaceRig);
    Patch::in_text(0x020988a4).bytes(replaceRig); 
    Patch::in_text(0x02098a60).bytes(replaceRig); //0x67, 0x22, 0x05, 0x14
    */
    if active == 0{
        Patch::in_text(0x02097c90).bytes(replaceB);
        Patch::in_text(0x020987ec).bytes(&[0x45, 0x61, 0x07, 0x94]);
        Patch::in_text(0x02096d7c).bytes(duration_25);
        Patch::in_text(0x02097028).bytes(duration_25);

        Patch::in_text(0x02097e4c).bytes(replaceS);
        Patch::in_text(0x020988a4).bytes(&[0x67, 0x22, 0x05, 0x14]);
        Patch::in_text(0x02098a60).bytes(&[0xf8, 0x21, 0x05, 0x14]);
    }
    else if active == 1{
        //Bonds
        Patch::in_text(0x02096d7c).bytes(duration_0); 
        Patch::in_text(0x02097028).bytes(duration_0);
        //Supports normal
        Patch::in_text(0x02097c90).bytes(replaceB);
        Patch::in_text(0x020987ec).bytes(&[0x45, 0x61, 0x07, 0x94]);

        Patch::in_text(0x02097e4c).nop();
        Patch::in_text(0x020988a4).bytes(replaceRig); 
        Patch::in_text(0x02098a60).bytes(replaceRig);
    }
    else if active == 2 { // Support
        Patch::in_text(0x02096d7c).bytes(duration_0); 
        Patch::in_text(0x02097028).bytes(duration_0);
        //bonds normal
        Patch::in_text(0x02097e4c).bytes(replaceS);
        Patch::in_text(0x020988a4).bytes(&[0x67, 0x22, 0x05, 0x14]);
        Patch::in_text(0x02098a60).bytes(&[0xf8, 0x21, 0x05, 0x14]);

        Patch::in_text(0x02097c90).bytes(replaceRig).unwrap();
        Patch::in_text(0x020987ec).nop(); 
    }
    else if active == 3{
        Patch::in_text(0x02096d7c).bytes(duration_0); 
        Patch::in_text(0x02097028).bytes(duration_0);

        Patch::in_text(0x02097e4c).nop();
        Patch::in_text(0x020988a4).bytes(replaceRig); 
        Patch::in_text(0x02098a60).bytes(replaceRig);

        Patch::in_text(0x02097c90).bytes(replaceRig).unwrap();
        Patch::in_text(0x020987ec).nop(); 

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
