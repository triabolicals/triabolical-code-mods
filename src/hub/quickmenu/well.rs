use super::*;
use engage::{gamedata::item::ItemData, random::Random, sequence::wellsequence::WellSequence};

pub struct WellQuickMenuItem; 
impl BasicMenuItemMethods for WellQuickMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() {
            close_hub_mini_map();
            TitleBar::hide_footer();
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            let proc = WellSequence::new();
            proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _).unwrap();
            let descs = proc.create_desc();
            if !can_well() {
                [4, 7, 8, 9, 10, 11, 12, 48].into_iter().for_each(|x| descs[x as usize] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing)));
            }
            proc.create_bind(this.menu, descs, "WellSequence");
            BasicMenuResult::se_decide()
        } else { BasicMenuResult::se_miss() }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Well") }
    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString {
        match WellSequence::get_use_flag() {
            0 => { Mess::get("MID_Hub_Well_Nothing2") }
            1 => { Mess::get("MID_Hub_Well_ItemExchange_Message")}
            _ => { Mess::get("MID_Hub_Area_ItemReturn") }
        }
    }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_不思議な井戸").unwrap().is_complete() { BasicMenuItemAttribute::Enable } else { BasicMenuItemAttribute::Hide }
    }
}

pub fn well_x_call(item: &BasicMenuItem,_method_info: OptionalMethod) -> BasicMenuResult {
    if WellSequence::get_use_flag() > 0 {
        let seed = engage::random::Random::get_system().value();
        WellSequence::set_seed(seed);
        engage::gamemessage::GameMessage::create_key_wait(item.menu, Mess::get("MID_Hub_Well_ItemExchange_Message").to_string());
        BasicMenuResult::se_decide()
    }
    else { BasicMenuResult::se_miss() }
}
pub fn well_plus_call(item: &BasicMenuItem,_method_info: OptionalMethod) -> BasicMenuResult {
    if WellSequence::get_use_flag() > 0 {
        let seed = WellSequence::get_seed();
        let rng = Random::instantiate().unwrap();
        rng.ctor(seed as u32);
        let level = WellSequence::get_exchange_level();
        let items = create_item_str( WellSequence::calc_item_exchange(level, rng) );
        engage::gamemessage::GameMessage::create_key_wait(item.menu, items);

        BasicMenuResult::se_decide()
    }
    else { BasicMenuResult::se_miss() }
}

fn create_item_str(item: &List<ItemData>) -> String {
    let mut new_str = String::new();
    let mut count: Vec<(i32, i32)> = Vec::new();

    item.iter().for_each(|item|{
        if let Some(found) = count.iter_mut().find(|x| x.0 == item.parent.hash) { found.1 += 1; }
        else { count.push( (item.parent.hash, 1));}
    });
    let mut str_count = 0;
    count.iter().for_each(|x|{
        if str_count > 0 {
            if str_count % 3 == 0 { new_str = format!("{}\n{}", new_str, Mess::get(ItemData::try_get_hash(x.0).unwrap().name)); }
            else {  new_str = format!("{}, {}", new_str, Mess::get(ItemData::try_get_hash(x.0).unwrap().name));  }
        }
        else {
            new_str = format!("{}", Mess::get(ItemData::try_get_hash(x.0).unwrap().name));
        }
        str_count +=1;
    });
    new_str
}

pub fn well_evil_build_attr_change() {
    if let Some(cc) = Il2CppClass::from_name("App", "WellTopMenu").unwrap().get_nested_types().iter()
        .find(|x| x.get_name() == "EvilMapStartMenuItem" || x.get_name() == "EvilMapChangeDifficultyMenuItem" ) {
        let menu_mut = Il2CppClass::from_il2cpptype(cc.get_type()).unwrap();
        menu_mut.get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = well_evil_menu_item_build_attr as _);
    }
}

fn well_evil_menu_item_build_attr(_this: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if can_well() {
        if crate::utils::has_dlc() || GameVariableManager::get_bool("G_Cleared_E001")  { BasicMenuItemAttribute::Enable }
        else { BasicMenuItemAttribute::Disable }
    }
    else { BasicMenuItemAttribute::Hide  }
}
