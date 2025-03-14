pub mod accessory;
pub mod shop;

pub fn sortie_menu_installs() {
    crate::utils::get_nested_virtual_methods_mut("App", "SortieTopMenu", "ShopMenuItem", "ACall")
        .map(|method| method.method_ptr = shop::sortie_shop_a_call as _);

    crate::utils::get_nested_virtual_methods_mut("App", "SortieUnitSelect", "UnitMenuItem", "YCall")
        .map(|method| method.method_ptr = accessory::unit_menu_item_y_call as _);

    crate::utils::get_nested_virtual_methods_mut("App", "MapUnitCommandMenu", "ItemMenuItem", "XCall")
        .map(|method| method.method_ptr = accessory::item_menu_item_x_call as _);

    println!("Sortie Menus installed");
}