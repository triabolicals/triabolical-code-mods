use skyline::patching::Patch;
use unity::prelude::*;
use std::ops::Deref;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gameuserdata::GameUserData;
use engage::{gamedata::*, singleton::SingletonClass};
use engage::{gamedata::person::*};

pub const CHARACTER_KEY: &str = "G_CHARACTER";
pub const LEVEL_DIS_KEY: &str = "G_LEVEL_TYPE";
pub const GROWTH_KEY: &str = "G_GROWTH_TYPE";
pub static mut setPerson: bool = false;
pub const NameArray : &[&str] = &["MPID_Vandre", "MPID_Clan", "MPID_Fram", "MPID_Alfred", "MPID_Etie", "MPID_Boucheron", "MPID_Celine", "MPID_Chloe", "MPID_Louis", "MPID_Yunaka", "MPID_Staluke", "MPID_Citrinica", "MPID_Lapis", "MPID_Diamand", "MPID_Umber", "MPID_Jade", "MPID_Ivy", "MPID_Kagetsu", "MPID_Zelkova", "MPID_Fogato", "MPID_Pandoro", "MPID_Bonet", "MPID_Misutira", "MPID_Panetone", "MPID_Merin", "MPID_Hortensia", "MPID_Seadas", "MPID_Rosado", "MPID_Goldmary", "MPID_Linden", "MPID_Saphir", "MPID_Veyre", "MPID_Mauve", "MPID_Anna", "MPID_Jean" ];
pub const OptionArray : &[&str] = &["None", "Vander", "Clanne", "Framme", "Alfred", "Etie", "Boucheron", "Céline", "Chloé", "Louis", "Yunaka", "Alcryst", "Citrinne", "Lapis", "Diamant", "Amber", "Jade", "Ivy", "Kagetsu", "Zelkov", "Fogato", "Pandero", "Bunet", "Timerra", "Panette", "Merrin", "Hortensia", "Seadall", "Rosado", "Goldmary", "Linden", "Saphir", "Veyle", "Mauvier", "Anna", "Jean"];
pub const HelpArray : &[&str] = &["MPID_H_Vandre", "MPID_H_Clan", "MPID_H_Fram", "MPID_H_Alfred", "MPID_H_Etie", "MPID_H_Boucheron", "MPID_H_Celine", "MPID_H_Chloe", "MPID_H_Louis", "MPID_H_Yunaka", "MPID_H_Staluke", "MPID_H_Citrinica", "MPID_H_Lapis", "MPID_H_Diamand", "MPID_H_Umber", "MPID_H_Jade", "MPID_H_Ivy", "MPID_H_Kagetsu", "MPID_H_Zelkova", "MPID_H_Fogato", "MPID_H_Pandoro", "MPID_H_Bonet", "MPID_H_Misutira", "MPID_H_Panetone", "MPID_H_Merin", "MPID_H_Hortensia", "MPID_H_Seadas", "MPID_H_Rosado", "MPID_H_Goldmary", "MPID_H_Linden", "MPID_H_Saphir", "MPID_H_Veyre", "MPID_H_Mauve", "MPID_H_Anna", "MPID_H_Jean" ];

pub fn patchLvl(){
    GameVariableManager::make_entry_norewind(LEVEL_DIS_KEY, 0);
    let result = GameVariableManager::get_bool(LEVEL_DIS_KEY);
    if (result){
        Patch::in_text(0x01a5bbc0).bytes(&[0x02, 0x24, 0x82, 0x39]).unwrap();
        Patch::in_text(0x01a5bbc4).bytes(&[0x03, 0x60, 0x85, 0x39]).unwrap();
        Patch::in_text(0x01a5bbc8).bytes(&[0x40, 0x00, 0x03, 0x8B]).unwrap();
        Patch::in_text(0x01a5bbcc).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
        Patch::in_text(0x01f9e280).bytes(&[0x50, 0xF6, 0xEA, 0x97]).unwrap();
        Patch::in_text(0x01f9e290).bytes(&[0x4C, 0xF6, 0xEA, 0x97]).unwrap();
        Patch::in_text(0x01c669fc).bytes(&[0x71, 0xD4, 0xF7, 0x97]).unwrap();
        Patch::in_text(0x01c66a0c).bytes(&[0x6D, 0xD4, 0xF7, 0x97]).unwrap();
        println!("Level Display switch to Total");
    }
    else {
        Patch::in_text(0x01a5bbc0).bytes(&[0x00, 0x24, 0x42, 0x39]).unwrap();
        Patch::in_text(0x01a5bbc4).bytes( &[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
        Patch::in_text(0x01f9e280).bytes(&[0x08, 0x56, 0xea, 0x97]).unwrap();
        Patch::in_text(0x01f9e290).bytes(&[0x04, 0x56, 0xea, 0x97]).unwrap();
        Patch::in_text(0x01c669fc).bytes(&[0x29, 0x34, 0xf7, 0x97]).unwrap();
        Patch::in_text(0x01c66a0c).bytes(&[0x25, 0x34, 0xf7, 0x97]).unwrap();
        println!("Level Display switch to Default");
    }
}
fn restoreDefault(){
    //Growth Mode Call
    Patch::in_text(0x01a3a3c4).bytes(&[0xe7, 0x6a, 0x2b, 0x94]).unwrap();
    //Random
    Patch::in_text(0x01a3a658).bytes(&[0x14,0x81,0x40, 0x39]).unwrap();
    //Random RNG 
    Patch::in_text(0x01a3a73c).bytes(&[0x5d, 0xeb, 0x24, 0x94]).unwrap();
    //Fixed
    Patch::in_text(0x01a3a410).bytes(&[0x14,0x81,0x40, 0x39]).unwrap();
}
pub fn patchGrowth(){
    GameVariableManager::make_entry_norewind(GROWTH_KEY, 0);
    let result =  GameVariableManager::get_number(GROWTH_KEY);
    restoreDefault();
    if (result == 0 ){ 
        println!("Growth set to save file default");
        restoreDefault(); 
    }
    else if (result == 1){
        //Opposite Mode
        let growthMode = GameUserData::get_grow_mode();
        if (growthMode == 0) {//Random -> Fixed
            Patch::in_text(0x01a3a3c4).bytes(&[0x20, 0x00, 0x80, 0xd2]).unwrap();
            println!("Growth set to 'Fixed' from save file default of 'Random'");
        }
        else { //Fixed -> Random
            Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
            println!("Growth set to 'Random' from save file default of 'Fixed'");
        }
    }
    else if (result == 2) {
        // No Growths
        Patch::in_text(0x01a3a410).bytes(&[0x14,0x00,0x80,0xD2]).unwrap();
        Patch::in_text(0x01a3a658).bytes(&[0x14,0x00, 0x80,0xD2]).unwrap();
        println!("Growth set to 'No Growths'");
    }
    else if (result == 3){
        // Perfect Level Ups, forcing to Random and RNG set to 1
        Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
        Patch::in_text(0x01a3a73c).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
        println!("Growth set to 'Perfect'");
    }
}
pub fn set_Person(){
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    let seted = unsafe { setPerson };

    if seted == false {
        unsafe {
            println!("Copying playable characters to unused character slots");
            for x in 2..33 {   
                set_person(t_list[x], t_list[902+x]);
            }
            // Veyle 39
            set_person(t_list[39], t_list[935]);
            // Mauvier 49
            set_person(t_list[49], t_list[936]);
            // Anna 51 
            set_person(t_list[51], t_list[937]);
            // Jean 52
            set_person(t_list[52], t_list[938]);
        }
    }
    unsafe { setPerson = true; }
}
pub fn changeCharacters(){
    GameVariableManager::make_entry_norewind(CHARACTER_KEY, 0);
    let result =  GameVariableManager::get_number(CHARACTER_KEY);
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    set_Person();
    if result == 0 {
        unsafe {
            for x in 2..33 {   
                set_person(t_list[902+x], t_list[x]);
                set_name(t_list[x], NameArray[x-2].into(), None);
                set_help(t_list[x], HelpArray[x-2].into(), None);
            }
            // Veyle 39
            set_person(t_list[935], t_list[39]);
            set_name(t_list[39], NameArray[31].into(), None);
            set_help(t_list[39], HelpArray[31].into(), None);
            // Mauvier 49
            set_person(t_list[936], t_list[49]);
            set_name(t_list[49], NameArray[32].into(), None);
            set_help(t_list[49], HelpArray[32].into(), None);

            // Anna 51 
            set_person(t_list[937], t_list[51]);
            set_name(t_list[51], NameArray[33].into(), None);
            set_help(t_list[51], HelpArray[33].into(), None);

            // Jean 52
            set_person(t_list[938], t_list[52]);
            set_name(t_list[52], NameArray[34].into(), None);
            set_help(t_list[52], HelpArray[34].into(), None);
            println!("Characters are set to default.");
        }
    }
    else {
        let index: usize = (903 + result).try_into().unwrap();
        let index2: usize = (result -1).try_into().unwrap();
        let name = NameArray[index2];
        let help = HelpArray[index2];
        unsafe {
            for x in 2..33 {   
                set_person(t_list[index], t_list[x]);
                set_name(t_list[x], name.into(), None);
                set_help(t_list[x], help.into(), None);
            }
            // Veyle 39
            set_person(t_list[index], t_list[39]);
            set_name(t_list[39], name.into(), None);
            set_help(t_list[39], help.into(), None);
            // Mauvier 49
            set_person(t_list[index], t_list[49]);
            set_name(t_list[49], name.into(), None);
            set_help(t_list[49], help.into(), None);

            // Anna 51 
            set_person(t_list[index], t_list[51]);
            set_name(t_list[51], name.into(), None);
            set_help(t_list[51], help.into(), None);

            // Jean 52
            set_person(t_list[index], t_list[52]);
            set_name(t_list[52], name.into(), None);
            set_help(t_list[52], help.into(), None);
            println!("Characters are set to {}", name);
        }
    }
    
}
pub struct CharacterMod;
impl ConfigBasicMenuItemSwitchMethods for CharacterMod {
    fn init_content(this: & mut ConfigBasicMenuItem){
        let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
        let t_list = &triabolical.list.items;
        let seted = unsafe { setPerson };
        if seted == false {
            unsafe {
                for x in 2..33 {   
                    set_person(t_list[x], t_list[902+x]);
                }
                // Veyle 39
                set_person(t_list[39], t_list[935]);
                // Mauvier 49
                set_person(t_list[49], t_list[936]);
                // Anna 51 
                set_person(t_list[51], t_list[937]);
                // Jean 52
                set_person(t_list[52], t_list[938]);
            }
        }
        unsafe { setPerson = true; }
        changeCharacters();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(CHARACTER_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 35, 1);

        if toggle != result {
            GameVariableManager::set_number(CHARACTER_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            changeCharacters();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(CHARACTER_KEY);
        if typeC == 0 {
            this.help_text = format!("Characters are set their default appearance/growths.").into(); 
        }
        else {
            this.help_text = format!("Characters are set to the selected character.").into(); 
        }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC: usize = (GameVariableManager::get_number(CHARACTER_KEY)).try_into().unwrap();
        this.command_text = OptionArray[typeC].into();
    }
}
pub struct GrowthMod;
impl ConfigBasicMenuItemSwitchMethods for  GrowthMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        patchGrowth();
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(GROWTH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);

        if toggle != result {
            GameVariableManager::set_number(GROWTH_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchGrowth();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(GROWTH_KEY);
        let growthMode = GameUserData::get_grow_mode();
        if typeC == 0 {
            if (growthMode == 1) { this.help_text = format!("Default growth mode. (Default: Fixed)").into(); }
            else { this.help_text = format!("Default growth mode: (Default: Random)").into(); }
        }
        else if typeC == 1 {
            if (growthMode == 1) { this.help_text = format!("Switch growth mode. (Fixed to Random)").into(); }
            else { this.help_text = format!("Switch growth mode. (Random to Fixed)").into(); }
        }
        else if typeC == 2 { this.help_text = format!("Units will not gain stats on level ups.").into(); }
        else if typeC == 3 { this.help_text = format!("Units will gain perfect level ups.").into();  }
        else {this.help_text = format!("Unknown Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(GROWTH_KEY);
        let growthMode = GameUserData::get_grow_mode();
        if type_C == 0 {
            if (growthMode == 1) { this.command_text = format!("Default: Fixed").into(); }
            else { this.command_text =format!("Default: Random").into(); }
        }
        else if type_C == 1 { 
            if (growthMode == 0) { this.command_text = format!("Switch: Fixed").into(); }
            else { this.command_text =format!("Switch: Random").into(); } 
        }
        else if type_C == 2 { this.command_text = format!("No Growths").into(); }
        else if type_C == 3 { this.command_text = format!("Perfect Growths").into();  }
        else {this.help_text = format!("Unknown").into(); }
    }
}
pub struct LevelMod;
impl ConfigBasicMenuItemSwitchMethods for LevelMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ 

    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(LEVEL_DIS_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchLvl();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        if (toggle) { this.help_text = format!("Displays unit's total level. (Internal + Displayed Level)").into(); } 
        else { this.help_text = format!("Default level display. (Displayed Level)").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(LEVEL_DIS_KEY);
        if (toggle){ this.command_text = format!("Total Level").into();} 
        else { this.command_text = format!("Default").into(); }
    }
}

#[no_mangle]
extern "C" fn level_callback() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<LevelMod>("Unit Level Display")
}
#[no_mangle]
extern "C" fn growth_callback() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<GrowthMod>("Growth Mode")
}
#[no_mangle]
extern "C" fn char_callback() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<CharacterMod>("Single Character Mode")
}
pub fn level_install(){
    cobapi::install_game_setting(growth_callback);
    cobapi::install_game_setting(level_callback);
    cobapi::install_game_setting(char_callback);
}