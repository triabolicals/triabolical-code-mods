use super::*;
use engage::unitpool::UnitPool;

fn cook_submenu(menu: &mut BasicMenuItem){
    let list = menu.menu.full_menu_item_list.get_class();
    let new_list = il2cpp::instantiate_class::<List<BasicMenuItem>>(list).unwrap();
    let count = 1;
    new_list.items = Il2CppArray::new(1).unwrap();

    let class = crate::sortie::accessory::get_base_menu_item_class();
    for _x in 0..count {
        let cock = class.clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = chef_name as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("LCall").map(|method| method.method_ptr = chef_sub_l_call as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("RCall").map(|method| method.method_ptr = chef_sub_r_call as _);
        new_list.add(new_menu_item);
    }
    let content = unsafe { sortie_sub_menu_content(None) };
    let new_menu = BasicMenu::new(new_list, content);
    let descs = new_menu.create_default_desc();
    new_menu.bind_parent_menu();
    new_menu.create_bind(menu.menu, descs, "");
    new_menu.set_transform_as_sub_menu(menu.menu, menu);
    new_menu.set_show_row_num(count);
}


pub fn chef_y_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    cook_submenu(this);
    return 0x80;
}
#[skyline::from_offset(0x024eb420)]
fn sortie_sub_menu_content(method_info: OptionalMethod) -> &'static BasicMenuContent;

fn chef_sub_l_call(this: &mut BasicMenuItem) -> BasicMenuResult {
    if let Some(cook_pid) = HubUtil::get_current_cooking_pid() {
        let cook_data_list = CookData::get_list().unwrap();
        let mut pos = cook_data_list.iter().position(|cook| cook.pid == cook_pid).unwrap();
        loop {
            if pos == 0 { pos = cook_data_list.len() - 1; }
            else { pos -= 1;  }
            let pid = cook_data_list[pos].pid;
            if pid == cook_pid { return BasicMenuResult::se_miss(); }
            if let Some(unit) = UnitPool::get_from_person_mut(cook_data_list[pos].pid, false) {
                if unit.force.is_some_and(|f| f.force_type == 0 || f.force_type == 3) {
                    HubUtil::set_cooking_pid(pid);
                    this.rebuild_text();
                    return BasicMenuResult::se_cursor();
                }
            }
        }
    }
    BasicMenuResult::new()
}
fn chef_sub_r_call(this: &mut BasicMenuItem) -> BasicMenuResult {
    if let Some(cook_pid) = HubUtil::get_current_cooking_pid() {
        let cook_data_list = CookData::get_list().unwrap();
        let mut pos = cook_data_list.iter().position(|cook| cook.pid == cook_pid).unwrap();
        loop {
            if pos == cook_data_list.len() - 1 { pos = 0 }
            else { pos += 1;  }
            let pid = cook_data_list[pos].pid;
            if pid == cook_pid { return BasicMenuResult::se_miss(); }
            if let Some(unit) = UnitPool::get_from_person_mut(cook_data_list[pos].pid, false) {
                if unit.force.is_some_and(|f| f.force_type == 0 || f.force_type == 3) {
                    HubUtil::set_cooking_pid(pid);
                    this.rebuild_text();
                    return BasicMenuResult::se_cursor();
                }
            }
        }
    }
    BasicMenuResult::new()
}

fn chef_name(_this: &mut BasicMenuItem) -> &'static Il2CppString {
    if let Some(cook_pid) = HubUtil::get_current_cooking_pid() {
        format!("{}: {}", Mess::get("MID_Hub_CafeTerrace_Cook_Chef"), Mess::get_name(cook_pid)).into()
    }
    else {
        Mess::get("MIID_H_None")
    }
}