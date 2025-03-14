use unity::{il2cpp::object::Array, prelude::*};
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamevariable::*,
    script::*,
};
use super::*;

pub struct CutsceneMod;
impl ConfigBasicMenuItemSwitchMethods for CutsceneMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(CUTSCENES_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(CUTSCENES_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = if GameVariableManager::get_bool(CUTSCENES_KEY){  "Disables cutscenes and movies during chapter maps." } 
            else { "Enables cutscenes and movies during chapter maps." }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = if GameVariableManager::get_bool(CUTSCENES_KEY) {  crate::utils::on_str()}  else { crate::utils::off_str() };
    }
}

extern "C" fn cutscene() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CutsceneMod>("Skip Cutscenes/Movies") }
pub fn cutscene_install(){ cobapi::install_game_setting(cutscene); }

#[unity::from_offset("App", "ScriptSystem", "Movie")]
fn script_movie(arg: &Array<DynValue>, method_info: OptionalMethod);

#[unity::from_offset("App", "ScriptSystem", "PuppetDemo")]
fn script_puppet_demo(arg: &Array<DynValue>, method_info: OptionalMethod);

pub extern "C" fn movie(arg: &Array<DynValue>, method_info: OptionalMethod) {
    if  GameVariableManager::get_bool(CUTSCENES_KEY) { return; }
    else { unsafe { script_movie(arg, method_info) };  }
}

pub extern "C" fn puppet_demo(arg: &Array<DynValue>, method_info: OptionalMethod) {
    if  GameVariableManager::get_bool(CUTSCENES_KEY) { return; }
    else { unsafe { script_puppet_demo(arg, method_info) };  }
}