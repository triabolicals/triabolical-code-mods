use super::*;
use engage::{random::Random, force::*, gamedata::unit::Unit, gameuserdata::GameUserData, menu::config::ConfigBasicMenuItemCommandMethods};

pub struct GrowthMod;
impl ConfigBasicMenuItemSwitchMethods for  GrowthMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry(GROWTH_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(GROWTH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(GROWTH_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_growth();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let growth_mode = GameUserData::get_grow_mode();
        this.help_text = match GameVariableManager::get_number(GROWTH_KEY) {
            0 => {
                let current =  if growth_mode == 1 {  Mess::get("MID_SYS_Grow_Fixed") } else {Mess::get("MID_SYS_Grow_Random") };
                format!("{} (Save File Selected: {})", Mess::get("MID_GAMESTART_GROWMODE_SELECT_HELP"), current)
            }
            1 => {
                let switch =  if growth_mode == 0 {  Mess::get("MID_SYS_Grow_Fixed") } else {Mess::get("MID_SYS_Grow_Random") };
                format!("{} (Switch to {})", Mess::get("MID_GAMESTART_GROWMODE_SELECT_HELP"), switch)
            }
            2 => { "No stats increases on level ups.".to_string() }
            3 => { "All possible stats are increase on level ups.".to_string() }
            _ => { Mess::get("MID_GAMESTART_GROWMODE_SELECT_HELP").to_string() }
        }.into();
    }
        
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = 
        match GameVariableManager::get_number(GROWTH_KEY) {
            0 => { "Selected".into() }
            1 => {  "Switch".into() }
            2 => { format!("{} {}", Mess::get("MID_MENU_NO"), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE")).into() }
            3 => {  Mess::get("MID_Hub_MuscleExercises_Perfect")  }
            _ => { "???".into() }
        };
    }
}

pub extern "C" fn growth() -> &'static mut ConfigBasicMenuItem { 
    let str1 = Mess::get("MID_GAMESTART_GROWMODE_TITLE");
    println!("Growth installed");
    ConfigBasicMenuItem::new_switch::<GrowthMod>(str1.to_string())

}

pub struct ReseedGrow;
impl ConfigBasicMenuItemCommandMethods for ReseedGrow {
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let pad_instance = engage::util::get_instance::<engage::pad::Pad>();
        if pad_instance.npad_state.buttons.a() {
            YesNoDialog::bind::<ReseedGrowConfirm>(this.menu, "Reseed Player Growth Seeds?", "Do it!", "Nah..");
            BasicMenuResult::se_cursor()
        }   
        else { BasicMenuResult::new() }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) { this.command_text = "Reseed".into(); }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) { this.help_text = "Reseeds player unit's growth seeds for Random Growths.".into(); }
}
pub struct ReseedGrowConfirm;
impl TwoChoiceDialogMethods for ReseedGrowConfirm {
    extern "C" fn on_first_choice(_this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        change_player_grow_seed();
        BasicMenuResult::se_cursor().with_close_this(true)
    }
    extern "C" fn on_second_choice(_this: &mut BasicDialogItemNo, _method_info: OptionalMethod) -> BasicMenuResult { BasicMenuResult::new().with_close_this(true) }
}

pub extern "C" fn reseed() -> &'static mut ConfigBasicMenuItem { 
    println!("Reseed installed");
    ConfigBasicMenuItem::new_command::<ReseedGrow>("Change Level up Seed")
}
pub fn growth_install() {
    cobapi::install_game_setting(growth);
    cobapi::install_game_setting(reseed);
}

fn change_player_grow_seed() {
    let rng = Random::get_game();
    Force::get(ForceType::Player).unwrap().iter()
        .chain(  Force::get(ForceType::Absent).unwrap().iter() )
        .for_each(|unit|{
            unsafe { set_grow_seed(unit, rng.value(), None); } 
        }
    );
}

fn restore_default(){
    //Growth Mode Call
    Patch::in_text(0x01a3a3c4).bytes(&[0xe7, 0x6a, 0x2b, 0x94]).unwrap();
    //Random
    Patch::in_text(0x01a3a658).bytes(&[0x14,0x81,0x40, 0x39]).unwrap();
    //Random RNG 
    Patch::in_text(0x01a3a73c).bytes(&[0x5d, 0xeb, 0x24, 0x94]).unwrap();
    //Fixed
    Patch::in_text(0x01a3a410).bytes(&[0x14,0x81, 0x40, 0x39]).unwrap();
    // Level Down but add the level instead of subtracting it
    Patch::in_text(0x01a3ac8c).bytes(&[0x08, 0x05, 0x0, 0x51]).unwrap();
}

pub fn patch_growth(){
    GameVariableManager::make_entry(GROWTH_KEY, 0);
    let result = GameVariableManager::get_number(GROWTH_KEY);
    restore_default();
    if result == 0{ 
        println!("Growth set to save file default");
        restore_default(); 
    }
    else if result == 1{
        //Opposite Mode
        let growth_mode = GameUserData::get_grow_mode();
        if growth_mode == 0 {//Random -> Fixed
            Patch::in_text(0x01a3a3c4).bytes(&[0x20, 0x00, 0x80, 0xd2]).unwrap();
            println!("Growth set to 'Fixed' from save file default of 'Random'");
        }
        else { //Fixed -> Random
            Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
            println!("Growth set to 'Random' from save file default of 'Fixed'");
        }
    }
    else if result == 2 {
        // No Growths
        Patch::in_text(0x01a3a410).bytes(&[0x14,0x00,0x80,0xD2]).unwrap();
        Patch::in_text(0x01a3a658).bytes(&[0x14,0x00, 0x80,0xD2]).unwrap();
        println!("Growth set to 'No Growths'");
    }
    else if result == 3 {
        // Perfect Level Ups, forcing to Random and RNG set to 1
        Patch::in_text(0x01a3a3c4).bytes(&[0x00, 0x00, 0x80, 0xd2]).unwrap();
        Patch::in_text(0x01a3a73c).bytes(&[0x20, 0x00, 0x80, 0x52]).unwrap();
        println!("Growth set to 'Perfect'");
    }
}

#[unity::from_offset("App", "Unit", "set_GrowSeed")]
fn set_grow_seed(this: &Unit, value: i32, _method_info: OptionalMethod);