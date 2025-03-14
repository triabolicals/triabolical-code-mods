use super::*;

pub struct GodRoomQuickMenuItem; 
impl BasicMenuItemMethods for GodRoomQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
               let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
               close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            unsafe { god_room_create_bind(this.menu, None) };
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _).unwrap();
            BasicMenuResult::se_decide() 
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_GodRoom") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_H_Hub_GodRoom") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_紋章士の間").unwrap().is_complete() { BasicMenuItemAttribute::Enable } else { BasicMenuItemAttribute::Hide }
    }
}

fn ring_cleaning_build_attr(this: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if this.menu.proc.parent.as_ref().unwrap().parent.as_ref().unwrap().name.unwrap().to_string() == "HubSequence" { BasicMenuItemAttribute::Enable  }
    else {  BasicMenuItemAttribute::Hide  }
}

pub fn hide_ring_cleaning() {
    crate::utils::get_nested_virtual_methods_mut("App", "GodRoomPedestalTopMenu", "RingCleaningItem", "BuildAttribute")
        .map(|method| method.method_ptr = ring_cleaning_build_attr as _ ).unwrap();
}
#[unity::from_offset("App", "GodRoomPedestalSequence", "CreateBind")]
fn god_room_create_bind<P: Bindable>(parent: &P, method_info: OptionalMethod);

