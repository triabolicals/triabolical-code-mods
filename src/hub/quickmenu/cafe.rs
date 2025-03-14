use super::*;
use engage::gamedata::cook::CookData;

pub mod chef;

pub struct CookQuickMenuItem; 
impl BasicMenuItemMethods for CookQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() || GameVariableManager::get_bool("G_拠点_料理済み") || unsafe { get_cook_data(None).is_none() } { return BasicMenuResult::se_miss(); }
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all =
                unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        close_hub_mini_map();
        TitleBar::hide_footer();
        unsafe { cook_sequence_create_bind(this.menu, None) };
        edit_cooking_desc(this.menu.proc.child.as_mut().unwrap().descs.get_mut());
        this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();

        BasicMenuResult::se_decide()
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Cafe") }
    // extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_H_Hub_Cafe") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_料理").unwrap().is_complete() { 
            if HubUtil::get_current_cooking_pid().is_none() || unsafe { get_cook_data(None).is_none() } {
                BasicMenuItemAttribute::Hide
            }
            else if GameVariableManager::get_bool("G_拠点_料理済み") {  BasicMenuItemAttribute::Disable }
            else { BasicMenuItemAttribute::Enable}
        }
        else { BasicMenuItemAttribute::Hide }
    }
}

pub fn edit_cooking_desc(cook_descs: &mut Array<&mut ProcDesc>) {
    for x in 14..27 { cook_descs[x] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing)); }
    for x in 28 ..42 { cook_descs[x] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));  }
    cook_descs[43] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    cook_descs[44] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
}

fn hub_talk_after_cook_build_attr(_this: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if HubFacilityData::get("AID_料理").unwrap().is_complete() { 
        if HubUtil::get_current_cooking_pid().is_none() || unsafe { get_cook_data(None).is_none() } {
            BasicMenuItemAttribute::Hide
        }
        else if GameVariableManager::get_bool("G_拠点_料理済み") {  BasicMenuItemAttribute::Disable }
        else { BasicMenuItemAttribute::Enable   }
    }
    else { BasicMenuItemAttribute::Hide }
}

pub fn hub_talk_cook_build_attr_install() {
    crate::utils::get_nested_virtual_methods_mut("App", "HubPlayTalkAfter", "CookingMenu", "BuildAttribute")
        .map(|method| method.method_ptr = hub_talk_after_cook_build_attr as _ ).unwrap();
}

#[unity::from_offset("App", "HubCookingSequence", "CreateBind")]
fn cook_sequence_create_bind<P: Bindable>(parent: &P, method_info: OptionalMethod);

#[skyline::from_offset(0x02d86ce0)]
pub fn get_cook_data(method_info: OptionalMethod) -> Option<&'static CookData>;