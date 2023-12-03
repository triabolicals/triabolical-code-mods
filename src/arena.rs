use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;

pub const ARENA_KEY: &str = "G_ARENA_SKIP";
pub struct ArenaMod;
pub fn patchArena(){
    GameVariableManager::make_entry_norewind(ARENA_KEY, 0);
    let active = GameVariableManager::get_bool(ARENA_KEY);
    if (active){
        let set_false = &[0x00, 0x00, 0x80, 0x52];
        let set_return = &[0xC0, 0x03, 0x5F, 0xD6];
        let set_nop =  &[0x1F,0x20,0x03,0xD5];

        Patch::in_text(0x01caa414).bytes(&[0x20,0x00, 0x80, 0x52]).unwrap();
        Patch::in_text(0x01bac5d0).bytes(set_nop).unwrap();
        Patch::in_text(0x01ca6f40).bytes(set_return).unwrap();
        Patch::in_text(0x01ca6ff0).bytes(set_return).unwrap();

        Patch::in_text(0x01bacb40).bytes(set_false).unwrap();
        Patch::in_text(0x01bacb44).bytes(set_return).unwrap();

        Patch::in_text(0x01bac790).bytes(set_false).unwrap();
        Patch::in_text(0x01bac794).bytes(set_return).unwrap();

        Patch::in_text(0x01bacd50).bytes(set_false).unwrap();
        Patch::in_text(0x01bacd54).bytes(set_return).unwrap();

        Patch::in_text(0x01bacf60).bytes(set_false).unwrap();
        Patch::in_text(0x01bacf64).bytes(set_return).unwrap();

        Patch::in_text(0x01bab900).bytes(set_false).unwrap();
        Patch::in_text(0x01bab904).bytes(set_return).unwrap();

        Patch::in_text(0x01bac6bc).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa2ac).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa2c0).bytes(set_nop).unwrap();

        Patch::in_text(0x01caa2d8).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa5bc).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa5d0).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa5e4).bytes(set_nop).unwrap();

        Patch::in_text(0x01ca616c).bytes(&[0x60,0x00, 0x80, 0x52]).unwrap();
        Patch::in_text(0x01ca6124).bytes(&[0x00,0x10, 0x20, 0x1E]).unwrap();
        Patch::in_text(0x01ca64c8).bytes(&[0xE1,0x03, 0x1F, 0xAA]).unwrap();
        println!("Arena battles are skipped");
    }
    else {
        Patch::in_text(0x01caa414).bytes(&[0xbb, 0x2f, 0x1f, 0x94]).unwrap();
        Patch::in_text(0x01bac5d0).bytes(&[0x54, 0xab, 0x13, 0x94]).unwrap();
        Patch::in_text(0x01ca6ff0).bytes(&[0xfd, 0x7b, 0xbe, 0xa9]).unwrap();
        Patch::in_text(0x01ca6f40).bytes(&[0xfd , 0x7b , 0xbe , 0xa9]).unwrap(); 
        Patch::in_text(0x01bacb40).bytes(&[0xfd , 0x7b , 0xbd , 0xa9]).unwrap();
        Patch::in_text(0x01bacb44).bytes(&[0xf5 , 0x0b , 0x00 , 0xf9]).unwrap();
        Patch::in_text(0x01bac790).bytes(&[0xfd , 0x7b , 0xbc , 0xa9]).unwrap();
        Patch::in_text(0x01bac794).bytes(&[0xf7 , 0x0b , 0x00 , 0xf9]).unwrap();
        Patch::in_text(0x01bacd50).bytes(&[0xfd , 0x7b , 0xbe , 0xa9]).unwrap();
        Patch::in_text(0x01bacd54).bytes(&[0xf4 , 0x4f , 0x01 , 0xa9]).unwrap();
        Patch::in_text(0x01bacf60).bytes(&[0xe8 , 0x0f , 0x1d , 0xfc]).unwrap();
        Patch::in_text(0x01bacf64).bytes(&[0xfd , 0x7b , 0x01 , 0xa9]).unwrap();
        Patch::in_text(0x01bab900).bytes(&[0xfd , 0x7b , 0xbc , 0xa9]).unwrap();
        Patch::in_text(0x01bab904).bytes(&[0xf7 , 0x0b , 0x00 , 0xf9]).unwrap();
        Patch::in_text(0x01bac6bc).bytes(&[0xe1 , 0xd2 , 0x18 , 0x94]).unwrap();
        Patch::in_text(0x01caa2ac).bytes(&[0xf9 , 0x3f , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01caa2c0).bytes(&[0xa8 , 0xe4 , 0x1c , 0x94]).unwrap();
        Patch::in_text(0x01caa2d8).bytes(&[0x06 , 0x0e , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01caa5bc).bytes(&[0x4d , 0x0d , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01caa5d0).bytes(&[0x0c , 0xe4 , 0x1c , 0x94]).unwrap();
        Patch::in_text(0x01caa5e4).bytes(&[0x23 , 0x3f , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01ca616c).bytes(&[0x80 , 0x00 , 0x80 , 0x52]).unwrap();
        Patch::in_text(0x01ca6124).bytes(&[0x00 , 0x10 , 0x2e , 0x1e]).unwrap();
        Patch::in_text(0x01ca64c8).bytes(&[0xe1 , 0x03 , 0x16 , 0xaa]).unwrap();
        println!("Arena battles are not skipped");
    }
}
impl ConfigBasicMenuItemSwitchMethods for ArenaMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchArena();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let active = GameVariableManager::get_bool(ARENA_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(active);

        if active != result {
            GameVariableManager::set_bool(ARENA_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchArena();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active = GameVariableManager::get_bool(ARENA_KEY);
        if (active) { this.help_text = format!("Arena battles are skipped.").into(); } 
        else { this.help_text = format!("Arena battles are not skipped.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active = GameVariableManager::get_bool(ARENA_KEY);
        if (active) { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}
#[no_mangle]
extern "C" fn arena() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<ArenaMod>("Skip Arena Battles") }


pub fn arena_install(){
  cobapi::install_game_setting(arena);
}