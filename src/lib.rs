#![feature(ptr_sub_ptr)]
use unity::prelude::*;
use skyline::patching::Patch;
use engage::{
    gamevariable::GameVariableManager,
    gameuserdata::GameUserData,
};
pub mod help;
pub mod configmenu;
// pub mod patches;
pub mod utils;
pub mod hub;
pub mod misc;
pub mod events;
pub mod sortie;
pub mod mapsave;
pub mod gmapshop;

#[skyline::main(name = "libtriabolical")]
pub fn main() {
    misc::misc_code_patches();
    cobapi::install_lua_command_registerer(configmenu::patch_code_mods);
    configmenu::initialize_and_install();
    configmenu::install_config_hooks();
    help::install_help_hooks();
    skyline::install_hook!(hub::quickmenu::hub_menu_create_bind);
    cobapi::register_system_event_handler(events::create_settings_install_menu_hooks);
    Patch::in_text(0x01b3d254).nop().unwrap(); //gmap shop bind nop'd
    
    println!("MyCode Mods are loaded");

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };
        let err_msg = format!(
            "My Code Mods plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );
        skyline::error::show_error(
            0,
            "My Code Mods has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
}