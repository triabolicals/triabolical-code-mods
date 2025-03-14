use engage::{gameuserglobaldata::GameUserGlobalData, gamedata::WeaponMask};
use unity::prelude::OptionalMethod;
use skyline::patching::Patch;


pub fn set_global_completed() {
    let global = GameUserGlobalData::get_instance();
    let mas = unsafe {gameuserglobaldata_get_flag(global, None) };
    mas.value |= 63;
}


pub fn misc_code_patches() {
    support_view_patch();
    player_name_input_size_patch();
    gmap_info_content_level_patch();
    arena_unit_selection_patch();
}

// Remove Sequence Checks to allow supports to be view in Map/Sortie/Kizuna
fn support_view_patch()  {    
    let replace = &[0x1f, 0x25, 0x00, 0x71];
    // Removing the Sequence Checks by comparing to 9
    Patch::in_text(0x0209950C).bytes(replace).unwrap(); // Map
    Patch::in_text(0x020994E0).bytes(replace).unwrap(); // Kizuna
    Patch::in_text(0x02099538).bytes(replace).unwrap(); // Sortie
}

/// Increase the limit from 8 to 32
fn player_name_input_size_patch() { Patch::in_text(0x01fdea34).bytes(&[0x01,0x04,0x80, 0x52]).unwrap(); }

/// Changes the max displayed level for maps to be 99 instead of 20
fn gmap_info_content_level_patch() { Patch::in_text(0x0252d124).bytes(&[0x60, 0x0c, 0x80, 0x52]).unwrap(); }

/// Expands unit selection to look in Player Force first when populating the Unit Selection menu in the Arena
fn arena_unit_selection_patch() {
    Patch::in_text(0x01ca4418).bytes(&[0x29, 0x01, 0x80, 0x52]).unwrap();   // ForceMask = 8 -> 9 for ArenaExp (Player + Absent)
    Patch::in_text(0x0233ce08).bytes(&[0x29, 0x01, 0x80, 0x52]).unwrap();   // ForceMask = 8 -> 9 for Emblem Selection  (Player + Absent)
}

#[skyline::from_offset(0x0251c430)]
fn gameuserglobaldata_get_flag(this: &GameUserGlobalData, method_info: OptionalMethod) -> &'static mut WeaponMask;
