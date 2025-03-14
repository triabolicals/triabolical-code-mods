pub use unity::{
    prelude::*,
    system::List,
    il2cpp::object::Array,
};
pub use engage:: {
    menu::{BasicMenu, BasicMenuItemMethods, BasicMenuItem, BasicMenuResult, BasicMenuItemAttribute, BasicMenuMethods, BasicMenuContent, BasicMenuSelect},
    mess::Mess,
    gameuserdata::GameUserData, gamevariable::GameVariableManager,
    hub::{hubsequence::HubSequence, access::*, *},
    gamedata::{Gamedata, HubFacilityData},
    proc::{ProcVoidMethod, desc::ProcDesc, ProcInst, Bindable},
    titlebar::TitleBar,
};
pub use super::*;
pub use crate::utils::{proc_do_nothing, open_anime_all_ondispose};

pub mod arena;
pub mod fishing;
pub mod mascot;
pub mod muscle;
pub mod well;
pub mod dragonride;
pub mod godroom;
pub mod cafe;
pub mod hubshops;
pub mod animal;
pub mod noticeboard;

pub fn can_well() -> bool {
    if GameUserData::get_sequence() == 4 {  HubUtil::get_current_scene_name().to_string() == "Hub_Solanel" }
    else { false }
}

pub fn close_hub_mini_map() {
    let sequence = GameUserData::get_sequence();
    if sequence == 4 || sequence == 5 {
        let hub = HubSequence::get_instance();
        let mini_map = hub.get_mini_map();
        if mini_map.is_show() { mini_map.set_mode(0); }
        mini_map.hide_system_menu();
    }
    else if sequence == 6 { 
        if let Some(gmap) =  engage::util::get_singleton_proc_instance::<engage::sequence::gmap_sequence::GmapSequence>() {
            gmap.map_info.close();
        }
    }
}

pub fn hub_menu_build_attrs_install() {
    mascot::build_attr_mascot_menu_items();
    godroom::hide_ring_cleaning();
    well::well_evil_build_attr_change();
    noticeboard::notice_somniel_map_build_attr_adjust();
    cafe::hub_talk_cook_build_attr_install();
}

pub struct HubMenuStaticFields {
    pub selected: &'static BasicMenuSelect,
}

#[unity::hook("App", "HubMenu", "CreateBind")]
pub fn hub_menu_create_bind(proc: &mut ProcInst,_method_info: OptionalMethod){
    call_original!(proc, None);
    // println!("HubMenu binding onto: {}", proc.get_class().get_name());
    let config_menu = proc.child.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>();
    if GameUserData::get_sequence() == 4 {
        for x in 0..config_menu.full_menu_item_list.len() {
            if config_menu.full_menu_item_list.items[x].get_class_mut().get_name() == "MapInfoItem" {

                config_menu.full_menu_item_list.items[x].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = hub_quick_menu_item_acall as _);
                config_menu.full_menu_item_list.items[x].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = hub_quick_menu_get_name as _);
                config_menu.full_menu_item_list.items[x].get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = hub_fast_travel_get_help as _);
                if GameVariableManager::get_bool("G_Cleared_M004") {
                    config_menu.reserved_show_row_num += 1;
                    let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[x].get_class().clone();
                    let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
                    new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = hub_market_item_acall as _);
                    new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = hub_market_get_name as _);
                    new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = hub_market_get_help as _);
                    config_menu.full_menu_item_list.insert(x as i32, new_menu_item);
                    break;
                }
            }
        }
    }
    else if GameVariableManager::get_bool("G_Cleared_M004") {
        let len = config_menu.full_menu_item_list.len() as i32;
        config_menu.reserved_show_row_num += 2;
        let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[1].get_class().clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = hub_quick_menu_item_acall as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = hub_quick_menu_get_name as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = hub_fast_travel_get_help as _);
        config_menu.full_menu_item_list.insert(len - 3, new_menu_item);
        let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[1].get_class().clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = hub_market_item_acall as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = hub_market_get_name as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = hub_market_get_help as _);
        config_menu.full_menu_item_list.insert(len - 3, new_menu_item);
    }
    config_menu.reserved_show_row_num += 1;
    let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
    let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
    new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = super::collector::collector_acall as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = super::collector::collector_getname as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = super::collector::collector_help as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = super::collector::collector_buildattribute as _);
    config_menu.full_menu_item_list.insert(0, new_menu_item);

}
fn hub_quick_menu_get_name(_menu: &mut BasicMenuItem,_method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_MENU_TITLE_HUB")}
fn hub_market_get_name(_menu: &mut BasicMenuItem,_method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_MENU_SHOPPING")}
fn hub_market_get_help(_menu: &mut BasicMenuItem,_method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_MENU_SHOPPING_HELP")}
fn hub_fast_travel_get_help(_menu: &mut BasicMenuItem,_method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_MENU_HUB_MAP_CHECK_HELP") }

pub extern "C" fn hub_quick_menu_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
        let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
        close_anime_all(this.menu, method.method_info);
    });
    let sf = Il2CppClass::from_name("App", "HubMenu").unwrap().get_static_fields::<HubMenuStaticFields>();

    this.menu.save_select(sf.selected);
    //basic_menu_save_select(this.menu, sf.selected, None); 
    unsafe { force_rebuild_layout(this, None); }
    hub_quick_menu_create_bind(this);
    BasicMenuResult::se_decide()
}
pub extern "C" fn hub_market_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
        let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
        close_anime_all(this.menu, method.method_info);
    });
    let sf = Il2CppClass::from_name("App", "HubMenu").unwrap().get_static_fields::<HubMenuStaticFields>();

    this.menu.save_select(sf.selected);
    unsafe {  force_rebuild_layout(this, None); }
    hub_market_create_bind(this);
    //BasicMenuResult::new().with_close_this(true).with_se_decide(true)
    BasicMenuResult::se_decide()
}

fn hub_quick_menu_item_bcall(item: &mut BasicMenuItem,_method_info: OptionalMethod) -> BasicMenuResult {
    // restore_instructions();
    hub_menu_create_bind( item.menu.proc.parent.as_mut().unwrap(), None);
    BasicMenuResult::close_cancel()
}

fn hub_quick_menu_create_bind(menu: &mut BasicMenuItem){
    let list = menu.menu.full_menu_item_list.get_class();
    let new_list = il2cpp::instantiate_class::<List<BasicMenuItem>>(list).unwrap();
    new_list.items = Il2CppArray::new(10).unwrap();
    let class2 = Il2CppClass::from_name("App", "HubMenu").unwrap().get_nested_types().iter().find(|x| x.get_name() == "FriendItem").unwrap();
    new_list.add(create_menu_item_impl(class2, noticeboard::NoticeBoardQuickMenuItem));   //Bulletin Board
    new_list.add(create_menu_item_impl(class2, arena::ArenaQuickMenuItem));    // Arena
    new_list.add(create_menu_item_impl(class2, cafe::CookQuickMenuItem));    // Cook
    new_list.add(create_menu_item_impl(class2, well::WellQuickMenuItem)); // Well
    new_list.add(create_menu_item_impl(class2, animal::AnimalMenuItem));
    new_list.add(create_menu_item_impl(class2, mascot::MascotQuickMenuItem));
    new_list.add(create_menu_item_impl(class2, muscle::MuscleMenuItem));
    new_list.add(create_menu_item_impl(class2, hubshops::RefreshMenuItem));
    new_list.add(create_menu_item_impl(class2, fishing::FishingMenuItem));
    new_list.add(create_menu_item_impl(class2, dragonride::DragonRideMenuItem));
    new_list[2].get_class_mut().get_virtual_method_mut("XCall").map(|method| method.method_ptr = cafe::chef::chef_y_call as _ );

    new_list.items.iter_mut().for_each(|item|{
         item.get_class_mut().get_virtual_method_mut("BCall").map(|method| method.method_ptr = hub_quick_menu_item_bcall as _ ); }
    );

    new_list[3].get_class_mut().get_virtual_method_mut("XCall").map(|method| method.method_ptr = well::well_x_call as _ ).unwrap();   // Reseed Well
    new_list[3].get_class_mut().get_virtual_method_mut("PlusCall").map(|method| method.method_ptr = well::well_plus_call as _ ).unwrap(); // Item Preview

    let content = unsafe { create_hub_menu_content(None) };
    let new_menu = BasicMenu::new(new_list, content);
    let descs = new_menu.create_default_desc();
    new_menu.bind_parent_menu();
    // println!("Hub Quick Menu Create Bind to: {}", menu.menu.proc.parent.as_ref().unwrap().get_class().get_name());
    new_menu.create_bind(menu.menu.proc.parent.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>(), descs, "HubQuickMenu");
    // new_menu.set_transform_as_sub_menu(menu.menu, menu); 
    new_menu.set_show_row_num(new_list.len() as i32);
}

fn hub_market_create_bind(menu: &mut BasicMenuItem){
    let list = menu.menu.full_menu_item_list.get_class();
    let new_list = il2cpp::instantiate_class::<List<BasicMenuItem>>(list).unwrap();
    new_list.items = Il2CppArray::new(if GameUserData::get_sequence() == 4 { 7 } else { 6 } ).unwrap();
    let class2 = Il2CppClass::from_name("App", "HubMenu").unwrap().get_nested_types().iter().find(|x| x.get_name() == "FriendItem").unwrap();

    new_list.add(create_menu_item_impl(class2, hubshops::ItemShopQuickMenuItem));   // ItemShop
    new_list.add(create_menu_item_impl(class2, hubshops::WeaponQuickMenuItem));    // Armory
    new_list.add(create_menu_item_impl(class2, hubshops::RefineQuickMenuItem));    // Smithy

    new_list.add(create_menu_item_impl(class2, hubshops::AccessoryQuickMenuItem)); // Accessory
    new_list.add(create_menu_item_impl(class2, hubshops::FleaMarketMenuItem));
    new_list.add(create_menu_item_impl(class2, godroom::GodRoomQuickMenuItem));
    if GameUserData::get_sequence() == 4 { new_list.add(create_menu_item_impl(class2, hubshops::JukeBoxMenuItem)); }


    new_list.items.iter_mut().for_each(|item| {
        item.get_class_mut().get_virtual_method_mut("BCall").map(|method| method.method_ptr = hub_quick_menu_item_bcall as _ ); }
    );

    let content = unsafe { create_hub_menu_content(None) };
    let new_menu = BasicMenu::new(new_list, content);
    let descs = new_menu.create_default_desc();
    new_menu.bind_parent_menu();
    // println!("Hub Quick Menu Create Bind to: {}", menu.menu.proc.parent.as_ref().unwrap().get_class().get_name());
    new_menu.create_bind(menu.menu.proc.parent.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>(), descs, "HubMarketMenu");
    // new_menu.set_transform_as_sub_menu(menu.menu, menu); 
    new_menu.set_show_row_num(new_list.len() as i32);
}


#[skyline::from_offset(0x028b78c0)]
pub fn create_hub_menu_content(method_info: OptionalMethod) -> &'static BasicMenuContent;

#[skyline::from_offset(0x028b6ef0)]
pub fn hub_menu_create(menu: &HubSequence,_method_info: OptionalMethod);

#[skyline::from_offset(0x0245d070)]
pub fn basic_menu_save_select(this: &BasicMenu<BasicMenuItem>, sel: u64,_method_info: OptionalMethod);

#[skyline::from_offset(0x02466340)]
pub fn force_rebuild_layout(this: &BasicMenuItem,_method_info: OptionalMethod);

