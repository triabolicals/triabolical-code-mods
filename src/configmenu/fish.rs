use super::*;

pub struct FishingMod;
impl ConfigBasicMenuItemSwitchMethods for FishingMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){  GameVariableManager::make_entry(FISH_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(FISH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(FISH_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = if GameVariableManager::get_bool(FISH_KEY) {  "Autofisher will only catch high bond fragment fish." } 
            else { "Autofisher will catch random fish based on rod." }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = if GameVariableManager::get_bool(FISH_KEY) { "Max Bond"} 
        else {  "Default" }.into();
    }
}
extern "C" fn fishing() -> &'static mut ConfigBasicMenuItem {  ConfigBasicMenuItem::new_switch::<FishingMod>("AutoFisher Setting") }
pub fn fishing_install(){ cobapi::install_game_setting(fishing); }