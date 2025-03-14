use super::*;
use engage::{
    menu::{BasicMenuItem, BasicMenuItemAttribute}, mess::Mess, proc::{desc::ProcDesc, inst::ProcInst, ProcVoidMethod}, 
    sequence::mapsequence::human::MapSequenceHumanLabel,
};
use utils::get_nested_virtual_methods_mut;


pub fn map_save_proc_edit(map_sequence_human: &ProcInst) {
    let descs = map_sequence_human.descs.get();
    unsafe { 
    // Force MapSequenceHuman to Jump to Label 47 at Label 48 MapSequenceHumanLabel::SuspendMenu
        (*descs)[0xd0] = ProcDesc::jump(MapSequenceHumanLabel::SaveMenu as i32); 
    // Replace MapSequenceHuman$$SaveAndSuspendMenuBefore to remove Temporary Status
        (*descs)[0xcb] = ProcDesc::call(ProcVoidMethod::new(None, remove_temporary_game_status));  
    }
}

extern "C" fn remove_temporary_game_status(_proc: &mut ProcInst, _method_info: OptionalMethod) {
    let status = GameUserData::get_status();
    status.value &= !0x200;
}

pub fn map_save_menu_edits() {
    get_nested_virtual_methods_mut("App", "MapSystemMenu", "TemporarySaveItem", "GetName")
        .map(|m| m.method_ptr = map_system_temp_save_menu_name as _).unwrap();
    get_nested_virtual_methods_mut("App", "MapSystemMenu", "TemporarySaveItem", "GetHelpText")
        .map(|m| m.method_ptr = map_system_temp_save_get_help_text as _).unwrap();
    get_nested_virtual_methods_mut("App", "MapSystemMenu", "TemporarySaveItem", "GetMapAttribute")
        .map(|m| m.method_ptr = map_system_temp_save_build_attr as _).unwrap();
}

fn map_system_temp_save_menu_name(_temp_save_menu_item: &BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_SORTIE_SAVE") }

fn map_system_temp_save_get_help_text(_temp_save_menu_item: &BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_SORTIE_SAVE_HELP") }

fn map_system_temp_save_build_attr(_temp_save_menu_item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if GameVariableManager::get_bool("G_Ironman") { BasicMenuItemAttribute::Hide }
    else { BasicMenuItemAttribute::Enable }
}