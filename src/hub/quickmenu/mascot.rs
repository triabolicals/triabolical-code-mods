use super::{*, super::data::{FoodstuffData, MascotFoodData}};
use engage::gamedata::item::ItemData;

pub mod strok;
pub mod food;

pub struct MascotQuickMenuItem;
impl BasicMenuItemMethods for  MascotQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            TitleBar::hide_footer();
            if !HubVariableMascot::is_found() {
                if !can_well() { 
                    HubVariableMascot::found(); 
                    let proc = MascotMenuSequence::instantiate().unwrap();
                    unsafe {  mascot_menu_ctor(proc, None); }
                    proc.create_bind(this.menu, create_mascot_desc(proc), "MascotMenuSequence");
                }
                else { unsafe { hub_mascot_seq(this.menu, None); }}
            }
            else {
                let proc = MascotMenuSequence::instantiate().unwrap();
                unsafe {  mascot_menu_ctor(proc, None); }
                proc.create_bind(this.menu, create_mascot_desc(proc), "MascotMenuSequence");
            }
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        } else { BasicMenuResult::se_miss() }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Mascot_Spot") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_H_Mascot_Spot") } 
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if engage::hub::HubUtil::is_comeplete("M004".into()) {
            if !can_well() {
                if HubVariableMascot::done_strok() && HubVariableMascot::is_done_food()  { BasicMenuItemAttribute::Disable }
                else { BasicMenuItemAttribute::Enable }
            } else {
                BasicMenuItemAttribute::Enable
            }
        }
        else { BasicMenuItemAttribute::Hide }
    }
}


fn custom_build_attr(_item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if !can_well() { BasicMenuItemAttribute::Hide }
    else { BasicMenuItemAttribute::Enable }
}


pub fn build_attr_mascot_menu_items() {
    let mascot_top_menu_item = Il2CppClass::from_name("App", "MascotTopMenu").unwrap();
    
    let strok = mascot_top_menu_item.get_nested_types().iter().find(|x| x.get_name() == "StrokMenuItem").unwrap();
    let mut_strok = Il2CppClass::from_il2cpptype(strok.get_type()).unwrap();
    mut_strok.get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = strok::strok_build_attr as _).unwrap();
    mut_strok.get_virtual_method_mut("ACall").map(|method| method.method_ptr = strok::strok_a_call as _).unwrap();
    
    let food = mascot_top_menu_item.get_nested_types().iter().find(|x| x.get_name() == "MealMenuItem").unwrap();
    let mut_food = Il2CppClass::from_il2cpptype(food.get_type()).unwrap();
    mut_food.get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = food::food_eat_build_attr as _).unwrap();
    mut_food.get_virtual_method_mut("ACall").map(|method| method.method_ptr = food::food_eat_a_call as _).unwrap();

    let custom = mascot_top_menu_item.get_nested_types().iter().find(|x| x.get_name() == "CustomMenuItem").unwrap();
    let mut_custom = Il2CppClass::from_il2cpptype(custom.get_type()).unwrap();
    mut_custom.get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = custom_build_attr as _).unwrap();
}

pub fn create_mascot_desc(proc: &'static MascotMenuSequence) -> &'static mut Il2CppArray<&'static mut ProcDesc> {
    let desc = unsafe { mascot_create_bind(proc, None) };
    for x in 19..31 { desc[x] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing)); }
    desc[31] = ProcDesc::call(ProcVoidMethod::new(proc, strok::add_strok_pts));
    desc[0x10] = ProcDesc::call(ProcVoidMethod::new(proc, food::mascot_create_food_eat_bind));
    desc
}

#[unity::class("App", "MascotFriendlyContent")]
pub struct MascotFriendlyContent {}
impl MascotFriendlyContent {
    pub fn try_popup(&self) { unsafe { try_popup_mascot_gauge(self, None);}}
}

#[unity::class("App", "MascotMenuSequence")]
pub struct MascotMenuSequence {
    junk: [u8; 0x68],
    pub mascot_friendly_gague: &'static MascotFriendlyContent,
    mascot_presentation: u64,
    top_menu_self: i32,
    custom_menu_result: i32,
    resource_handler: u64,
    pub food_stuff: Option<&'static FoodstuffData>,
}

impl Bindable for MascotMenuSequence {}

#[unity::from_offset("App", "MascotMenuSequence", "FriendlyPopup")]
fn mascotmenusequence_friendlypopup(this: &MascotMenuSequence, _method_info: OptionalMethod);

#[skyline::from_offset(0x020392d0)]
fn try_popup_mascot_gauge(this: &MascotFriendlyContent, _method_info: OptionalMethod);

#[skyline::from_offset(0x0203d6b0)]
fn mascot_menu_ctor(this: &MascotMenuSequence, _method_info: OptionalMethod);

#[skyline::from_offset(0x028b5e40)]
fn hub_mascot_seq(this: &BasicMenu<BasicMenuItem>, _method_info: OptionalMethod);

#[skyline::from_offset(0x0203bdf0)]
fn mascot_create_bind(this: &MascotMenuSequence, _method_info: OptionalMethod) -> &'static mut Array<&'static mut ProcDesc>;