use super::*;

pub struct WeaponQuickMenuItem; 
impl BasicMenuItemMethods for WeaponQuickMenuItem {
    extern "C" fn a_call (this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            unsafe { hub_weapon_create_bind(this.menu, None); }
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_WeaponShop") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_武器屋").unwrap().is_complete() { BasicMenuItemAttribute::Enable } else { BasicMenuItemAttribute::Hide }
    }
}


pub struct ItemShopQuickMenuItem;
impl BasicMenuItemMethods for ItemShopQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            TitleBar::show_footer();
            close_hub_mini_map();
            unsafe { hub_item_create_bind(this.menu, None); }
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_ItemShop") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if let Some(fac) = HubFacilityData::get("AID_道具屋") { if fac.is_complete() { return BasicMenuItemAttribute::Enable;  } }
        return BasicMenuItemAttribute::Hide;
    }
}


pub struct RefineQuickMenuItem; 
impl BasicMenuItemMethods for RefineQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            TitleBar::show_footer();
            unsafe { hub_refine_create_bind(this.menu, None); }
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Blacksmith") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_錬成屋").unwrap().is_complete() { BasicMenuItemAttribute::Enable } else { BasicMenuItemAttribute::Hide }
    }
}

pub struct AccessoryQuickMenuItem; 
impl BasicMenuItemMethods for AccessoryQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            TitleBar::show_footer();
            unsafe { hub_accessory_create_bind(this.menu, 0, None); }
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_AccessoryShop") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_アクセサリ").unwrap().is_complete() { if can_well() { return BasicMenuItemAttribute::Enable; } }
        return BasicMenuItemAttribute::Hide;
    }
}

pub struct JukeBoxMenuItem; 
impl BasicMenuItemMethods for JukeBoxMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all =
                    unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            TitleBar::hide_footer();
            unsafe { juke_box_create_bind(this.menu, None); }
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_CafeTerrace_Jukebox") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_CafeTerrace_H_Jukebox") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if GameUserData::get_sequence() == 4 && HubFacilityData::get("AID_ジュークボックス").unwrap().is_complete() {  BasicMenuItemAttribute::Enable }
        else { BasicMenuItemAttribute::Hide }
    }
}

pub struct RefreshMenuItem; 
impl BasicMenuItemMethods for RefreshMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all =
                    unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            TitleBar::hide_footer();
            unsafe { refresh_sequence_create_bind(this.menu, "AID_水遊び".into(), None) };
            this.menu.proc.child.as_mut().unwrap().descs.get_mut()[20] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Poolside") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_H_Hub_Poolside") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_水遊び").unwrap().is_complete() {  BasicMenuItemAttribute::Enable }
        else { BasicMenuItemAttribute::Hide }
    }
}

pub struct FleaMarketMenuItem; 
impl BasicMenuItemMethods for FleaMarketMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if this.is_attribute_disable() { BasicMenuResult::se_miss() }
        else {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all =
                    unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            unsafe { flea_market_bind(this.menu, GameUserData::get_sequence() == 4, None) };
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_FreeMarket") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_蚤の市").unwrap().is_complete() {  BasicMenuItemAttribute::Enable }
        else { BasicMenuItemAttribute::Hide }
    }
}

#[unity::from_offset("App", "RefreshSequence", "CreateBind")]
fn refresh_sequence_create_bind<P: Bindable>(parent: &P, aid: &Il2CppString, _method_info: OptionalMethod);

#[unity::from_offset("App", "HubFleaMarketSequence", "CreateBind")]
fn flea_market_bind<P: Bindable>(parent: &P, is_voice: bool, _method_info: OptionalMethod);

#[unity::from_offset("App", "HubWeaponShopSequence", "CreateBind")]
fn hub_weapon_create_bind<P: Bindable>(parent: &P, _method_info: OptionalMethod);

#[unity::from_offset("App", "HubItemShopSequence", "CreateBind")]
fn hub_item_create_bind<P: Bindable>(parent: &P, _method_info: OptionalMethod);

#[unity::from_offset("App", "HubRefineShopSequence", "CreateBind")]
fn hub_refine_create_bind<P: Bindable>(parent: &P, _method_info: OptionalMethod);

#[unity::from_offset("App", "HubAccessoryRoom", "CreateBind")]
fn hub_accessory_create_bind<P: Bindable>(parent: &P, shop: i32, _method_info: OptionalMethod);

#[unity::from_offset("App", "JukeboxSequence", "CreateBind")]
fn juke_box_create_bind<P: Bindable>(parent: &P, _method_info: OptionalMethod);