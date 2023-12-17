use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
pub const CUTSCENES_KEY: &str = "G_CUTSCENE";
<<<<<<< HEAD
pub const BGM_KEY: &str = "G_BGM";
pub const GIFT_KEY: &str = "G_GIFT";
=======
>>>>>>> e2b5f3fd6e61426eeb2782af6f11aed4cdd66f91

pub struct CutsceneMod;
pub fn patchCutscenes(){
    GameVariableManager::make_entry_norewind(CUTSCENES_KEY, 0);
    let active = GameVariableManager::get_bool(CUTSCENES_KEY);
    if (active){
        let replace = &[0xC0, 0x03, 0x5F, 0xD6];
        Patch::in_text(0x01ed8e20).bytes(replace).unwrap();
        Patch::in_text(0x01ed8ef0).bytes(replace).unwrap();
<<<<<<< HEAD
=======
        Patch::in_text(0x01eda300).bytes(replace).unwrap();
        Patch::in_text(0x01eda3c0).bytes(replace).unwrap();
>>>>>>> e2b5f3fd6e61426eeb2782af6f11aed4cdd66f91
        println!("Cutscenes/Movies are skipped");
    }
    else {
        Patch::in_text(0x01ed8e20).bytes(&[0xFD, 0x7B, 0xBE, 0xA9]).unwrap();
        Patch::in_text(0x01ed8ef0).bytes(&[0xFD, 0x7B, 0xBD, 0xA9]).unwrap();
<<<<<<< HEAD
        println!("Cutscenes/Movies are not skipped");
    }
}
impl ConfigBasicMenuItemSwitchMethods for CutsceneMod {
    fn init_content(this: &mut ConfigBasicMenuItem){  patchCutscenes(); }
=======
        Patch::in_text(0x01eda300).bytes(&[0xe8, 0x0f, 0x1b,0xfc]).unwrap();
        Patch::in_text(0x01eda3c0).bytes(&[0xec, 0x0f, 0x1b,0xfc]).unwrap();
        println!("Cutscenes/Movies are not skipped");

    }
}


impl ConfigBasicMenuItemSwitchMethods for CutsceneMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ 
        patchCutscenes();
    }
>>>>>>> e2b5f3fd6e61426eeb2782af6f11aed4cdd66f91
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(CUTSCENES_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(CUTSCENES_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patchCutscenes();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(CUTSCENES_KEY);
        if (toggle) { this.help_text = format!("Disables cutscenes and movies during chapter maps.").into(); } 
        else { this.help_text = format!("Enables cutscenes and movies during chapter maps.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(CUTSCENES_KEY);
        if (toggle) { this.command_text = format!("On").into();} 
        else { this.command_text = format!("Off").into(); }
    }
}
<<<<<<< HEAD
pub struct BGMmod;
impl ConfigBasicMenuItemSwitchMethods for BGMmod {
    fn init_content(this: &mut ConfigBasicMenuItem){ 
        GameVariableManager::make_entry_norewind(BGM_KEY, 0);

     }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_bool(BGM_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(BGM_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(BGM_KEY);
        if (toggle) { this.help_text = format!("Player phase BGM overrides other phase BGM.").into(); } 
        else { this.help_text = format!("Default BGM Setting").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_bool(BGM_KEY);
        if (toggle) { this.command_text = format!("Skip Enemy/Ally Phase").into();} 
        else { this.command_text = format!("Default").into(); }
    }
}

//Force BGM to player phase
#[skyline::hook(offset=0x02d56700)]
pub fn ChangeBGM(this: u64, forceType: i32, proc: u64, isTurn: bool, method_info: OptionalMethod) {
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { call_original!(this, 0, proc, isTurn, method_info) }
    else { call_original!(this, forceType, proc, isTurn,method_info) }
}
//For Time Crystals Phase Change
#[skyline::hook(offset=0x02d56930)]
pub fn ChangeBGM2(this: u64, forceType: i32, method_info: OptionalMethod) {
    println!("Change force IMMM called with force {}", forceType);
    let toggle = GameVariableManager::get_bool(BGM_KEY);
    if toggle { call_original!(this, 0,  method_info) }
    else { call_original!(this, forceType, method_info) }
}

pub struct Giftmod;
impl ConfigBasicMenuItemSwitchMethods for Giftmod {
    fn init_content(this: &mut ConfigBasicMenuItem){  GameVariableManager::make_entry_norewind(GIFT_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number(GIFT_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(GIFT_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_number(GIFT_KEY);
        if toggle == 0 { this.help_text = format!("Accept both patch and DLC gift items.").into(); } 
        else if toggle == 1 { this.help_text = format!("Accept only patch items.").into(); } 
        else if toggle == 2 { this.help_text = format!("Accept only paid DLC items.").into(); }
        else if toggle == 3 { this.help_text = format!("Accept no gifts.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let toggle = GameVariableManager::get_number(GIFT_KEY);
        if toggle == 0 { this.command_text = format!("All Gifts").into();} 
        else if toggle == 1 { this.command_text = format!("Patch Update Gifts").into(); }
        else if toggle == 2 { this.command_text = format!("Expansion Pass Gifts").into(); }
        else if toggle == 3 { this.command_text = format!("No Gifts").into(); }
    }
}

#[skyline::hook(offset= 0x023f3c00)]
pub fn TryGiftEvent(this: u64, flagName: &Il2CppString, rewardID: &Il2CppString, messageID: &Il2CppString, method_info: OptionalMethod){
    let toggle = GameVariableManager::get_number(GIFT_KEY);
    if toggle == 0{ call_original!(this, flagName, rewardID, messageID, method_info); }
    else if toggle == 1 {
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_DLC_Accessory1" {return; }
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_DLC_Accessory2" {return; }
        call_original!(this, flagName, rewardID, messageID, method_info);
    }
    else if toggle == 2 {
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_Patch0" {return; }
        if messageID.get_string().unwrap() == "MID_MSG_GET_ITEM_Patch3" {return; }
        call_original!(this, flagName, rewardID, messageID, method_info);
    }
    else { return;  }


}


#[no_mangle]
extern "C" fn cutscene() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CutsceneMod>("Skip Cutscenes/Movies") }
extern "C" fn bgm() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<BGMmod>("Battle BGM Settings") }
extern "C" fn gift() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<Giftmod>("Gift Options") }
pub fn cutscene_install(){
   cobapi::install_game_setting(cutscene);
   cobapi::install_game_setting(bgm);
   cobapi::install_game_setting(gift);
=======
#[no_mangle]
extern "C" fn cutscene() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<CutsceneMod>("Skip Cutscenes/Movies") }

pub fn cutscene_install(){
   cobapi::install_game_setting(cutscene);
>>>>>>> e2b5f3fd6e61426eeb2782af6f11aed4cdd66f91
}