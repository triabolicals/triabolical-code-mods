use super::*;

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

pub fn dlc_map_install(){ cobapi::install_game_setting(dlc_complete); }

pub struct DLCConfirm;
impl TwoChoiceDialogMethods for DLCConfirm {
    extern "C" fn on_first_choice(_this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        complete_dlc_chapters();
        BasicMenuResult::se_cursor().with_close_this(true)
    }
    extern "C" fn on_second_choice(_this: &mut BasicDialogItemNo, _method_info: OptionalMethod) -> BasicMenuResult { BasicMenuResult::new().with_close_this(true) }
} 


extern "C" fn dlc_complete() -> &'static mut ConfigBasicMenuItem { 
    let menu_item = ConfigBasicMenuItem::new_switch::<CompleteDLCMod>("DLC Setting"); 
    menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = dlc_setting_acall as _ );
    menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = dlc_build_attribute as _);
    menu_item
 }


 fn dlc_build_attribute(_this: &ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute  {
    if GameVariableManager::get_number("G_Continuous") > 0 { return BasicMenuItemAttribute::Hide; }
    if crate::utils::has_dlc() { return BasicMenuItemAttribute::Hide; }
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
                    engage::godpool::GodPool::create_by_gid(gid);
                }
            }
            else {
                for x in emblems {
                    let key = format!("G_R_GID_{}", x);
                    let gid =
                    if GameVariableManager::exist(&key) { GameVariableManager::get_string(&key) }
                        else { format!("GID_{}", x).into() };
                    engage::godpool::GodPool::create_by_gid(gid);
                }
            }
            let complete = complete | 1;
            GameVariableManager::set_number("G_DLC_Complete2", complete);
        }
    }
}
