use super::*;

pub struct AnimalMenuItem;
impl BasicMenuItemMethods for AnimalMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() {  BasicMenuResult::se_miss() }
        else {
            TitleBar::hide_footer();
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            let proc = unsafe {  animal_create_bind(this.menu, None) };
            proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            if !can_well() {
                let desc = this.menu.proc.child.as_mut().unwrap().descs.get_mut();
                desc[10] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
                desc[11] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
            }
            BasicMenuResult::se_decide()
        }

    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Animal") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_H_Hub_Farm") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if let Some(fac) = HubFacilityData::get("AID_動物小屋"){ if fac.is_complete() { return BasicMenuItemAttribute::Enable;  } }
       return BasicMenuItemAttribute::Hide;
    }
}

#[skyline::from_offset(0x01c9bb30)]
fn animal_create_bind<P: Bindable>(parent: &P, method_info: OptionalMethod) -> &'static mut ProcInst;