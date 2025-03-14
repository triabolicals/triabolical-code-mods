use super::*;

pub struct LevelMod;
impl ConfigBasicMenuItemSwitchMethods for LevelMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry(LEVEL_DIS_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(LEVEL_DIS_KEY, 0);
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(LEVEL_DIS_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = "Displays stat increases in the stat help box.".into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = if GameVariableManager::get_bool(LEVEL_DIS_KEY) { crate::utils::on_str() } else { crate::utils::off_str() };
    }
}

extern "C" fn level_callback() -> &'static mut ConfigBasicMenuItem {  ConfigBasicMenuItem::new_switch::<LevelMod>("Next Level Display")}

pub fn level_install(){ cobapi::install_game_setting(level_callback); }