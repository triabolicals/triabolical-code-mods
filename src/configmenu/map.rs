use super::*;

pub struct MapMod;
impl ConfigBasicMenuItemSwitchMethods for MapMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_map(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(MAP_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 4, 1);
        if toggle != result {
            GameVariableManager::set_number(MAP_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_map();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active  =  GameVariableManager::get_number(MAP_KEY);
        if active == 0 { this.help_text = "Enables map dialogue and tutorials.".into(); } 
        else if active == 1 { this.help_text = "Skips tutorials.".into(); }
        else if active == 2 { this.help_text = "Skips map dialogue.".into(); }
        else if active == 3 { this.help_text = "Skips tutorials and dialogue.".into(); }
        else if active == 4 { this.help_text = "Skips tutorials and all dialogue.".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let active  =  GameVariableManager::get_number(MAP_KEY);
        if active == 0 { this.command_text = "Off".into(); } 
        else if active == 1 { this.command_text = "Tutorials".into(); }
        else if active == 2 { this.command_text = "In-Map Dialogue".into(); }
        else if active == 3 { this.command_text = "Tutorials/In-Dialogue".into(); }
        else if active == 4 { this.command_text  = "Tutorials/All-Dialogue".into(); }
    }
}

pub fn clear_all_tutorials() {
    if GameVariableManager::exist("G_TUT_CLEARED") { return; }
    if GameVariableManager::get_number(MAP_KEY) != 0  {
        GameVariableManager::find_starts_with("G_解説_").iter().for_each(|g_key| GameVariableManager::set_bool(g_key.to_string().as_str(), true));
        GameVariableManager::make_entry("G_TUT_CLEARED", 1);
    }
}

extern "C" fn maps() -> &'static mut ConfigBasicMenuItem {  engage::menu::config::ConfigBasicMenuItem::new_switch::<MapMod>("Skip Map Dialogue/Tutorials") }

pub fn map_mod_install() { cobapi::install_game_setting(maps);}

pub fn patch_map(){
    let active = GameVariableManager::get_number(MAP_KEY);
    let ret = &[0xC0, 0x03, 0x5F, 0xD6];
    if active == 0 { //None
        Patch::in_text(0x01ed91c0).bytes(&[0xfd,0x7b,0xbd,0xa9]).unwrap();
        Patch::in_text(0x01ed8370).bytes(&[0xfd, 0x7b, 0xbd, 0xa9]).unwrap();

        Patch::in_text(0x01a01840).bytes(&[0xfd, 0x7b, 0xbd, 0xa9]).unwrap();  // TutorialSequence Create Bind
        Patch::in_text(0x01a010d0).bytes(&[0xff, 0x83, 0x01, 0xd1]).unwrap();
        Patch::in_text(0x020b81f0).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();    // Talk Create Bind
    }
    else if active == 1 { // Tutorial
        Patch::in_text(0x01ed91c0).bytes(ret).unwrap();
        Patch::in_text(0x01ed8370).bytes(&[0xfd, 0x7b, 0xbd, 0xa9]).unwrap();
        println!("Map Tutorials are skipped");

        Patch::in_text(0x01a01840).bytes(ret).unwrap();   // TutorialSequence Create Bind
        Patch::in_text(0x01a010d0).bytes(ret).unwrap();    // TutorialSequence Create Bind
        Patch::in_text(0x020b81f0).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();    // Talk Create Bind
    }
    else if active == 2 {//Map dialogue
        Patch::in_text(0x01ed8370).bytes(ret).unwrap();
        Patch::in_text(0x01ed91c0).bytes(&[0xfd,0x7b,0xbd,0xa9]).unwrap();
        // Patch::in_text(0x020b81f0).bytes(ret).unwrap();   // Talk Create Bind does nothing
        Patch::in_text(0x01a01840).bytes(&[0xfd, 0x7b, 0xbd, 0xa9]).unwrap();  // TutorialSequence Create Bind
        Patch::in_text(0x01a010d0).bytes(&[0xff, 0x83, 0x01, 0xd1]).unwrap();
        Patch::in_text(0x020b81f0).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();   // Talk Create Bind
        println!("Map Dialogue are skipped");
    }
    else if active == 3 { //Both
        Patch::in_text(0x01ed91c0).bytes(ret).unwrap();
        Patch::in_text(0x01ed8370).bytes(ret).unwrap();

        Patch::in_text(0x01a01840).bytes(ret).unwrap();   // TutorialSequence Create Bind
        Patch::in_text(0x01a010d0).bytes(ret).unwrap();    // TutorialSequence Create Bind
        Patch::in_text(0x020b81f0).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]).unwrap();    // Talk Create Bind
        println!("Map Tutorials and Dialogue are skipped");
    }
    else if active == 4 {
        Patch::in_text(0x01ed91c0).bytes(ret).unwrap();
        Patch::in_text(0x01ed8370).bytes(ret).unwrap();

        Patch::in_text(0x020b81f0).bytes(ret).unwrap();   // Talk Create Bind does nothing
        Patch::in_text(0x01a01840).bytes(ret).unwrap();   // TutorialSequence Create Bind
        Patch::in_text(0x01a010d0).bytes(ret).unwrap();    // TutorialSequence Create Bind
        println!("Map Tutorials and All Dialogue are skipped");
    }
}
