use unity::prelude::*;
use engage::gamedata::{Gamedata, music::*};

pub fn add_to_juke_box(){
    let music_list = MusicData::get_list().unwrap();
    if JukeBoxData::get_count() > 10 { return; }
    let jukebox_list = JukeBoxData::get_list_mut().unwrap();
    for x in 0..music_list.len() {
        let new_juke_box = JukeBoxData::instantiate().unwrap();
        unsafe {
            juke_box_data_ctor(new_juke_box, None);
            juke_box_data_set_event(new_juke_box, music_list[x as usize].event_name, None);
            juke_box_data_set_name(new_juke_box, music_list[x as usize].name, None);
        }
        jukebox_list.add(new_juke_box);
    }
}

#[unity::from_offset("App", "JukeboxData", ".ctor")]
pub fn juke_box_data_ctor(this: &JukeBoxData, method_info: OptionalMethod);

#[unity::from_offset("App", "JukeboxData", "set_Condition")]
pub fn juke_box_data_set_condition(this: &JukeBoxData, value: &Il2CppString,method_info: OptionalMethod);

#[unity::from_offset("App", "JukeboxData", "set_EventName")]
pub fn juke_box_data_set_event(this: &JukeBoxData, value: &Il2CppString,method_info: OptionalMethod);

#[unity::from_offset("App", "JukeboxData", "set_Name")]
pub fn juke_box_data_set_name(this: &JukeBoxData, value: &Il2CppString, method_info: OptionalMethod);