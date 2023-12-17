use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
<<<<<<< HEAD
use crate::il2cpp::object::Array;
use engage::gameuserdata::*;
use engage::{gamedata::*, singleton::SingletonClass};
=======
use engage::gameuserdata::*;
>>>>>>> e2b5f3fd6e61426eeb2782af6f11aed4cdd66f91
#[unity::from_offset("App", "GameVariable", "GetNumber")]
pub fn get_number1(this: &GameVariable, key: &Il2CppString, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "GetGrowMode")]
pub fn get_growMode(this: &GameUserData, method_info: OptionalMethod) -> i32;

<<<<<<< HEAD
#[unity::from_offset("App", "GameUserData", "GetDifficulty")]
pub fn get_difficulty(this: &GameUserData, is_dynamic: bool, method_info: OptionalMethod) -> i32;

=======
>>>>>>> e2b5f3fd6e61426eeb2782af6f11aed4cdd66f91
pub fn getNumber(key: &str) -> i32 {
    let game_variable = GameUserData::get_variable();
    unsafe { get_number1(game_variable, key.into(), None) }
}

pub fn setNumber(key: &str, value: i32){
    let game_variable = GameUserData::get_variable();
    unsafe { set_number(game_variable, key.into(), value, None); }
}
pub fn getGrowMode() -> i32 {
    let game_user_date = GameUserData::get_instance();
    unsafe { get_growMode(game_user_date, None)}
<<<<<<< HEAD
}
pub fn Get_Difficulty() -> i32 {
    let game_user_date = GameUserData::get_instance();
    unsafe { get_difficulty(game_user_date, false, None)}
}
//Methods from App.PersonData$$


=======
}
>>>>>>> e2b5f3fd6e61426eeb2782af6f11aed4cdd66f91
