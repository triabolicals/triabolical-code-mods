use unity::prelude::*;
use unity::system::List;
use engage::{gamedata::unit::Unit, tmpro::TextMeshProUGUI};

#[unity::class("App", "UnitStatusSetter")]
pub struct UnitStatusSetter {
    junk: [u8; 0x118],
    pub capability: &'static List<CapabilityValueParam>,
    junk2: [u64; 11],
    pub level: &'static UnitStatusSetterValueParam,
}

#[unity::class("App", "UnitStatusSetterValueParam")]
pub struct UnitStatusSetterValueParam {
    setter: &'static UnitStatusSetter,
    m_root_ptr: u64,
    pub title: &'static mut TextMeshProUGUI,
    value: &'static TextMeshProUGUI,
    //
}
#[unity::class("", "CapabilityValueParam")]
pub struct CapabilityValueParam {
    setter: &'static UnitStatusSetter,
    m_root_ptr: u64,
    pub title: &'static TextMeshProUGUI,
    pub value: &'static TextMeshProUGUI,
    jubk: [u8; 0x28],
    pub cap_type: i32,
    //
}
#[unity::class("App", "UnitInfoParamSetter")]
pub struct UnitInfoParamSetter {
    parent: u64,
    engine: u64,
    pub simple_ui: i32,
    junk : [u8; 136],
    pub level : &'static mut TextMeshProUGUI,
}
#[unity::class("App","LevelUpWindowController")]
pub struct LevelUpWindowController {
    junk: u64,
    pub char_name: &'static TextMeshProUGUI,
    pub title_level: &'static TextMeshProUGUI,
    pub level: &'static mut TextMeshProUGUI,
    pub job: &'static mut TextMeshProUGUI,
}
/// Level up Window
#[unity::hook("App","LevelUpWindowController", "SetupParams")]
pub fn level_up_window_setup_hook(this: &mut LevelUpWindowController, unit: &Unit, next: &Unit, method_info: OptionalMethod){
    call_original!(this, unit, next, method_info);
    if unit.internal_level > 0 {
        let level_str = format!("{}/{}", unit.level, unit.internal_level).into();
        this.level.set_text(level_str, true);
    }
}
/// Unit Info 
/*
#[skyline::hook(offset = 0x1f9d320)]
pub fn unit_info_set_level_hook(this: &mut UnitInfoParamSetter, unit: Option<&Unit>, x: i32, z: i32, selected_god: bool, god: &GodUnit, method_info: OptionalMethod){
    call_original!(this, unit, x, z, selected_god, god, method_info);
    if let Some(p) = unit { 
        this.level.rect_transform.iter().for_each(|v|println!("{} {} {}", v.0, v.1, v.2) );
        let displayed_level= p.get_enchanced_level();
        if p.internal_level > 0 {
            let level_text = format!("{}/{}", displayed_level, p.internal_level);
            this.level.set_text(level_text.into(), true);
        }
    }
}
*/
// /Hooking to where the game sets the level display in the unit status screen
#[skyline::hook(offset = 0x1c66980)]
pub fn set_total_level(this: &UnitStatusSetter, unit: &Unit, unit_no_enhance: &Unit, method_info: OptionalMethod){
    call_original!(this, unit, unit_no_enhance, method_info);
    let enhance_level= unit.get_enchanced_level();
    let not_enhance_level = unit_no_enhance.get_enchanced_level();
    let unit_level = unit_no_enhance.level;
    let max_level = unit_no_enhance.job.get_max_level();
    let boost: i32 = (not_enhance_level < enhance_level) as i32;
    let at_limit: bool = max_level <= unit_level;
    let displayed_level = enhance_level;
    let internal_level = unit_no_enhance.internal_level;
    if internal_level > 0 {
        let level_str = format!("{}/{}", displayed_level, internal_level).into();
        unsafe {  set_value_direct(this.level, level_str , boost, at_limit, None) };
    }
}

#[skyline::from_offset(0x290f1c0)]
pub fn try_set_text(tmp: &TextMeshProUGUI, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x0290f0a0)]
pub fn try_set_text_string(tmp: &TextMeshProUGUI, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1b58360)]
pub fn set_value_direct(this: &UnitStatusSetterValueParam, str: &Il2CppString, dir: i32, is_limit: bool, method_info: OptionalMethod);


