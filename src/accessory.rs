use unity::{
    prelude::*,
    system::List,
    il2cpp::*,
};
use engage::{
    gameuserdata::GameUserData,
    menu::*,
    sortie::*,
    random::Random,
    mess::Mess,
    proc::Bindable,
    gamedata::{*, unit::*, accessory::*},

};

fn change_accessory(unit: &Unit, kind: i32, increase: bool) -> bool {
    let accessory = unsafe { unit_get_accessory_list(unit, None)};
    let index = accessory.unit_accessory_array[kind as usize].index;
    let accessories = AccessoryData::get_list().unwrap();
    if increase {
        if let Some(new_index) = accessories.iter()
            .filter(|acc| acc.get_num() > 0 && acc.can_equip(unit) && acc.kind == kind && acc.parent.index > index )
            .map(|acc| acc.parent.index).min() 
        {
            accessory.unit_accessory_array[kind as usize].index = new_index;
            return true;
        }
        else if index != 0 {
            accessory.unit_accessory_array[kind as usize].index = 0;
            return true;
        }
    }
    else if index == 0 {
        if let Some(new_index) = accessories.iter()
            .filter(|acc| acc.get_num() > 0 && acc.can_equip(unit) && acc.kind == kind && acc.parent.index > index )
            .map(|acc| acc.parent.index).max() 
            {
                accessory.unit_accessory_array[kind as usize].index = new_index;
                return true;
            }
    }
    else {
        if let Some(new_index) = accessories.iter()
            .filter(|acc| acc.get_num() > 0 && acc.can_equip(unit) && acc.kind == kind && acc.parent.index < index )
            .map(|acc| acc.parent.index).max() {
                accessory.unit_accessory_array[kind as usize].index = new_index;
                return true;
            }
        else if index != 0 {
            accessory.unit_accessory_array[kind as usize].index = 0;
            return true;
        }
    }
    return false;
}

pub fn reload_unit_info(unit: &Unit) -> i32 {
    unsafe {
        help_set_unit(0, None, false, false, false, None, None);
        help_set_unit(1, None, false, false, false, None, None);
        help_set_unit(0, Some(unit), false, false, false, None, None);
    }
    return 0x80;
}

fn unit_accessory_sub_menu_create_bind(menu: &mut BasicMenuItem){
    let list = menu.menu.full_menu_item_list.get_class();
    let new_list = il2cpp::instantiate_class::<List<BasicMenuItem>>(list).unwrap();
    let count;
    if unsafe { accessory_count(0, None) >= 6 } {
        new_list.items = Il2CppArray::new(10).unwrap();
        count = 7;
    }
    else {
        new_list.items = Il2CppArray::new(4).unwrap();
        count = 4;
    }
    let is_sortie = GameUserData::get_sequence() != 3;
    let class = get_base_menu_item_class();
    for _x in 0..count {
        let cock = class.clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = unit_access_sub_menu_name as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("LCall").map(|method| method.method_ptr = unit_access_sub_menu_l_call as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("RCall").map(|method| method.method_ptr = unit_access_sub_menu_r_call as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("MinusCall").map(|method| method.method_ptr = unit_access_sub_menu_minus_call as _);
        if !is_sortie {
            new_menu_item.get_class_mut().get_virtual_method_mut("GetCommandHelp").map(|method| method.method_ptr = unit_access_map_command_help as _);
        }
        new_list.add(new_menu_item);
    }
    let content = if is_sortie{ unsafe { create_basic_menu_content(None) } } else { unsafe { map_command_menu_conent(None)}}; 
    let new_menu = BasicMenu::new(new_list, content);
    let descs = new_menu.create_default_desc();
    new_menu.bind_parent_menu();
    new_menu.create_bind(menu.menu, descs, "");
    if is_sortie { new_menu.set_transform_as_sub_menu(menu.menu, menu);  }
    new_menu.set_show_row_num(count);
}

fn get_base_menu_item_class() -> &'static mut Il2CppClass {
    let menu = if GameUserData::get_sequence() != 3 {
        Il2CppClass::from_name("App", "UnitSelectSubMenu").unwrap().get_nested_types().iter().find(|x| x.get_name() == "BaseMenuItem")
    }
    else {
        Il2CppClass::from_name("App", "MapUnitCommandMenu").unwrap().get_nested_types().iter().find(|x| x.get_name() == "ItemMenuItem")
    }.unwrap();
    Il2CppClass::from_il2cpptype(menu.get_type()).unwrap()

}
pub fn install_accessory_sub_menu() {
    if let Some(cc) = Il2CppClass::from_name("App", "SortieUnitSelect").unwrap().get_nested_types().iter().find(|x| x.get_name() == "UnitMenuItem") {
        let menu_mut = Il2CppClass::from_il2cpptype(cc.get_type()).unwrap();
        menu_mut.get_virtual_method_mut("YCall").map(|method| method.method_ptr = y_call as _);
    }

    if let Some(cc) = Il2CppClass::from_name("App", "MapUnitCommandMenu").unwrap().get_nested_types().iter().find(|x| x.get_name() == "ItemMenuItem") {
        let menu_mut = Il2CppClass::from_il2cpptype(cc.get_type()).unwrap();
        menu_mut.get_virtual_method_mut("YCall").map(|method| method.method_ptr = y_call as _);
    }

}


pub fn y_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    unit_accessory_sub_menu_create_bind(this);
    return 0x80;
}

pub fn unit_access_sub_menu_name(this: &BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString {
    let accessory_index = if this.index < 4 { this.index }
        else { this.index + 1 };

    let unit = if GameUserData::get_sequence() != 3 { SortieSelectionUnitManager::get_unit() } 
        else { engage::mapmind::MapMind::get_unit() };
    let slot = &unit.accessory_list.unit_accessory_array[accessory_index as usize];
    if slot.index == 0 { return "--------".into(); }
    if let Some(acc) = AccessoryData::try_index_get(slot.index) { 
        return Mess::get(acc.name);
    }
    else { return "--------".into(); }
}

pub fn unit_access_sub_menu_r_call(this: &BasicMenuItem, _method_info: OptionalMethod) -> i32{
    let kind = if this.index < 4 { this.index }
        else { this.index + 1 };

    let unit = if GameUserData::get_sequence() != 3 { SortieSelectionUnitManager::get_unit() } 
    else { engage::mapmind::MapMind::get_unit() };
    if change_accessory(unit, kind, true) {
        this.rebuild_text();
        return reload_unit_info(unit);
    }
    else { return 0; }
}

pub fn unit_access_sub_menu_l_call(this: &BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    let kind = if this.index < 4 { this.index }
        else { this.index + 1 };
    let unit = if GameUserData::get_sequence() != 3 { SortieSelectionUnitManager::get_unit() } 
    else { engage::mapmind::MapMind::get_unit() };
    if change_accessory(unit, kind, false) {
        this.rebuild_text();
        return reload_unit_info(unit);
    }
    else { return 0; }

}

pub fn unit_access_sub_menu_minus_call(this: &BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    let unit = if GameUserData::get_sequence() != 3 { SortieSelectionUnitManager::get_unit() } 
    else { engage::mapmind::MapMind::get_unit() };
    let accessory_list = &mut unsafe { unit_get_accessory_list(unit, None) }.unit_accessory_array;
    let kind = if this.index < 4 { this.index }
        else { this.index + 1 };
    if accessory_list[kind as usize].index != 0 {
        accessory_list[kind as usize].index = 0;
        this.rebuild_text();
        return reload_unit_info(unit);
    }
    return 0;
}
pub fn unit_access_map_command_help(this: &BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { "".into() }


#[skyline::from_offset(0x01a4dff0)]
fn unit_get_accessory_list(this: &Unit, method_info: OptionalMethod) -> &'static mut UnitAccessoryList;

#[skyline::from_offset(0x01f86a50)]
fn help_set_unit(side: i32, unit: Option<&Unit>, relax: bool, reverse_rotation: bool, is_delay_load: bool, action: OptionalMethod, method_info: OptionalMethod);

#[skyline::from_offset(0x01f61b10)]
pub fn accessory_count(lol: u64, method_info: OptionalMethod) -> i32; 

#[skyline::from_offset(0x024622f0)]
fn create_basic_menu_content(method_info: OptionalMethod) -> &'static BasicMenuContent; 

#[skyline::from_offset(0x0202b7a0)]
fn map_command_menu_conent(method_info: OptionalMethod) -> &'static BasicMenuContent; 