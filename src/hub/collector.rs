use super::*;
use engage::{
    sequence::commonrewardsequence::CommonRewardSequence,
    gamedata::{animal::AnimalData, item::ItemData},
};
use crate::utils::is_null_empty;

pub extern "C" fn collector_acall(this: &mut BasicMenuItem) -> BasicMenuResult {
    if this.is_attribute_disable() { return BasicMenuResult::se_miss(); } 
    if GameUserData::get_sequence() == 4 || GameUserData::get_sequence() == 5 {
        let hub = HubSequence::get_instance();
        let mini_map = hub.get_mini_map();
            if mini_map.is_show() { mini_map.set_mode(0); }
            mini_map.hide_system_menu();
    }
    this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
        let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
        close_anime_all(this.menu, method.method_info);
    });
    let hub_sequence = HubSequence::get_instance();
    let access_list = &mut hub_sequence.get_locator_group().unwrap().access_list;
    
    let it =  crate::utils::get_list_item_class();
    let test = il2cpp::instantiate_class::<List<ItemData>>(it);
    if test.is_err() { return BasicMenuResult::se_miss(); }
    else if it.get_methods().iter().find(|method| method.get_name() == Some(String::from("Add"))).is_none() {return BasicMenuResult::se_miss(); }
    let item_list = test.unwrap();
    let mut animal_count = 0;
    item_list.items = Il2CppArray::new(750).unwrap();
    access_list.iter_mut().for_each(|access_point|{
        if let Some(access) = &access_point.access_data {
            if !access.get_is_done() {
                if try_capture_animal(access) {  animal_count += 1;  }
                else if let Some(iid) = access.aid {
                    if let Some(w_item) =  ItemData::get(iid) {
                        if unsafe { !is_null_empty(w_item.name, None) && !is_null_empty(w_item.help, None) } {
                            let count = HubUtil::get_item_count_with_bonus(w_item, access.get_item_count() );
                            for _y in 0..count { if item_list.len() < 750 {  if let Some(valid_item) = ItemData::get_mut(iid) { item_list.add( valid_item ); }}}
                            if access_point.item_effect != 0 { unsafe { set_game_object_active(access_point.item_effect, false, None); } }
                        }
                    }
                }
                if item_list.len() < 750 { 
                    if let Some(item) = ItemData::get_mut(access.talk_item.unwrap_or_else(|| "none".into())){
                        item_list.add( item );
                    }
                }
                access.done();
                access_point.done();
            }
        }
    });
    if GameVariableManager::get_number("G_Continuous") > 0 && GameVariableManager::get_number("G_Continuous") < 4  {
        for x in 0..5 {
            if item_list.len() < 750 {
                if let Some(iid) = GameVariableManager::try_get_string( HubUtil::get_animal_item_flag(x) ) {
                    if let Some(item) = ItemData::get_mut(iid) { item_list.add( item); }
                }
            }
        }
    }
    let title = if GameUserData::get_sequence() == 4 { "MID_MENU_TITLE_HUB" } else { "MID_MENU_TITLE_KIZUNA" }.into();
    CommonRewardSequence::create_bind_for_well(this.menu, item_list, title);
    BasicMenuResult::new().with_close_this(true).with_se_decide(true)
}

pub extern "C" fn collector_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Area_ItemReturn") }
pub extern "C" fn collector_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if get_item_count() != 0 { BasicMenuItemAttribute::Enable } else { BasicMenuItemAttribute::Hide }
}
pub extern "C" fn collector_help(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Well_ItemExchange_Get") }


fn can_adopt(access: &HubAccessData) -> bool { 
    if GameUserData::get_sequence() != 5  {return false; }  //If not Exploration
    if !HubFacilityData::get("AID_動物小屋").unwrap().is_complete() { return false; }   // If farm isn't unlocked
    if !access.is_animal() { return false; }
    if access.get_is_done() { return false; }

    let animal_pid = access.try_get_pid();
    if animal_pid.is_none() { return false; }

    let animal = AnimalData::get_by_pid(animal_pid.unwrap());
    if animal.is_none() { return false; }

    if animal.unwrap().can_capture() { return true; }
    return false;
}

fn try_capture_animal(access: &HubAccessData) -> bool {
    if can_adopt(access) && access.try_get_pid().is_some() {
        let animal = AnimalData::get_by_pid(access.try_get_pid().unwrap()).unwrap();
        animal.increment_capture();
        return access.done();
    }
    return false;
}

fn get_item_count() -> i32 {
    let mut result = 0;
    let hub_sequence = HubSequence::get_instance();
    let access_manager = hub_sequence.get_current_access_data();
    let access_list = access_manager.access_list;
    let it =  crate::utils::get_list_item_class();
    let test = il2cpp::instantiate_class::<List<ItemData>>(it);
    if test.is_err() { return 0; }
    else if it.get_methods().iter().find(|method| method.get_name() == Some(String::from("Add"))).is_none() { return 0; }
    for x in 0..access_list.len() {
        if access_list[x].get_is_done() { continue; }
        if can_adopt(access_list[x]){ result += 1; }
        if let Some(iid) = access_list[x].aid {
            if let Some(item) = ItemData::get(iid) {
                if unsafe { is_null_empty(item.name, None) || is_null_empty(item.help, None) } { continue; }
                else { result += 1; }
            }
        }
    }
    result
}

#[skyline::from_offset(0x02c4ea10)]
pub fn set_game_object_active(this: u64, value: bool, method_info: OptionalMethod);