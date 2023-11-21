#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use engage::gamevariable::*;
use engage::gameuserdata::*;
#[unity::from_offset("App", "GameVariable", "GetNumber")]
pub fn get_number1(this: &GameVariable, key: &Il2CppString, method_info: OptionalMethod) -> i32;


pub fn getNumber(key: &str) -> i32 {
    let game_variable = GameUserData::get_variable();

    unsafe { get_number1(game_variable, key.into(), None) }

}

pub fn setNumber(key: &str, value: i32){
    let game_variable = GameUserData::get_variable();

    unsafe { set_number(game_variable, key.into(), value, None); }

}