use super::*;

pub struct AIMod;
impl ConfigBasicMenuItemSwitchMethods for AIMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number(AI_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 2, 1);
        if toggle != result {
            GameVariableManager::set_number(AI_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_ignorance();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = match GameVariableManager::get_number(AI_KEY) {
            1 => { "Displays unit's AI/support bonuses in unit description text."},
            2 => { "Displays as little information as possible."},
            _ => { "Unit description text will not contain AI/support data." }
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(AI_KEY) {
            1 => { "AI/Support"},
            2 => { "Ignorance"},
            _ => { "Normal" },
        }.into();
    }
}

extern "C" fn ai() -> &'static mut ConfigBasicMenuItem {  ConfigBasicMenuItem::new_switch::<AIMod>("Unit Data Mode") }
pub fn ai_install(){ cobapi::install_game_setting(ai); }

pub fn patch_ignorance() {
    //return;
    if GameVariableManager::get_number(AI_KEY) == 2 {
        let w2_0 = &[0x02, 0x00, 0x80, 0x52];
        let ret = &[0xc0, 0x03, 0x5f, 0xd6];
        let pop_up_offsets = [ 0x01f47c90, 0x02997f80, ];
        for x in pop_up_offsets {
            Patch::in_text(x).bytes(ret).unwrap();
        }
        //Patch::in_text(0x02089968).bytes(&[0x00, 0x80, 0x52, 0x20]).unwrap();
        Patch::in_text(0x01c6576c).nop().unwrap();
        Patch::in_text(0x01f9e120).nop().unwrap();
        Patch::in_text(0x023584a0).nop().unwrap();
        Patch::in_text(0x0207b520).bytes(w2_0).unwrap();
        Patch::in_text(0x0207bb7c).bytes(w2_0).unwrap();
        Patch::in_text(0x0207c08c).bytes(w2_0).unwrap();
        Patch::in_text(0x02089830).bytes(&[0x00, 0x00, 0x80, 0x52]).unwrap();
        Patch::in_text(0x02089834).bytes(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    }

    else {
        let w2_0_revert = &[0xe2, 0x17, 0x9f, 0x1a];
        Patch::in_text(0x01f47c90).bytes(&[0xff, 0x03, 0x01, 0xd1]).unwrap();
        Patch::in_text(0x02997f80).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();
        Patch::in_text(0x01c6576c).bytes(&[0xa0, 0x1a, 0x00, 0x54]).unwrap();
        Patch::in_text(0x01f9e120).bytes(&[0xe0, 0x01, 0x00, 0x54]).unwrap();
        Patch::in_text(0x023584a0).bytes(&[0x20, 0x01, 0x00, 0x54]).unwrap();
        Patch::in_text(0x0207b520).bytes(w2_0_revert).unwrap();
        Patch::in_text(0x0207b520).bytes(w2_0_revert).unwrap();
        Patch::in_text(0x0207c08c).bytes(w2_0_revert).unwrap();
        Patch::in_text(0x02089830).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();
        Patch::in_text(0x02089834).bytes(&[0xf7, 0x0b, 0x00, 0xf9]).unwrap();
        //Patch::in_text(0x02089968).bytes(&[0x20, 0x80, 0x52, 0x20]).unwrap();

    }
}