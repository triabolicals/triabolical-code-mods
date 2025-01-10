use unity::prelude::*;
use skyline::patching::Patch;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem, ConfigBasicMenuItemCommandMethods}},
    pad::*,
    util::get_instance,
    force::*,
    dialog::yesno::*,
    gamedata::unit::*,
    gameuserdata::GameUserData,
    gamevariable::*,
    random::*,
    mess::*,
};
use crate::string::*;
// Level, Growth Mods
pub const LEVEL_DIS_KEY: &str = "G_LEVEL_TYPE";
pub const GROWTH_KEY: &str = "G_GROWTH_TYPE";

//Structure and functions to hook to Level display settings
#[unity::class("TMPro", "TMP_Text")]
pub struct TMP_Text {}

#[unity::class("TMPro", "TextMeshProUGUI")]
pub struct TextMeshProUGUI {
    pub parent: TMP_TextFields,
}

#[unity::class("App", "UnitStatusSetter")]
pub struct UnitStatusSetter {
    junk: [u8; 376],
    pub level: &'static UnitStatusSetterValueParam,
}

#[unity::class("App", "UnitStatusSetterValueParam")]
pub struct UnitStatusSetterValueParam {
    setter: &'static UnitStatusSetter,
    m_root_ptr: u64,
    pub title: &'static TextMeshProUGUI,
    value: &'static TextMeshProUGUI,
    //
}
#[unity::class("App", "UnitInfoParamSetter")]
pub struct UnitInfoParamSetter {
    parent: u64,
    engine: u64,
    pub simple_ui: i32,
    junk : [u8; 136],
    pub level : &'static TextMeshProUGUI,
}
#[unity::class("App","LevelUpWindowController")]
pub struct LevelUpWindowController {
    junk: u64,
    pub char_name: &'static TextMeshProUGUI,
    pub title_level: &'static TextMeshProUGUI,
    pub level: &'static TextMeshProUGUI,
    pub job: &'static TextMeshProUGUI,
}
#[unity::hook("App","LevelUpWindowController", "SetupParams")]
pub fn level_up_window_setup_hook(this: &LevelUpWindowController, unit: &Unit, next: &Unit, method_info: OptionalMethod){
    call_original!(this, unit, next, method_info);
    if GameVariableManager::get_number(LEVEL_DIS_KEY) != 0 {
        if unit.internal_level!= 0 {
            let level_str: &Il2CppString = format!("{}/{}", unit.level, unit.internal_level).into();
            unsafe { try_set_text_string(this.level, level_str, None); }
        }
    }
}
#[skyline::hook(offset = 0x1f9d320)]
pub fn unit_info_set_level_hook(this: &mut UnitInfoParamSetter, unit: Option<&Unit>, x: i32, z: i32, selected_god: bool, god: &GodUnit, method_info: OptionalMethod){
    call_original!(this, unit, x, z, selected_god, god, method_info);
    if let Some(p) = unit { 
        GameVariableManager::make_entry_norewind(LEVEL_DIS_KEY, 0);
        let result = GameVariableManager::get_number(LEVEL_DIS_KEY);
        let enhance_level= p.get_enchanced_level();
        let mut displayed_level = enhance_level;
        if result != 0 { 
            if p.internal_level >= 0 { displayed_level = enhance_level+ (p.internal_level as i32); }
        }
        unsafe { try_set_text(this.level, displayed_level, None) };
    }
}

#[skyline::from_offset(0x290f1c0)]
pub fn try_set_text(tmp: &TextMeshProUGUI, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x0290f0a0)]
pub fn try_set_text_string(tmp: &TextMeshProUGUI, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1b58360)]
pub fn set_value_direct(this: &UnitStatusSetterValueParam, str: &Il2CppString, dir: i32, is_limit: bool, method_info: OptionalMethod);

//Hooking to where the game sets the level display in the unit status screen
#[skyline::hook(offset = 0x1c66980)]
pub fn set_total_level(this: &UnitStatusSetter, unit: &Unit, unit_no_enhance: &Unit, method_info: OptionalMethod){
    call_original!(this, unit, unit_no_enhance, method_info);
    let result = GameVariableManager::get_number(LEVEL_DIS_KEY);
    unsafe {
        let enhance_level= unit.get_enchanced_level();
        let not_enhance_level = unit_no_enhance.get_enchanced_level();
        let unit_level = unit_no_enhance.level;
        let max_level = unit_no_enhance.job.get_max_level();
        let boost: i32 = (not_enhance_level < enhance_level) as i32;
        let at_limit: bool = max_level <= unit_level;
        let displayed_level = enhance_level;
        if result > 0 {
            let internal_level = unit_no_enhance.internal_level;
            if internal_level <= 0 {
                let level_str = format!("{}", displayed_level).into();
                set_value_direct(this.level, level_str , boost, at_limit, None);
            }
            else {
                let level_str = format!("{}/{}", displayed_level, internal_level).into();
                set_value_direct(this.level, level_str , boost, at_limit, None);
            }
        }
        else {
            let level_str = format!("{}", displayed_level).into();
            set_value_direct(this.level, level_str , boost, at_limit, None);
        }
    }
}

// Growth mode default
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
        let mode =  GameVariableManager::get_number(GROWTH_KEY);
        let growth_mode = GameUserData::get_grow_mode();
        if mode == 0 { this.command_text = "Selected".into(); }
        else if mode == 1 { this.command_text = "Switch".into(); }
        else if mode == 2 { unsafe { this.command_text = concat_strings3( Mess::get("MID_MENU_NO"), " ".into(), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"), None); } }
        else if mode == 3 { this.command_text =  Mess::get("MID_Hub_MuscleExercises_Perfect");  }
        else if mode == 4 { this.command_text = "???".into();  }
    }
}
pub struct LevelMod;
impl ConfigBasicMenuItemSwitchMethods for LevelMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry(LEVEL_DIS_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        GameVariableManager::make_entry(LEVEL_DIS_KEY, 0);
        let toggle = GameVariableManager::get_number(LEVEL_DIS_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 2, 1);
        if toggle != result {
            GameVariableManager::set_number(LEVEL_DIS_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = match GameVariableManager::get_number(LEVEL_DIS_KEY) {
            1 => { "Displays unit's total level. (Displayed / Internal)" }
            2 => { "Displays total level and stats for next level up." }
            _ => { "Default level display. (Displayed)" }
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(LEVEL_DIS_KEY) {
            1 => { "Total" }
            2 => { "Total + Next" }
            _ => { "Default" }
        }.into();
    }
}

pub struct ReseedGrow;
impl ConfigBasicMenuItemCommandMethods for ReseedGrow {
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let pad_instance = get_instance::<Pad>();
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
    extern "C" fn on_first_choice(this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        change_player_grow_seed();
        BasicMenuResult::se_cursor().with_close_this(true)
    }
    extern "C" fn on_second_choice(_this: &mut BasicDialogItemNo, _method_info: OptionalMethod) -> BasicMenuResult { BasicMenuResult::new().with_close_this(true) }
}

fn change_player_grow_seed() {
    let force_type = [ForceType::Player, ForceType::Absent, ForceType::Dead];
    let rng = Random::get_game();
    for ff in force_type {
        let force_iter = Force::iter(Force::get(ff).unwrap());
        for unit in force_iter {
            unsafe { set_grow_seed(unit, rng.value(), None); }
        }
    }
}
#[unity::from_offset("App", "Unit", "set_GrowSeed")]
fn set_grow_seed(this: &Unit, value: i32, _method_info: OptionalMethod);

#[no_mangle]
extern "C" fn level_callback() -> &'static mut ConfigBasicMenuItem {  ConfigBasicMenuItem::new_switch::<LevelMod>("Display Total Level")}

#[no_mangle]
extern "C" fn growth_callback() -> &'static mut ConfigBasicMenuItem { 
    let str1 = Mess::get("MID_GAMESTART_GROWMODE_TITLE");
    ConfigBasicMenuItem::new_switch::<GrowthMod>(str1.get_string().unwrap())
}
#[no_mangle]
extern "C" fn reseed_callback() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_command::<ReseedGrow>("Change Level up Seed")
}
#[no_mangle]
pub fn level_install(){
    cobapi::install_game_setting(growth_callback);
    cobapi::install_game_setting(level_callback);
    cobapi::install_game_setting(reseed_callback);
}