use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuItemAttribute, BasicMenuResult};
use engage::gamevariable::*;
use engage::gamedata::*;
use engage::dialog::yesno::*;

pub const GIFT_KEY: &str = "G_GIFT";
pub const DLC_COMPLETE: &str = "G_DLC_Complete";


pub fn patch_gift(){
    let ret = &[0xC0, 0x03, 0x5F, 0xD6];
    match GameVariableManager::get_number(GIFT_KEY) {
        1 => {
            Patch::in_text(0x023f3f18).bytes(ret).unwrap();
            Patch::in_text(0x023f4088).bytes(ret).unwrap();
        }
        2 => {
            Patch::in_text(0x023f3fc8).bytes(ret).unwrap();
            Patch::in_text(0x023f4138).bytes(ret).unwrap();
        }
        3 => {
            Patch::in_text(0x023f3f18).bytes(ret).unwrap();
            Patch::in_text(0x023f4088).bytes(ret).unwrap();
            Patch::in_text(0x023f3fc8).bytes(ret).unwrap();
            Patch::in_text(0x023f4138).bytes(ret).unwrap();
        }
        _ => {
            Patch::in_text(0x023f3f18).bytes(&[0x3a,0xff,0xff,0x17]).unwrap();   //DLC0
            Patch::in_text(0x023f3fc8).bytes(&[0x0e,0xff,0xff,0x17]).unwrap();  //Patch0
            Patch::in_text(0x023f4088).bytes(&[0xde,0xfe,0xff,0x17]).unwrap();   //DLC1
            Patch::in_text(0x023f4138).bytes(&[0xb2,0xfe,0xff, 0x17]).unwrap();  //Patch3
        }
    }
}

pub struct Giftmod;
impl ConfigBasicMenuItemSwitchMethods for Giftmod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_gift(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number(GIFT_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(GIFT_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            patch_gift();
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = match GameVariableManager::get_number(GIFT_KEY){ 
            1 => { "Accept only patch update items." },
            2 => { "Accept only paid DLC items." },
            3 => { "Accept no gift items." },
            _ => { "Accept both patch and DLC gift items." } 
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(GIFT_KEY){ 
            1 => { "Patch Updates." },
            2 => { "Expansion Pass" },
            3 => { "No Freebies" },
            _ => { "All Gifts" },
        }.into();
    }
}

pub struct CompleteDLCMod;
impl ConfigBasicMenuItemSwitchMethods for CompleteDLCMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ complete_dlc_chapters(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number(DLC_COMPLETE);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(DLC_COMPLETE, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = match GameVariableManager::get_number(DLC_COMPLETE) {
            1 => { "Autocomplete all Divine Paralogues." },
            2 => { "Autocomplete Fell Xenologue."},
            3 => { "Autocomplete all DLC chapters."},
            _ => { "DLC chapters will not be autocomplete."},
        }.into(); 
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(DLC_COMPLETE) {
            1 => { "Complete Divine" },
            2 => { "Complete FX"},
            3 => { "Complete All"},
            _ => { "Default"},
        }.into();
    }
}

pub fn complete_dlc_chapters() {
    if GameVariableManager::get_bool("G_拠点_神竜導入イベント再生済み") && GameVariableManager::get_number(DLC_COMPLETE) != 0 {
        let mut complete = GameVariableManager::get_number("G_DLC_Complete2");
        if complete == 3 { return; }
        let mut completed_fx = true;
        for x in 1..7 {
            if !GameVariableManager::get_bool(&format!("G_Cleared_E00{}", x)) {
                completed_fx = false;
            }
        }
        if completed_fx { complete = complete | 2; }

        completed_fx = true;
        for x in 1..7 {
            if !GameVariableManager::get_bool(&format!("G_Cleared_G00{}", x)) {
                completed_fx = false;
            }
        }
        if completed_fx { complete = complete | 1; }
        if complete == 3 { return; }

        if GameVariableManager::get_number(DLC_COMPLETE) & 2 == 2 && complete & 2 == 0 {
            for x in 1..7 {
                GameVariableManager::set_bool(&format!("G_Cleared_E00{}", x), true); 
                GameVariableManager::set_number(&format!("G_GmapSpot_E00{}", x), 3); 
            }
            complete = complete | 2;
            GameVariableManager::set_number("G_DLC_Complete2", complete);
        }
        if GameVariableManager::get_number(DLC_COMPLETE) & 1 == 1 && complete & 1 == 0 {
            for x in 1..7 {
                GameVariableManager::set_bool(&format!("G_Cleared_G00{}", x), true);
                GameVariableManager::set_number(&format!("G_GmapSpot_G00{}", x), 3); 
            }
            let emblems = ["エーデルガルト", "チキ", "ヘクトル", "ヴェロニカ", "セネリオ", "カミラ", "クロム"];
            if GameVariableManager::get_number("G_Emblem_Mode") == 0 {
                for x in emblems {
                    let gid = format!("GID_{}", x);
                    unsafe { godpool_create(GodData::get(&gid).unwrap(), None); }
                }
            }
            else {
                for x in emblems {
                    let key = format!("G_R_GID_{}", x);
                    let gid =
                    if GameVariableManager::exist(&key) { GameVariableManager::get_string(&key) }
                        else { format!("GID_{}", x).into() };
                    
                    unsafe { godpool_create(GodData::get(&gid.get_string().unwrap()).unwrap(), None); }
                }
            }
            let complete = complete | 1;
            GameVariableManager::set_number("G_DLC_Complete2", complete);
        }
    }
}


#[skyline::from_offset(0x023349c0)]
fn godpool_create(this: &GodData, method_info: OptionalMethod) -> u64;

extern "C" fn gift() -> &'static mut ConfigBasicMenuItem { 
    let menu_item = ConfigBasicMenuItem::new_switch::<Giftmod>("Gift Options");
    menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = gift_build_attribute as _);
    menu_item
}

pub fn gift_install(){ cobapi::install_game_setting(gift); }

extern "C" fn dlc_complete() -> &'static mut ConfigBasicMenuItem { 
    let menu_item = ConfigBasicMenuItem::new_switch::<CompleteDLCMod>("DLC Setting"); 
    menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = dlc_setting_acall as _ );
    menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = dlc_build_attribute as _);
    menu_item
 }
 pub fn dlc_map_install(){ cobapi::install_game_setting(dlc_complete); }

 fn dlc_build_attribute(_this: &ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute  {
    if unsafe { !crate::character::has_content(0, None) } { return BasicMenuItemAttribute::Hide; }
    if GameVariableManager::get_number("G_DLC_Complete2") == 3 { return BasicMenuItemAttribute::Hide; }
    if !GameVariableManager::get_bool("G_拠点_神竜導入イベント再生済み") { 
        for x in 1..7 {
            if GameVariableManager::get_bool(&format!("G_Cleared_E00{}", x)) || GameVariableManager::get_bool(&format!("G_Cleared_G00{}", x)) {
                GameVariableManager::make_entry("G_拠点_神竜導入イベント再生済み", 1);
                GameVariableManager::set_bool("G_拠点_神竜導入イベント再生済み", true);
                return BasicMenuItemAttribute::Enable; 
            }
        }
        return BasicMenuItemAttribute::Hide; 
    }
    else { BasicMenuItemAttribute::Enable }
}

fn gift_build_attribute(_this: &ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute  {
    if unsafe { crate::character::has_content(0, None) } {
        if GameVariableManager::get_bool("G_拠点_DLC特典アイテム0受け取り済み") || GameVariableManager::get_bool("G_拠点_DLC特典アイテム1受け取り済み") {
            return BasicMenuItemAttribute::Hide;
        }
        else { return BasicMenuItemAttribute::Enable; }
    }
    if GameVariableManager::get_bool("G_拠点_Patch0特典アイテム受け取り済み") || GameVariableManager::get_bool("G_拠点_Patch3特典アイテム受け取り済み") {
        BasicMenuItemAttribute::Hide
    }
    else { BasicMenuItemAttribute::Enable }
}

pub fn dlc_setting_acall(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if GameVariableManager::get_number(DLC_COMPLETE) == 0  { return BasicMenuResult::se_cursor(); }
    else if GameVariableManager::get_number("G_DLC_Complete2") & GameVariableManager::get_number(DLC_COMPLETE) == 0 {
        let text = match GameVariableManager::get_number(DLC_COMPLETE) {
            1 => { "Autocomplete Divine Paralogues?"},
            2 => { "Autocomplete Fell Xenologue?"},
            3 => { "Autocomplete all DLC Chapters?"},
            _ => { "Do it?"},
        };
        YesNoDialog::bind::<DLCConfirm>(this.menu, text, "Do it!", "Nah..");
        return BasicMenuResult::new();
    }
    return BasicMenuResult::se_cursor(); 
}
pub struct DLCConfirm;
impl TwoChoiceDialogMethods for DLCConfirm {
    extern "C" fn on_first_choice(this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        complete_dlc_chapters();
        BasicMenuResult::se_cursor().with_close_this(true)
    }
    extern "C" fn on_second_choice(_this: &mut BasicDialogItemNo, _method_info: OptionalMethod) -> BasicMenuResult { BasicMenuResult::new().with_close_this(true) }
} 

