use skyline::patching::Patch;
use unity::prelude::*;
use engage::{
    gamedata::unit::*,
    gamevariable::*,
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    mess::*,
};
use crate::string::*;

pub const ARENA_KEY: &str = "G_ARENA_SKIP";

#[skyline::from_offset(0x01ca67f0)]
pub fn arena_order_ctor(this: &ArenaOrderSequence, method_info: OptionalMethod);

#[unity::class("App", "ArenaOrderSequence")]
pub struct ArenaOrderSequence {
    junk: [u8; 0x78],
    pub is_emblem_battle: bool,
    pub is_special_battle: bool, 
    padding: [u8; 2],
    pub training_type: i32,
    pub training_unit: &'static Unit,
    pub battle_unit: &'static Unit,
    pub battle_emblem: &'static GodUnit,
    pub emblem_type: i32,
    pub bond_exp: i32,
    pub calculator: u64, 
    pub sim_calculator: u64,
    objects: [u64; 4],
    pub next_label: i32,
    pub is_back_bond_select_emblem: bool,
    padding2: [u8; 3],
    arena_objects: u64,
    pub god_unit: Option<&'static GodUnit>,
    pub ring: Option<&'static UnitRing>,
}
impl ArenaOrderSequence {
    pub fn new() -> &'static mut Self {
        let item = Self::instantiate().unwrap();
        unsafe { arena_order_ctor(item, None); }
        item
    }
}

#[skyline::from_offset(0x01a4f180)]
pub fn unit_set_god_unit(this: &Unit, god_unit:Option<&GodUnit>, method_info: OptionalMethod);

#[skyline::from_offset(0x01a4e000)]
pub fn unit_set_ring(this: &Unit, ring: Option<&UnitRing>, method_info: OptionalMethod);

#[unity::from_offset("App", "BattleCalculator", "GetDeadSide")]
pub fn battle_calculator_get_dead_side(this: u64, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x027cb090)]
pub fn achieve_add_unit_battle_count(method_info: OptionalMethod);

#[skyline::from_offset(0x027cb0f0)]
pub fn achieve_add_unit_battle_win(method_info: OptionalMethod);

#[skyline::from_offset(0x027cb150)]
pub fn achieve_add_god_battle_count(method_info: OptionalMethod);

#[skyline::from_offset(0x027cb1b0)]
pub fn achieve_add_god_battle_win(method_info: OptionalMethod);

#[unity::hook("App", "ArenaOrderSequence", "FinishTraining")]
pub fn arena_finish_training(this: &ArenaOrderSequence, method_info: OptionalMethod){
    if GameVariableManager::get_bool(ARENA_KEY) {
        unsafe {
            unit_set_god_unit(this.training_unit, this.god_unit, None);
            unit_set_ring(this.training_unit, this.ring, None);
            let dead_side = battle_calculator_get_dead_side(this.calculator, None);
            if this.is_emblem_battle == false {
                achieve_add_unit_battle_count(None);
                if dead_side == 1 { achieve_add_unit_battle_win(None); }
            }
            else {
                achieve_add_god_battle_count(None);
                if dead_side == 1 { achieve_add_god_battle_win(None); }
            }
            if this.training_type == 0 {
                let mut value =  GameVariableManager::get_number("G_拠点_闘技場済み");
                value += 1; 
                GameVariableManager::set_number("G_拠点_闘技場済み", value);
            }
        }
    }
    else { call_original!(this, method_info); }
}

pub struct ArenaMod;
pub fn patch_arena(){
    let active = GameVariableManager::get_bool(ARENA_KEY);
    if active{
        let set_false = &[0x00, 0x00, 0x80, 0x52];
        let set_return = &[0xC0, 0x03, 0x5F, 0xD6];
        let set_nop =  &[0x1F,0x20,0x03,0xD5];

        // App.ArenaOrderSequence$$StartTraining         
        Patch::in_text(0x01caa414).bytes(&[0x20,0x00, 0x80, 0x52]).unwrap();

        // bool Combat.ArenaCombatSequence.<Grow2>d__36$$MoveNext
        Patch::in_text(0x01bac5d0).bytes(set_nop).unwrap();
        Patch::in_text(0x01bac6bc).bytes(set_nop).unwrap();

        // App.ArenaOrderSequence$$BackgroundIn
        Patch::in_text(0x01ca6f40).bytes(set_return).unwrap();
        // App.ArenaOrderSequence$$BackgroundOut
        Patch::in_text(0x01ca6ff0).bytes(set_return).unwrap();
        
        // Combat.ArenaCombatSequence.<StartFight>d__32$$MoveNext
        Patch::in_text(0x01bacb40).bytes(set_false).unwrap();
        Patch::in_text(0x01bacb44).bytes(set_return).unwrap();
        
        // Combat.ArenaCombatSequence.<Setup>d__30$$MoveNext
        Patch::in_text(0x01bac790).bytes(set_false).unwrap();
        Patch::in_text(0x01bac794).bytes(set_return).unwrap();

        // Combat.ArenaCombatSequence.<WaitBegin>d__31$$MoveNext
        Patch::in_text(0x01bacd50).bytes(set_false).unwrap();
        Patch::in_text(0x01bacd54).bytes(set_return).unwrap();

        // Combat.ArenaCombatSequence.<WaitFinish>d__33$$MoveNext
        Patch::in_text(0x01bacf60).bytes(set_false).unwrap();
        Patch::in_text(0x01bacf64).bytes(set_return).unwrap();

        // Combat.ArenaCombatSequence.<Exit>d__38$$MoveNext
        Patch::in_text(0x01bab900).bytes(set_false).unwrap();
        Patch::in_text(0x01bab904).bytes(set_return).unwrap();

        // App.ArenaOrderSequence$$SetupTraining
        Patch::in_text(0x01caa2d8).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa2ac).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa2c0).bytes(set_nop).unwrap();

        // App.ArenaOrderSequence$$FinishTraining
        Patch::in_text(0x01caa5bc).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa5d0).bytes(set_nop).unwrap();
        Patch::in_text(0x01caa5e4).bytes(set_nop).unwrap();

        // App.ArenaOrderSequence$$CreateBind
        Patch::in_text(0x01ca616c).bytes(&[0x60,0x00, 0x80, 0x52]).unwrap();    // App.Fade$$FadeWait(3)
        Patch::in_text(0x01ca6124).bytes(&[0x00,0x10, 0x20, 0x1E]).unwrap();    //App.Fade$$BlackOut(0.0, 4)
        Patch::in_text(0x01ca67b0).nop().unwrap(); 

        //BlackIn/BlackOut duration to 0
        Patch::in_text(0x01ca6484).bytes(&[0xE0,0x03, 0x27, 0x1E]).unwrap();
        Patch::in_text(0x01ca6124).bytes(&[0xE0,0x03, 0x27, 0x1E]).unwrap();

        //Patch::in_text(0x01caa4b0).bytes(set_return).unwrap();
        Patch::in_text(0x01ca67b0).nop().unwrap();

        Patch::in_text(0x01ca9e80).bytes(set_return).unwrap();
        println!("Arena battles are skipped");
    }
    else {
        Patch::in_text(0x01caa414).bytes(&[0xbb, 0x2f, 0x1f, 0x94]).unwrap();
        Patch::in_text(0x01bac5d0).bytes(&[0x54, 0xab, 0x13, 0x94]).unwrap();
        Patch::in_text(0x01ca6ff0).bytes(&[0xfd, 0x7b, 0xbe, 0xa9]).unwrap();
        Patch::in_text(0x01ca6f40).bytes(&[0xfd , 0x7b , 0xbe , 0xa9]).unwrap(); 
        Patch::in_text(0x01bacb40).bytes(&[0xfd , 0x7b , 0xbd , 0xa9]).unwrap();
        Patch::in_text(0x01bacb44).bytes(&[0xf5 , 0x0b , 0x00 , 0xf9]).unwrap();
        Patch::in_text(0x01bac790).bytes(&[0xfd , 0x7b , 0xbc , 0xa9]).unwrap();
        Patch::in_text(0x01bac794).bytes(&[0xf7 , 0x0b , 0x00 , 0xf9]).unwrap();
        Patch::in_text(0x01bacd50).bytes(&[0xfd , 0x7b , 0xbe , 0xa9]).unwrap();
        Patch::in_text(0x01bacd54).bytes(&[0xf4 , 0x4f , 0x01 , 0xa9]).unwrap();
        Patch::in_text(0x01bacf60).bytes(&[0xe8 , 0x0f , 0x1d , 0xfc]).unwrap();
        Patch::in_text(0x01bacf64).bytes(&[0xfd , 0x7b , 0x01 , 0xa9]).unwrap();
        Patch::in_text(0x01bab900).bytes(&[0xfd , 0x7b , 0xbc , 0xa9]).unwrap();
        Patch::in_text(0x01bab904).bytes(&[0xf7 , 0x0b , 0x00 , 0xf9]).unwrap();
        Patch::in_text(0x01bac6bc).bytes(&[0xe1 , 0xd2 , 0x18 , 0x94]).unwrap();
        Patch::in_text(0x01caa2ac).bytes(&[0xf9 , 0x3f , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01caa2c0).bytes(&[0xa8 , 0xe4 , 0x1c , 0x94]).unwrap();
        Patch::in_text(0x01caa2d8).bytes(&[0x06 , 0x0e , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01caa5bc).bytes(&[0x4d , 0x0d , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01caa5d0).bytes(&[0x0c , 0xe4 , 0x1c , 0x94]).unwrap();
        Patch::in_text(0x01caa5e4).bytes(&[0x23 , 0x3f , 0x30 , 0x94]).unwrap();
        Patch::in_text(0x01ca616c).bytes(&[0x80 , 0x00 , 0x80 , 0x52]).unwrap();
        Patch::in_text(0x01ca6124).bytes(&[0x00 , 0x10 , 0x2e , 0x1e]).unwrap();
        Patch::in_text(0x01ca64c8).bytes(&[0xe1 , 0x03 , 0x16 , 0xaa]).unwrap();
        Patch::in_text(0x01ca6484).bytes(&[0x00 , 0x10 , 0x2e , 0x1e]).unwrap();
        Patch::in_text(0x01ca9e80).bytes(&[0xff, 0x43, 0x02, 0xd1]).unwrap();
        println!("Arena battles are not skipped");
    }
}
impl ConfigBasicMenuItemSwitchMethods for ArenaMod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_arena(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let active = GameVariableManager::get_bool(ARENA_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(active);
        if active != result {
            GameVariableManager::set_bool(ARENA_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            patch_arena();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = if GameVariableManager::get_bool(ARENA_KEY) {  "Arena battles are skipped. (Required to use in menus)" } 
        else { "Arena battles are not skipped." }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if !GameVariableManager::get_bool(ARENA_KEY) { this.command_text = on_str(); }
        else { this.command_text = off_str(); }
    }
}
#[no_mangle]
extern "C" fn arena() -> &'static mut ConfigBasicMenuItem { 
    unsafe {
        let str0 = concat_strings3(Mess::get("MID_Hub_Arena"), " ".into(), Mess::get("MID_CONFIG_COMBATANIME"), None);
        ConfigBasicMenuItem::new_switch::<ArenaMod>(str0.get_string().unwrap()) 
    }
}
pub fn arena_install(){ cobapi::install_game_setting(arena); }