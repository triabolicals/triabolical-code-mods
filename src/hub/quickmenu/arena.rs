use super::*;
use skyline::patching::Patch;
use engage::{
    gamedata::achieve::AchieveData,
    sequence::arenaordersequence::ArenaOrderSequence,
};

pub struct ArenaQuickMenuItem;
impl BasicMenuItemMethods for ArenaQuickMenuItem {
    extern "C" fn a_call (this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() {
            close_hub_mini_map();
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            ArenaOrderSequence::create_bind(this.menu);
            edit_arena_desc( this.menu.proc.child.as_mut().unwrap() );
            TitleBar::open_footer(2);   // BondFrag Only
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        } else { BasicMenuResult::se_miss() }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Arena") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_H_Hub_Arena") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_闘技場").unwrap().is_complete() {  BasicMenuItemAttribute::Enable  }
        else { BasicMenuItemAttribute::Hide }
    }
}

pub extern "C" fn arena_finish_training(this: &'static mut ArenaOrderSequence, _method_info: OptionalMethod){
    if let Some(god) = this.god_unit { this.training_unit.set_god_unit(god); }
    if let Some(ring) = this.ring { this.training_unit.set_ring(ring);}
    let dead_side = this.calculator.get_dead_side();

    if !this.is_emblem_battle {
        AchieveData::add_count_unit_battle_count();
        if dead_side == 1 { AchieveData::add_count_unit_battle_count(); }
    }
    else {
        AchieveData::add_count_god_battle_count();
        if dead_side == 1 { AchieveData::add_count_god_battle_win(); }
    }
    if this.training_type == 0 {
        let mut value = GameVariableManager::get_number("G_拠点_闘技場済み");
        value += 1; 
        GameVariableManager::set_number("G_拠点_闘技場済み", value);
    }
    post_combat_arena_patch();
}

pub fn edit_arena_desc(proc: &mut ProcInst) {
    // let arena_order = proc.cast_mut::<ArenaOrderSequence>();
    let descs = proc.descs.get_mut();
    let background_in: extern "C" fn(&mut ArenaOrderSequence, OptionalMethod) =  unsafe { std::mem::transmute( arena_background_in::get_ref().method_ptr) };
    let background_out: extern "C" fn(&mut ArenaOrderSequence, OptionalMethod)  =  unsafe { std::mem::transmute( arena_background_out::get_ref().method_ptr) };
        // 29 BlackOut(1.0 Layer=4)
        // 30 FadeWait(Layer=4)
        // 31 Unload Arena Prefabs
        // 32 Background out
        // 33 Setup Training
    [29, 30, 32, 33, 38].iter().for_each(|&x| descs[x] = ProcDesc::call(ProcVoidMethod::new(None, arena_do_nothing)) );
    descs[35] = ProcDesc::call(ProcVoidMethod::new(None, start_training));
    descs[36] = ProcDesc::call(ProcVoidMethod::new(None, arena_finish_training)); //FinishTraining
    // escs[38] = Fade::black_in(0.0, 4);  //BlackIn(1.0, 4)
    descs[2] = ProcDesc::call(ProcVoidMethod::new(None, background_in)); //BackgroundOut
    descs[44] = ProcDesc::call(ProcVoidMethod::new(None, background_out)); //BackgroundOut
}

pub extern "C" fn start_training(this: &mut ArenaOrderSequence,  _method_info: OptionalMethod) {
    pre_combat_arena_patch();
    this.start_training();
    
    let descs = this.proc.child.as_mut().unwrap().descs.get_mut();
    [2, 3, 10, 11].iter().for_each(|&x| descs[x] = ProcDesc::call(ProcVoidMethod::new(None, arena_do_nothing)) );
}

pub extern "C" fn arena_do_nothing(_arena_order: &'static mut ArenaOrderSequence, _method_info: OptionalMethod){}

pub fn pre_combat_arena_patch() {
    let set_false = &[0x00, 0x00, 0x80, 0x52];
    let set_return = &[0xC0, 0x03, 0x5F, 0xD6];
    let set_nop =  &[0x1F,0x20,0x03,0xD5];
// bool Combat.ArenaCombatSequence.<Grow2>d__36$$MoveNext
    Patch::in_text(0x01bac5d0).bytes(set_nop).unwrap();
    Patch::in_text(0x01bac6bc).bytes(set_nop).unwrap();
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
//  Combat.ArenaCombatSequence.<Grow1>d__35$$MoveNext
    Patch::in_text(0x01babef4).bytes(&[0xe0, 0x03, 0x27, 0x1e]).unwrap();   
    Patch::in_text(0x01babef8).bytes(&[0xe1, 0x03, 0x27, 0x1e]).unwrap();
//  Combat.ArenaCombatSequence.<Grow2>d__36$$MoveNext
    Patch::in_text(0x01bac194).bytes(&[0xe1, 0x03, 0x27, 0x1e]).unwrap();

}
pub fn post_combat_arena_patch() {
    //Patch::in_text(0x01caa414).bytes(&[0xbb, 0x2f, 0x1f, 0x94]).unwrap();
// bool Combat.ArenaCombatSequence.<Grow2>d__36$$MoveNext
    Patch::in_text(0x01bac5d0).bytes(&[0x54, 0xab, 0x13, 0x94]).unwrap();
    Patch::in_text(0x01bacb40).bytes(&[0xfd , 0x7b , 0xbd , 0xa9]).unwrap();
// Combat.ArenaCombatSequence.<StartFight>d__32$$MoveNext
    Patch::in_text(0x01bacb44).bytes(&[0xf5 , 0x0b , 0x00 , 0xf9]).unwrap();
    Patch::in_text(0x01bac790).bytes(&[0xfd , 0x7b , 0xbc , 0xa9]).unwrap();
// Combat.ArenaCombatSequence.<Setup>d__30$$MoveNext
    Patch::in_text(0x01bac794).bytes(&[0xf7 , 0x0b , 0x00 , 0xf9]).unwrap();
    Patch::in_text(0x01bacd50).bytes(&[0xfd , 0x7b , 0xbe , 0xa9]).unwrap();
// Combat.ArenaCombatSequence.<WaitBegin>d__31$$MoveNext
    Patch::in_text(0x01bacd54).bytes(&[0xf4 , 0x4f , 0x01 , 0xa9]).unwrap();
    Patch::in_text(0x01bacf60).bytes(&[0xe8 , 0x0f , 0x1d , 0xfc]).unwrap();
// Combat.ArenaCombatSequence.<WaitFinish>d__33$$MoveNext
    Patch::in_text(0x01bacf64).bytes(&[0xfd , 0x7b , 0x01 , 0xa9]).unwrap();
    Patch::in_text(0x01bab900).bytes(&[0xfd , 0x7b , 0xbc , 0xa9]).unwrap();
// Combat.ArenaCombatSequence.<Exit>d__38$$MoveNext
    Patch::in_text(0x01bab904).bytes(&[0xf7 , 0x0b , 0x00 , 0xf9]).unwrap();
    Patch::in_text(0x01bac6bc).bytes(&[0xe1 , 0xd2 , 0x18 , 0x94]).unwrap();
//  Combat.ArenaCombatSequence.<Grow1>d__35$$MoveNext
    Patch::in_text(0x01babef4).bytes(&[0x00, 0x90, 0x20, 0x1e]).unwrap();
    Patch::in_text(0x01babef8).bytes(&[0x01, 0x90, 0x2c, 0x1e]).unwrap();
//  Combat.ArenaCombatSequence.<Grow2>d__36$$MoveNext    
    Patch::in_text(0x01bac194).bytes(&[0x01, 0x10, 0x2e, 0x1e]).unwrap();
}

#[unity::from_offset("App", "ArenaOrderSequence", "BackgroundIn")]
fn arena_background_in(this: &'static mut ArenaOrderSequence,  method_info: OptionalMethod);

#[unity::from_offset("App", "ArenaOrderSequence", "StartTraining")]
fn arena_start_training(this: &ArenaOrderSequence,  method_info: OptionalMethod);

#[unity::from_offset("App", "ArenaOrderSequence", "BackgroundOut")]
fn arena_background_out(this: &'static mut ArenaOrderSequence,  method_info: OptionalMethod);
