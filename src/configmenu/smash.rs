use super::*;

pub struct SmashMod;
impl ConfigBasicMenuItemSwitchMethods for SmashMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_smash();  }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(SMASH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(SMASH_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_smash(); 
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = 
            if GameVariableManager::get_bool(SMASH_KEY) { "First hit of every attack will smash."}
            else { "Default behavior for smash attacks." }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text =  if GameVariableManager::get_bool(SMASH_KEY) { crate::utils::on_str() }  else {  crate::utils::off_str() };
    }
}

pub extern "C" fn smash() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<SmashMod>("All Smash Attacks") }

pub fn install_smash() { cobapi::install_game_setting(smash);}
pub fn patch_smash() {
    if GameVariableManager::get_bool(SMASH_KEY) {
        Patch::in_text(0x02472714).bytes(&[0x80, 0x0C, 0x80, 0x52]).unwrap();
        Patch::in_text(0x02472CB8).bytes(&[0x8B, 0x02, 0x00, 0x54]).unwrap();
        Patch::in_text(0x02472758).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
        println!("Smashing activated");
    }
    else {
        Patch::in_text(0x02472714).bytes(&[0xAB, 0x0F, 0xE8, 0x97]).unwrap();
        Patch::in_text(0x02472CB8).bytes(&[0x81, 0x02, 0x00, 0x54]).unwrap();
        Patch::in_text(0x02472758).bytes(&[0xb2, 0x0f, 0xe8, 0x97]).unwrap();
        println!("Smashing deactivated");
    }
}

