use unity::prelude::*;
use engage::gameuserdata::*;
use engage::gamedata::*;

//Stat Capability
#[unity::class("App", "Capability")]
pub struct Capability {}

#[unity::class("App", "SkillArray")]
pub struct SkillArray {}

#[unity::class("App", "PersonData_FlagField")]
pub struct PersonData_FlagField {}

#[unity::class("App", "SkillData")]
pub struct SkillData {
    pub parent: StructBaseFields,
    pub sid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    //
}
impl Gamedata for SkillData {}

//Getter from App.PersonData
#[unity::from_offset("App", "PersonData", "get_Pid")]
pub fn get_Pid(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "PersonData", "get_Name")] //#[skyline::from_offset(0x1f25d40)]
pub fn get_Name(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "PersonData", "get_UnitIconID")] //#[skyline::from_offset(0x1f25d20)]
pub fn get_UnitIconID(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "PersonData", "get_Gender")] //#[skyline::from_offset(0x1f25da0)]
pub fn get_Gender(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "get_Grow")] //#[skyline::from_offset(0x1f26020)]
pub fn get_Grow(this: &PersonData, method_info: OptionalMethod) -> &Capability;

#[unity::from_offset("App", "PersonData", "get_CommonSids")] //#[skyline::from_offset(0x1f26040)]
pub fn get_CommonSids(this: &PersonData, method_info: OptionalMethod) -> &il2cpp::object::Array<Il2CppString>;

#[skyline::from_offset(0x1f2a6f0)]
pub fn get_CommonSkill(this: &PersonData, method_info: OptionalMethod) -> &SkillArray;

#[skyline::from_offset(0x1f26000)]
pub fn get_limit(this: &PersonData, method_info: OptionalMethod) -> &Capability;

#[skyline::from_offset(0x1f2a790)]
pub fn get_facedata(this: &PersonData, method_info: OptionalMethod) -> &PersonData;

#[skyline::from_offset(0x1f26160)]
pub fn get_ascii_name(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x1f25f40)]
pub fn get_flag(this: &PersonData, method_info: OptionalMethod) -> &PersonData_FlagField;

#[skyline::from_offset(0x1f261a0)]
pub fn get_attrs(this: &PersonData, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x1f29e30)]
pub fn GetJob(this: &PersonData, method_info: OptionalMethod) -> &JobData;

#[skyline::from_offset(0x1f25c60)]
pub fn get_jid(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x1f25dc0)]
pub fn get_level(this: &PersonData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x1f25de0)]
pub fn get_InternalLevel(this: &PersonData, method_info: OptionalMethod) -> i8;

//Setters from App.PersonData.set_
#[skyline::from_offset(0x1f25dd0)]
pub fn set_level(this: &PersonData, value: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25db0)]
pub fn set_gender(this: &PersonData, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25c50)]
pub fn set_name(this: &PersonData, name: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26050)]
pub fn set_commonSids(this: &PersonData, value: &il2cpp::object::Array<Il2CppString>, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25d30)]
pub fn set_UnitIconID(this: &PersonData, name: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26030)]
pub fn set_grow(this: &PersonData, value: &Capability, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26010)]
pub fn set_limit(this: &PersonData, value: &Capability, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25cd0)]
pub fn set_help(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f2a700)]
pub fn set_CommonSkill(this: &PersonData, value : &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x1f2a7a0)]
pub fn set_facedata(this: &PersonData, value : &PersonData, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26170)]
pub fn set_ascii_name(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25f50)]
pub fn set_flag(this: &PersonData, value: &PersonData_FlagField, method_info: OptionalMethod);

#[skyline::from_offset(0x1f261b0)]
pub fn set_attrs(this: &PersonData, value: i32, method_info: OptionalMethod);

//Wrappers
pub fn SetName(this: &PersonData, name: &str){
    unsafe { set_name(this, name.into(), None); }
}
pub fn SetGender(this: &PersonData, gender: i32){
    unsafe {set_gender(this, gender, None); }
}

pub fn set_person(src: &PersonData, dst: &PersonData){
    unsafe {
        let grow = get_Grow(src, None);
        let sids = get_CommonSkill(src, None);
        let icon = get_UnitIconID(src, None);
        let limit = get_limit(src, None);
        let gender = get_Gender(src, None);
        let face = get_facedata(src, None);
        let ascii = get_ascii_name(src, None);
        let flags = get_flag(src, None);
        let attrs = get_attrs(src, None);

        set_attrs(dst, attrs, None);
        set_flag(dst, flags, None);
        set_ascii_name(dst, ascii, None);
        set_facedata(dst, face, None);
        set_grow(dst, grow, None);
        set_CommonSkill(dst, sids, None);
        set_limit(dst, limit, None);
        set_gender(dst, gender, None);
        set_UnitIconID(dst, icon, None);
    }


}
#[skyline::from_offset(0x2053ea0)]
pub fn get_job_internal_level(this: &JobData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;
// Jobdata

#[skyline::from_offset(0x2b4afa0)]
pub fn GetAverageLevel(difficulty: i32, sortieCount: i32, method_info: OptionalMethod) -> i32;

/*
pub struct Person {
    name : String,
    unit_icon_id : String,
    gender : i32,
    grow : &'static Capability,
    commonSids : &'static il2cpp::object::Array<Il2CppString>,
}
impl Person {
    fn new (x: &PersonData) -> Self {
        unsafe { 
        Self { name : get_name(x, None).get_string().unwrap(),
             unit_icon_id : get_unitIconID(x, None).get_string().unwrap(),
             gender : get_Gender(x, None), 
             grow : get_Grow(x, None),
            commonSids : get_CommonSids(x, None),
        }
    }
}
}
*/