use engage::menu::*;
use super::*;
use crate::hub::quickmenu::*;

pub fn gmap_shop_new_a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    unsafe { gmap_shop_a_call(this, None) };   
    gmap_shop_menu_create_bind(this);
    BasicMenuResult::se_decide()
}

fn gmap_shop_menu_create_bind(menu: &mut BasicMenuItem){
    let list = menu.menu.full_menu_item_list.get_class();
    let new_list = il2cpp::instantiate_class::<List<BasicMenuItem>>(list).unwrap();
    new_list.items = Il2CppArray::new(6).unwrap();
    let gmap_menu_class = Il2CppClass::from_name("App", "GmapMenuSequence").unwrap()
        .get_nested_types().iter().find(|x| x.get_name() == "GmapMenu").unwrap()
        .get_nested_types().iter().find(|x| x.get_name() == "SubShopMenu").unwrap();
    let item_class = gmap_menu_class.get_nested_types().iter().find(|x| x.get_name() == "ItemShopMenuItem").unwrap();
    let item = il2cpp::instantiate_class::<BasicMenuItem>(item_class).unwrap();
    
    let weapon = il2cpp::instantiate_class::<BasicMenuItem>(gmap_menu_class.get_nested_types().iter().find(|x| x.get_name() == "WeaponShopMenuItem").unwrap()).unwrap();
    new_list.add(create_menu_item_impl(item_class, noticeboard::NoticeBoardQuickMenuItem));   //Bulletin Board
    new_list.add(item);
    new_list.add(weapon);
    new_list.add(create_menu_item_impl(item_class, hubshops::RefineQuickMenuItem)); 
    new_list.add(create_menu_item_impl(item_class, arena::ArenaQuickMenuItem));    // Arena   // Armory
    new_list.add(create_menu_item_impl(item_class, well::WellQuickMenuItem));
    new_list.add(create_menu_item_impl(item_class, godroom::GodRoomQuickMenuItem));
    new_list[5].get_class_mut().get_virtual_method_mut("XCall").map(|method| method.method_ptr = well::well_x_call as _ ).unwrap();   // Reseed Well
    new_list[5].get_class_mut().get_virtual_method_mut("PlusCall").map(|method| method.method_ptr =  well::well_plus_call as _ ).unwrap(); // Item Preview

    new_list.add(create_menu_item_impl(item_class, cafe::CookQuickMenuItem));
    let content = unsafe { crate::sortie::accessory::create_basic_menu_content(None) };
    let new_menu = BasicMenu::new(new_list, content);
    let descs = new_menu.create_default_desc();
    new_menu.bind_parent_menu();
    new_menu.create_bind(menu.menu, descs, "");
    new_menu.set_transform_as_sub_menu(menu.menu, menu); 
    new_menu.set_show_row_num(8);
}

pub fn gmapmenu_shop_acall_install() {    
    let gmap_menu_class = Il2CppClass::from_name("App", "GmapMenuSequence").unwrap()
        .get_nested_types().iter().find(|x| x.get_name() == "GmapMenu").unwrap()
        .get_nested_types().iter().find(|x| x.get_name() == "ShopItem").unwrap();
    let menu_mut = Il2CppClass::from_il2cpptype(gmap_menu_class.get_type()).unwrap();
    menu_mut.get_virtual_method_mut("ACall").map(|method| method.method_ptr = gmap_shop_new_a_call as _);
}

#[skyline::from_offset(0x01b3aab0)]
fn get_chapter_type(method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01cd07a0)]
fn gmap_dispos_create_bind(this: &BasicMenu<BasicMenuItem>, method_info: OptionalMethod);

#[skyline::from_offset(0x01b3d1b0)]
pub fn gmap_shop_a_call(this: &mut BasicMenuItem, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x0252d880)]
pub fn gmap_menu_content(method_info: OptionalMethod) -> &'static BasicMenuContent;

#[skyline::from_offset(0x024eb420)]
pub fn sortie_menu_content(method_info: OptionalMethod) -> &'static BasicMenuContent;

