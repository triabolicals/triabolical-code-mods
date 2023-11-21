use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
pub static mut REMOVE_CUTSCENES: bool = false;
pub static mut SUPPORT_BOND_TYPE: i32 = 0;
pub static mut REMOVE_MAPDIALOGUE: bool = false;
pub static mut REMOVE_MAPTUTORIAL: bool = false;
pub static mut RNG_TYPE: i32 = 0;
pub static mut COOK_TYPE: i32 = 0;
pub static mut ARENA_SKIP: bool = false;
pub static mut LEVELDISPLAY: bool = false;
pub struct SupportMod;
impl ConfigBasicMenuItemSwitchMethods for SupportMod {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { SUPPORT_BOND_TYPE };
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        let replaceB = &[0xfd, 0x7b, 0xba, 0xa9];
        let replaceS = &[0xff,0xc3,0x01,0xd1];
        let replaceRig = &[0xC0, 0x03, 0x5F, 0xD6];
        if (result == 0){
            Patch::in_text(0x020969b0).bytes(replaceB).unwrap();
            Patch::in_text(0x02097320).bytes(replaceS).unwrap();
        }
        else if (result == 1){
            // Bond
            Patch::in_text(0x020969b0).bytes(replaceB).unwrap();
            Patch::in_text(0x02097320).bytes(replaceRig).unwrap();
        }
        else if (result == 2){ // Support
            Patch::in_text(0x020969b0).bytes(replaceRig).unwrap();
            Patch::in_text(0x02097320).bytes(replaceS).unwrap();
        }
        else if (result == 3){ // Both
            Patch::in_text(0x020969b0).bytes(replaceRig).unwrap();
            Patch::in_text(0x02097320).bytes(replaceRig).unwrap();
        }
        if toggle != result {
            unsafe { SUPPORT_BOND_TYPE = result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC = unsafe { SUPPORT_BOND_TYPE };
        if (typeC == 0) {this.help_text = format!("Play bond and support conversations in the reference menu.").into(); }
        else if (typeC == 1) { this.help_text = format!("Skip bond conversations in the reference menu.").into(); }
        else if (typeC == 2) { this.help_text = format!("Skip support conversations in the reference menu.").into(); }
        else if (typeC == 3) { this.help_text = format!("Skip bond and support conversations in the reference menu.").into();  }
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC = unsafe { SUPPORT_BOND_TYPE };
        if (typeC == 0) {this.command_text = format!("Off").into(); }
        else if (typeC == 1) { this.command_text = format!("Bond Only").into(); }
        else if (typeC == 2) { this.command_text = format!("Support Only").into(); }
        else if (typeC == 3) { this.command_text = format!("Bond and Support").into();  }
        else {this.help_text = format!("Unknown").into(); }
    }
}
pub struct ArenaMod;
impl ConfigBasicMenuItemSwitchMethods for ArenaMod {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { ARENA_SKIP };
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if (result){
            let mut instr = &[0x20,0x00, 0x80, 0x52];
            let set_false = &[0x00, 0x00, 0x80, 0x52];
            let set_return = &[0xC0, 0x03, 0x5F, 0xD6];
            let set_nop =  &[0x1F,0x20,0x03,0xD5];
            let NOP = 0x1F2003D5;
            Patch::in_text(0x01caa414).bytes(instr).unwrap();
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

            instr = &[0x60,0x00, 0x80, 0x52];
            Patch::in_text(0x01ca616c).bytes(instr).unwrap();
            instr = &[0x00,0x10, 0x20, 0x1E];
            Patch::in_text(0x01ca6124).bytes(instr).unwrap();
            instr = &[0xE1,0x03, 0x1F, 0xAA];
            Patch::in_text(0x01ca64c8).bytes(instr).unwrap();
        }
        else {
            let mut instr = &[0xbb, 0x2f, 0x1f, 0x94]; 

            Patch::in_text(0x01caa414).bytes(instr).unwrap();
            instr = &[0x54, 0xab, 0x13, 0x94]; 
            Patch::in_text(0x01bac5d0).bytes(instr).unwrap();
            instr = &[0xfd, 0x7b, 0xbe, 0xa9]; 
            Patch::in_text(0x01ca6ff0).bytes(instr).unwrap();
            instr = &[0xfd , 0x7b , 0xbe , 0xa9]; 
            Patch::in_text(0x01ca6f40).bytes(instr).unwrap();
            instr = &[0xfd , 0x7b , 0xbd , 0xa9]; 
            Patch::in_text(0x01bacb40).bytes(instr).unwrap();
            instr = &[0xf5 , 0x0b , 0x00 , 0xf9]; 
            Patch::in_text(0x01bacb44).bytes(instr).unwrap();
            instr = &[0xfd , 0x7b , 0xbc , 0xa9]; 
            Patch::in_text(0x01bac790).bytes(instr).unwrap();
            instr = &[0xf7 , 0x0b , 0x00 , 0xf9]; 
            Patch::in_text(0x01bac794).bytes(instr).unwrap();
            instr = &[0xfd , 0x7b , 0xbe , 0xa9];
            Patch::in_text(0x01bacd50).bytes(instr).unwrap();
            instr = &[0xf4 , 0x4f , 0x01 , 0xa9]; 
            Patch::in_text(0x01bacd54).bytes(instr).unwrap();
            instr = &[0xe8 , 0x0f , 0x1d , 0xfc]; 
            Patch::in_text(0x01bacf60).bytes(instr).unwrap();
            instr = &[0xfd , 0x7b , 0x01 , 0xa9]; 
            Patch::in_text(0x01bacf64).bytes(instr).unwrap();
            instr = &[0xfd , 0x7b , 0xbc , 0xa9]; 
            Patch::in_text(0x01bab900).bytes(instr).unwrap();
            instr = &[0xf7 , 0x0b , 0x00 , 0xf9]; 
            Patch::in_text(0x01bab904).bytes(instr).unwrap();
            
            instr = &[0xe1 , 0xd2 , 0x18 , 0x94]; 
            Patch::in_text(0x01bac6bc).bytes(instr).unwrap();
            instr = &[0xf9 , 0x3f , 0x30 , 0x94]; 
            Patch::in_text(0x01caa2ac).bytes(instr).unwrap();

            instr = &[0xa8 , 0xe4 , 0x1c , 0x94]; 
            Patch::in_text(0x01caa2c0).bytes(instr).unwrap();
            instr = &[0x06 , 0x0e , 0x30 , 0x94]; 
            Patch::in_text(0x01caa2d8).bytes(instr).unwrap();

            instr = &[0x4d , 0x0d , 0x30 , 0x94]; 
            Patch::in_text(0x01caa5bc).bytes(instr).unwrap();
            instr = &[0x0c , 0xe4 , 0x1c , 0x94]; 
            Patch::in_text(0x01caa5d0).bytes(instr).unwrap();
            instr = &[0x23 , 0x3f , 0x30 , 0x94]; 
            Patch::in_text(0x01caa5e4).bytes(instr).unwrap();


            instr = &[0x80 , 0x00 , 0x80 , 0x52]; 
            Patch::in_text(0x01ca616c).bytes(instr).unwrap();

            instr = &[0x00 , 0x10 , 0x2e , 0x1e]; 
            Patch::in_text(0x01ca6124).bytes(instr).unwrap();
            
            instr = &[0xe1 , 0x03 , 0x16 , 0xaa]; 
            Patch::in_text(0x01ca64c8).bytes(instr).unwrap();
        }
        if toggle != result {
            unsafe {  ARENA_SKIP = result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe { ARENA_SKIP } { this.help_text = format!("Arena battles are skipped.").into(); } 
        else { this.help_text = format!("Arena battles are not skipped.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe {ARENA_SKIP } { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}

pub struct CodeMod;
impl ConfigBasicMenuItemSwitchMethods for CodeMod {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { REMOVE_CUTSCENES };
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if (toggle){
            let replace = &[0xC0, 0x03, 0x5F, 0xD6];
            Patch::in_text(0x01ed8e20).bytes(replace).unwrap();
            Patch::in_text(0x01ed8ef0).bytes(replace).unwrap();
        }
        else {
            let replace  = &[0xFD, 0x7B, 0xBE, 0xA9];
            let replace2 = &[0xFD, 0x7B, 0xBD, 0xA9];
            Patch::in_text(0x01ed8e20).bytes(replace).unwrap();
            Patch::in_text(0x01ed8ef0).bytes(replace2).unwrap();
        }
        if toggle != result {
            unsafe {  REMOVE_CUTSCENES= result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe { REMOVE_CUTSCENES } { this.help_text = format!("Disables cutscenes and movies during chapter maps.").into(); } 
        else { this.help_text = format!("Enables cutscenes and movies during chapter maps.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe {REMOVE_CUTSCENES} { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}
pub struct CodeMod2;
impl ConfigBasicMenuItemSwitchMethods for CodeMod2 {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { REMOVE_MAPDIALOGUE };
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if (toggle){
            let replace = &[0xC0, 0x03, 0x5F, 0xD6];
            Patch::in_text(0x01ed8e20).bytes(replace).unwrap();
        }
        else {
            let replace  = &[0xfd, 0x7b, 0xbd, 0xa9];
            Patch::in_text(0x01ed8370).bytes(replace).unwrap();
        }
        if toggle != result {
            unsafe { REMOVE_MAPDIALOGUE = result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe { REMOVE_MAPDIALOGUE } { this.help_text = format!("Disables map dialogue.").into(); } 
        else { this.help_text = format!("Enables map dialogue.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe {REMOVE_MAPDIALOGUE } { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}
pub struct CodeMod3;
impl ConfigBasicMenuItemSwitchMethods for CodeMod3 {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { REMOVE_MAPTUTORIAL };
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if (toggle){
            let replace = &[0xC0, 0x03, 0x5F, 0xD6];
            Patch::in_text(0x01ed8e20).bytes(replace).unwrap();
        }
        else {
            let replace  = &[0xfd,0x7b,0xbd,0xa9];
            Patch::in_text(0x01ed91c0).bytes(replace).unwrap();
        }
        if toggle != result {
            unsafe { REMOVE_MAPTUTORIAL = result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe { REMOVE_MAPTUTORIAL } { this.help_text = format!("Disables in-map tutorials.").into(); } 
        else { this.help_text = format!("Enables in-map tutorials").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe { REMOVE_MAPTUTORIAL} { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}
pub struct CookMod;
impl ConfigBasicMenuItemSwitchMethods for CookMod {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { COOK_TYPE };
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 5, 1);
        let mut replace = &[0xe1, 0x03, 0x00, 0x2a];
        if (result == 0){ replace = &[0xe1, 0x03, 0x00, 0x2a];}
        else if (result == 1){  replace = &[0x81, 0x00, 0x80, 0x52];}
        else if (result == 2){  replace = &[0x61, 0x00, 0x80, 0x52];}
        else if (result == 3){  replace = &[0x41, 0x00, 0x80, 0x52];}
        else if (result == 4){  replace = &[0x21, 0x00, 0x80, 0x52];}
        else if (result == 5){  replace = &[0x01, 0x00, 0x80, 0x52];}
        Patch::in_text(0x02544808).bytes(replace);
        if toggle != result {
            unsafe {  COOK_TYPE = result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } 
        else { return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = format!("Sets chief quality to influence meal rating.").into();
     }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC = unsafe { COOK_TYPE };
        if (typeC == 0) {this.command_text = format!("Default").into(); }
        else if (typeC == 1) { this.command_text = format!("Horrible").into(); }
        else if (typeC == 2) { this.command_text = format!("Bad").into(); }
        else if (typeC == 3) { this.command_text = format!("Average").into();  }
        else if (typeC == 4) { this.command_text = format!("Very Good").into();  }
        else if (typeC == 5) { this.command_text = format!("Excellent").into();  }
    }
}
pub struct RNGMod;
impl ConfigBasicMenuItemSwitchMethods for RNGMod {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { RNG_TYPE };
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        let replaceH = &[0x11, 0xa0, 0x13, 0x94];
        let replaceRN = &[0xe0, 0xd7, 0x9f, 0x1a];
        let replaceRig = &[0x20, 0x00, 0x80, 0x52];
        if (result == 0){
            Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
            Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        }
        else if (result == 1){// 1 RN 
            Patch::in_text(0x02375510).bytes(replaceRig).unwrap();
            Patch::in_text(0x01e8d12c).bytes(replaceH).unwrap();
        }
        else if (result == 2){ // Hybrid
            Patch::in_text(0x01e8d12c).bytes(replaceRig).unwrap();
            Patch::in_text(0x02375510).bytes(replaceRN).unwrap();
        }
        else if (result == 3){//1 RN + Hybrid 
            Patch::in_text(0x01e8d12c).bytes(replaceRig).unwrap();
            Patch::in_text(0x02375510).bytes(replaceRig).unwrap();
        }
        if toggle != result {
            unsafe {  RNG_TYPE = result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC = unsafe { RNG_TYPE };
        if (typeC == 0) {this.help_text = format!("Default RNG behavior.").into(); }
        else if (typeC == 1) { this.help_text = format!("Disables normal RNG. (Crits, Skill Procs, Well, Cooking, etc.)").into(); }
        else if (typeC == 2) { this.help_text = format!("Disables hybrid RNG. (Hit Rates)").into(); }
        else if (typeC == 3) { this.help_text = format!("Disables normal and hybrid RNG. (No Randomness)").into();  }
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC = unsafe { RNG_TYPE };
        if (typeC == 0) {this.command_text = format!("None").into(); }
        else if (typeC == 1) { this.command_text = format!("Rig Normal").into(); }
        else if (typeC == 2) { this.command_text = format!("Rig Hybrid").into(); }
        else if (typeC == 3) { this.command_text = format!("Rig Normal/Hybrid").into();  }
        else {this.help_text = format!("Unknown").into(); }
    }
}
pub struct LevelMod;
impl ConfigBasicMenuItemSwitchMethods for LevelMod {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = unsafe { LEVELDISPLAY };
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if (toggle){
            let replace = &[0xC0, 0x03, 0x5F, 0xD6];
            Patch::in_text(0x01a5bbc0).bytes(&[0x02, 0x24, 0x82, 0x39]).unwrap();
            Patch::in_text(0x01a5bbc4).bytes(&[0x03, 0x60, 0x85, 0x39]).unwrap();
            Patch::in_text(0x01a5bbc8).bytes(&[0x40, 0x00, 0x03, 0x8B]).unwrap();
            Patch::in_text(0x01a5bbcc).bytes(replace).unwrap();

            Patch::in_text(0x01f9e280).bytes(&[0x50, 0xF6, 0xEA, 0x97]).unwrap();
            Patch::in_text(0x01f9e290).bytes(&[0x4C, 0xF6, 0xEA, 0x97]).unwrap();
            Patch::in_text(0x01c669fc).bytes(&[0x71, 0xD4, 0xF7, 0x97]).unwrap();
            Patch::in_text(0x01c66a0c).bytes(&[0x6D, 0xD4, 0xF7, 0x97]).unwrap();
            
        }
        else {
            let replace = &[0xC0, 0x03, 0x5F, 0xD6];
            Patch::in_text(0x01a5bbc0).bytes(&[0x00, 0x24, 0x42, 0x39]).unwrap();
            Patch::in_text(0x01a5bbc4).bytes( &[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
            Patch::in_text(0x01f9e280).bytes(&[0x08, 0x56, 0xea, 0x97]).unwrap();
            Patch::in_text(0x01f9e290).bytes(&[0x04, 0x56, 0xea, 0x97]).unwrap();
            Patch::in_text(0x01c669fc).bytes(&[0x29, 0x34, 0xf7, 0x97]).unwrap();
            Patch::in_text(0x01c66a0c).bytes(&[0x25, 0x34, 0xf7, 0x97]).unwrap();
        }
        if toggle != result {
            unsafe {   LEVELDISPLAY = result }
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe { LEVELDISPLAY } { this.help_text = format!("Displays unit's total level. (Internal + Displayed Level)").into(); } 
        else { this.help_text = format!("Default level display. (Displayed Level)").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if unsafe {LEVELDISPLAY} { this.command_text = format!("Total Level").into();} 
        else { this.command_text = format!("Default").into(); }
    }
}
#[no_mangle]
extern "C" fn CodeMods1() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CodeMod>("Skip Cutscenes") }
#[no_mangle]
extern "C" fn level() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<LevelMod>("Unit Level Display") }
#[no_mangle]
extern "C" fn Supports() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<SupportMod>("Skip Bond/Support Conversations") }
#[no_mangle]
extern "C" fn CodeMods2() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CodeMod2>("Skip Map Dialogue") }
#[no_mangle]
extern "C" fn CodeMods3() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CodeMod3>("Skip Map Tutorials") }
#[no_mangle]
extern "C" fn arena() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<ArenaMod>("Skip Arena Battles") }
#[no_mangle]
extern "C" fn RNG() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<RNGMod>("RNG Rigging Mode") }
#[no_mangle]
extern "C" fn cook() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CookMod>("Cooking Chief Quality") }
#[skyline::main(name = "libtriabolical")]
pub fn main() {
    //Enables support/bond viewing in maps and exploration

    let replace = &[0x1f, 0x25, 0x00, 0x71];
    Patch::in_text(0x0209950C).bytes(replace).unwrap();
    Patch::in_text(0x020994E0).bytes(replace).unwrap();
    Patch::in_text(0x02099538).bytes(replace).unwrap();

    println!("triabolical code mods are loaded");
    cobapi::install_game_setting(CodeMods1);
    cobapi::install_game_setting(Supports);
    cobapi::install_game_setting(CodeMods2);
    cobapi::install_game_setting(CodeMods3);
    cobapi::install_game_setting(level);
    cobapi::install_game_setting(arena);
    cobapi::install_game_setting(cook);
    cobapi::install_game_setting(RNG);
    

}
