use super::*;

pub struct DragonRideMenuItem; 
impl BasicMenuItemMethods for DragonRideMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() && GameVariableManager::get_number("G_DragonRide_GetPrize") == 0 {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all =
                    unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            TitleBar::hide_footer();
            unsafe { dragon_ride_bind(this.menu, false, None) };
            dragon_ride_edit_desc(this.menu.proc.child.as_mut().unwrap().descs.get_mut());
                this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        } else { BasicMenuResult::se_miss() }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Talk_DragonRide") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_ドラゴンライド").unwrap().is_complete() { 
            if GameVariableManager::get_number("G_DragonRide_GetPrize") == 0 {
                BasicMenuItemAttribute::Enable
            }
            else { BasicMenuItemAttribute::Disable }
        }
        else { BasicMenuItemAttribute::Hide }
    }
}


#[skyline::from_offset(0x02ad6c40)]
pub fn dragon_ride_bind<P: Bindable>(parent: &P, debug: bool, _method_info: OptionalMethod);

pub fn dragon_ride_edit_desc(desc: &mut Array<&mut ProcDesc>) {
    //desc.iter_mut().filter(|x|x.ty == ProcDescType::User).map(|x| *x = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing)));
    unsafe { set_dragon_ride_rank(1, None); }
    [7, 13, 16, 22, 23, 26, 27, 28, 29, 42].into_iter().for_each(|x| desc[x as usize] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing)));
    desc[30] = ProcDesc::jump(10);
    //desc[34] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
}

#[skyline::from_offset(0x02a64fd0)]
fn set_dragon_ride_rank(value:i32, _method_info: OptionalMethod);
