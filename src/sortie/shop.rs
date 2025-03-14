use unity::prelude::*;
use engage::menu::*;
use crate::hub::quickmenu;
use crate::utils::create_menu_item_impl;

pub fn sortie_shop_a_call(item: &mut BasicMenuItem, method_info: OptionalMethod) -> i32 {
    let original_result = unsafe { original_sortie_shop_a_call(item, method_info) };
    if original_result == 0x800 { return 0x800; }

    let base_menu_class = Il2CppClass::from_name("App", "SortieTopMenuShopSubMenu").unwrap()
        .get_nested_types().iter().find(|x| x.get_name() == "ItemShopMenuItem").unwrap();

    let config_menu = item.menu.proc.child.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>();
    config_menu.full_menu_item_list.add(create_menu_item_impl(base_menu_class, quickmenu::noticeboard::NoticeBoardQuickMenuItem));
    config_menu.full_menu_item_list.add(create_menu_item_impl(base_menu_class, quickmenu::arena::ArenaQuickMenuItem)); 
    // Hide Cobalt's Skill Inheritance Menu
    config_menu.full_menu_item_list[3].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = hide_inherit_menu_item as _ ).unwrap();  
    /*
    let content = unsafe { sortie_menu_content(None) };
    let new_menu = BasicMenu::new(new_list, content);
    let descs = new_menu.create_default_desc();
    new_menu.bind_parent_menu();
    new_menu.create_bind(menu.menu, descs, "TriabolicalSortieMenu");
    new_menu.set_transform_as_sub_menu(menu.menu, menu); 
    */
    config_menu.set_show_row_num(5);
    return 0x80;
}

fn hide_inherit_menu_item(_item: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute { BasicMenuItemAttribute::Hide }

#[skyline::from_offset(0x01d78b90)]
fn original_sortie_shop_a_call(item: &mut BasicMenuItem, method_info: OptionalMethod) -> i32;
