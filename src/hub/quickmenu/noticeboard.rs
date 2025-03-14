use super::*;


pub struct NoticeBoardQuickMenuItem;
impl BasicMenuItemMethods for NoticeBoardQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            unsafe { noticeboard_create_bind(this.menu, None) };
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        } else { BasicMenuResult::se_miss() }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_MENU_NOTICE_BOARD_TITLE") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get( "MID_MENU_NOTICE_BOARD_HELP") }
}

#[unity::from_offset("App", "NoticeBoardSequence", "CreateBind")]
fn noticeboard_create_bind<P: Bindable>(parent: &P, method_info: OptionalMethod);

pub fn notice_somniel_map_build_attr(_this: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if can_well() { BasicMenuItemAttribute::Enable }
    else { BasicMenuItemAttribute::Hide  }
}

pub fn notice_somniel_map_build_attr_adjust() {
    crate::utils::get_nested_virtual_methods_mut("App", "NoticeBoardTopMenu", "SolanelInfoItem", "BuildAttribute")
        .map(|method| method.method_ptr = notice_somniel_map_build_attr as _ ).unwrap();
}