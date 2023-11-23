use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use crate::game_var_i;

pub const COOK_KEY: &str = "G_CHIEF";
pub struct CookMod;


fn patchCook(){
    GameVariableManager::make_entry_norewind(COOK_KEY, 0);
    let result =  game_var_i::getNumber(COOK_KEY);
    let mut replace = &[0xe1, 0x03, 0x00, 0x2a];
    if (result == 0){ replace = &[0xe1, 0x03, 0x00, 0x2a];}
    else if (result == 1){  replace = &[0x81, 0x00, 0x80, 0x52];}
    else if (result == 2){  replace = &[0x61, 0x00, 0x80, 0x52];}
    else if (result == 3){  replace = &[0x41, 0x00, 0x80, 0x52];}
    else if (result == 4){  replace = &[0x21, 0x00, 0x80, 0x52];}
    else if (result == 5){  replace = &[0x01, 0x00, 0x80, 0x52];}
    Patch::in_text(0x02544808).bytes(replace);
}

impl ConfigBasicMenuItemSwitchMethods for CookMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchCook();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = game_var_i::getNumber(COOK_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 5, 1);
        if toggle != result {
            game_var_i::setNumber(COOK_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchCook();
            return BasicMenuResult::se_cursor();
        } 
        else { return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = format!("Sets chief quality to influence meal rating.").into();
     }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C = game_var_i::getNumber(COOK_KEY);
        if type_C == 0 {this.command_text = format!("Default").into(); }
        else if type_C == 1 { this.command_text = format!("Horrible").into(); }
        else if type_C == 2 { this.command_text = format!("Bad").into(); }
        else if type_C == 3 { this.command_text = format!("Average").into();  }
        else if type_C == 4 { this.command_text = format!("Very Good").into();  }
        else if type_C == 5 { this.command_text = format!("Excellent").into();  }
    }
}
#[no_mangle]
extern "C" fn cook() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CookMod>("Cooking Chief Quality") }

pub fn cook_install(){
    cobapi::install_game_setting(cook);

}