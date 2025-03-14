use desc::ProcDescType;
use engage::proc::*;
pub use super::*;

pub struct MuscleMenuItem; 
impl BasicMenuItemMethods for MuscleMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() && GameVariableManager::get_number("G_MuscleExercise_GetPrize") == 0 {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all =
                    unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            TitleBar::hide_footer();
            unsafe {
                super::muscle::muscle_bind(this.menu, None);
                super::muscle::muscle_descs_edit(this.menu.proc.child.as_mut().unwrap().descs.get_mut());
                this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            }
            BasicMenuResult::se_decide()
        } else { BasicMenuResult::se_miss() }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_TrainingSpot") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_H_Hub_TrainingSpot") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_筋肉体操").unwrap().is_complete() { 
            if GameVariableManager::get_number("G_MuscleExercise_GetPrize") == 0 {
                BasicMenuItemAttribute::Enable
            }
            else { BasicMenuItemAttribute::Disable }
        }
        else { BasicMenuItemAttribute::Hide }
    }
}


#[unity::class("App", "MuscleExerciseSequence")]
pub struct MuscleExerciseSequence {
    pub proc: ProcInstFields,
    junk: [i32; 3],
    pub selected_level: i32,
    pub assist: bool,
}
impl Bindable for MuscleExerciseSequence {}

#[skyline::from_offset(0x02dd8250)]
pub fn muscle_bind(parent: &mut BasicMenu<BasicMenuItem>, _method_info: OptionalMethod);

#[skyline::from_offset(0x02dd7d60)]
fn muscle_excerise_bind<P: Bindable>(proc: &P,  _method_info: OptionalMethod);

#[skyline::from_offset(0x02786b70)]
fn push_up_sequence_bind<P: Bindable>(proc: &P,  level: i32, assist: bool, _method_info: OptionalMethod);

#[skyline::from_offset(0x02dc96b0)]
fn sit_up_sequence_bind<P: Bindable>(proc: &P,  level: i32, assist: bool, _method_info: OptionalMethod);
#[skyline::from_offset(0x02dd4b00)]
fn squat_sequence_bind<P: Bindable>(proc: &P,  level: i32, assist: bool, _method_info: OptionalMethod);

pub fn muscle_descs_edit(descs: &mut Array<&mut ProcDesc>) {
    descs[3] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    descs[28] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    descs[26] = ProcDesc::call(ProcVoidMethod::new(None, muscle_excerise_sequence_edit));

    descs[24] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    descs[25] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));

    descs[31] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    descs[32] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));

    descs[39] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    descs[40] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
}

extern "C" fn muscle_excerise_sequence_edit(proc: &mut ProcInst, _method_info: OptionalMethod) {
    unsafe { muscle_excerise_bind(proc, None); }
    let desc = proc.child.as_mut().unwrap().descs.get_mut();
    desc[10] = ProcDesc::call(ProcVoidMethod::new(None, push_up_sequence_edit));
    desc[14] = ProcDesc::call(ProcVoidMethod::new(None, sit_up_sequence_edit));
    desc[18] = ProcDesc::call(ProcVoidMethod::new(None, squat_sequence_edit));
    desc[22] = ProcDesc::call(ProcVoidMethod::new(None,proc_do_nothing));
}

extern "C" fn push_up_sequence_edit(proc: &mut MuscleExerciseSequence, _method_info: OptionalMethod) {
    unsafe { push_up_sequence_bind(proc, proc.selected_level, proc.assist, None);}
    let desc = proc.proc.child.as_mut().unwrap().descs.get_mut();
    for x in 0..desc.len() {
        if x >= 32 && x <= 38 {
            continue;
        }
        if desc[x].ty != ProcDescType::Label && desc[x].ty != ProcDescType::End && desc[x].ty != ProcDescType::Jump {
            desc[x] = ProcDesc::call(ProcVoidMethod::new(None,proc_do_nothing));
        }
    }
    desc[22] = ProcDesc::call(ProcVoidMethod::new(None, muscle_push_up_perfect_rank));
}

extern "C" fn sit_up_sequence_edit(proc: &mut MuscleExerciseSequence, _method_info: OptionalMethod) {
    unsafe { sit_up_sequence_bind(proc, proc.selected_level, proc.assist, None);}
    let desc = proc.proc.child.as_mut().unwrap().descs.get_mut();
    for x in 0..desc.len() {
        if x >= 32 && x <= 38 {
            continue;
        }
        if desc[x].ty != ProcDescType::Label && desc[x].ty != ProcDescType::End && desc[x].ty != ProcDescType::Jump {
            desc[x] = ProcDesc::call(ProcVoidMethod::new(None,proc_do_nothing));
        }
    }
    desc[22] = ProcDesc::call(ProcVoidMethod::new(None, muscle_sit_up_perfect_rank));
}

extern "C" fn squat_sequence_edit(proc: &mut MuscleExerciseSequence, _method_info: OptionalMethod) {
    unsafe { squat_sequence_bind(proc, proc.selected_level, proc.assist, None);}
    let desc = proc.proc.child.as_mut().unwrap().descs.get_mut();
    for x in 0..desc.len() {
        if x >= 33 && x <= 39  {
            continue;
        }
        if desc[x].ty != ProcDescType::Label && desc[x].ty != ProcDescType::End && desc[x].ty != ProcDescType::Jump {
            desc[x] = ProcDesc::call(ProcVoidMethod::new(None,proc_do_nothing));
        }
    }
    desc[23] = ProcDesc::call(ProcVoidMethod::new(None, muscle_squat_perfect_rank));
}

#[unity::class("App", "PushUp.MusclePushupSequence")]
pub struct MusclePushupSequence {
    junk: [u8; 0x240],
    pub score: [i32; 3],
    pub count: [i32; 4],

}
impl Bindable for MusclePushupSequence {}

#[unity::class("App", "Situp.MuscleSitupSequence")]
pub struct MuscleSitupSequence {
    junk: [u8; 0x1ec],
    pub perfect_count: i32,
    pub good_count: i32,
    junk2: [u8; 0x78],
    pub perfect_score: i32,
}
impl Bindable for MuscleSitupSequence {}

#[skyline::from_offset(0x02dc67a0)]
fn sit_up_calc_rank(this: &MuscleSitupSequence, _method_info: OptionalMethod);

extern "C" fn muscle_sit_up_perfect_rank(this: &mut MuscleSitupSequence, _method_info: OptionalMethod) {
    this.perfect_count = 99;
    this.perfect_score = 1000;
    unsafe { sit_up_calc_rank(this, None);}
}



#[unity::class("App", "Squat.MuscleSquatSequence")]
pub struct MuscleSquatSequence {
    junk: [u8; 0x1cc],
    pub perfect_score: i32,
    pub target_score: i32,
    pub endless_count: i32,
    pub bad_count: i32,
    pub perfect_count: i32,
    pub good_count: i32,

}
impl Bindable for MuscleSquatSequence {}

#[skyline::from_offset(0x02790240)]
fn push_up_calc_rank(this: &MusclePushupSequence, _method_info: OptionalMethod);

extern "C" fn muscle_push_up_perfect_rank(this: &mut MusclePushupSequence, _method_info: OptionalMethod) {
    this.count[1] = 99;
    this.count[2] = 99;
    this.score[0] = 1000;
    this.score[1] = 1000;
    unsafe { push_up_calc_rank(this, None);}
}

#[skyline::from_offset(0x02dd1b20)]
fn squat_calc_rank(this: &MuscleSquatSequence, _method_info: OptionalMethod);

extern "C" fn muscle_squat_perfect_rank(this: &mut MuscleSquatSequence, _method_info: OptionalMethod) {
    this.perfect_count = 99;
    this.perfect_score = 1000;
    unsafe { squat_calc_rank(this, None);}
}

