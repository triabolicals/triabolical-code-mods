#![feature(lazy_cell, ptr_sub_ptr)]
use skyline::patching::Patch;
use unity::prelude::*;
mod game_var_i;
mod map;
mod support;
mod arena;
mod cutscene;
mod rng;
mod cook;
mod level;

#[skyline::main(name = "libtriabolical")]
pub fn main() {

    //Enables support/bond viewing in maps and exploration
    let replace = &[0x1f, 0x25, 0x00, 0x71];
    Patch::in_text(0x0209950C).bytes(replace).unwrap();
    Patch::in_text(0x020994E0).bytes(replace).unwrap();
    Patch::in_text(0x02099538).bytes(replace).unwrap();

    cutscene::cutscene_install();
    support::support_install();
    map::map_mod_install();
    arena::arena_install();
    level::level_install();
    rng::rng_install();
    cook::cook_install();
    
    println!("triabolical code mods are loaded");
    

}
