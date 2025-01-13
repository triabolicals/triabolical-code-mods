use skyline::patching::Patch;
use unity::{
    prelude::*,
    system::List,
};
use crate::{arena::ArenaOrderSequence, hub::*};
use engage::{
    sequence::hubrefineshopsequence::*,
    menu::{
        BasicMenuResult,
        config::ConfigBasicMenuItem,
        BasicMenu, BasicMenuItem,
    },
    mess::*,
    random::Random,
    proc::{ProcInstFields,Bindable,desc::ProcDesc, ProcInst},
    hub::{*, access::*, hubsequence::*},
    gamedata::{*,item::ItemData},
    gamevariable::*,
    gameuserdata::*,
};
pub fn restore_instructions(){
    //God Room for cleaning
    let _ = Patch::in_text(0x01cd99a0).bytes(&[0x20, 0, 0x80, 0x52]);
    let _ = Patch::in_text(0x01cd97e0).bytes(&[0xfd, 0x7b, 0xbe, 0xa9]);
    let _ = Patch::in_text(0x01cd97e4).bytes(&[0xf4, 0x4f, 0x01, 0xa9]);
    // Well FX menu item
    let _ = Patch::in_text(0x01b2d6d0).bytes(&[0xff, 0x03, 0x01, 0xd1]);
    let _ = Patch::in_text(0x01b2d6d4).bytes(&[0xfd, 0x7b, 0x01, 0xa9]);
}
#[skyline::from_offset(0x01f1a990)]
pub fn return_to_title(this: &mut ProcInst, _method_info: OptionalMethod);

extern "C" fn open_anime_all_ondispose(this: &mut ProcInst, _method_info: OptionalMethod) {
    //TitleBar::close_header();
   // TitleBar::show_header();
    restore_instructions();
    this.parent.as_ref().unwrap().get_class().get_virtual_method("OpenAnimeAll").map(|method| {
        let open_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&ProcInst, &MethodInfo)>(method.method_info.method_ptr) };
        open_anime_all(this.parent.as_ref().unwrap(), method.method_info);
    });

    // Restoring Bullentin Board original items and the title bar when going back to the Bullentin Board
    if this.parent.as_ref().unwrap().klass.get_name() == "NoticeBoardTopMenu" {
        unsafe { return_to_title(this, None); }
        if this.klass.get_name() == "NoticeBoardSequence" {
            let config_menu = this.parent.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>();
            config_menu.full_menu_item_list.items[0].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = notice_item_acall as _);
            config_menu.full_menu_item_list.items[0].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = notice_item_getname as _);
        
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = weapon_shop_acall as _);
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = weapon_menu_item_getname as _);
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = weapon_menu_item_buildattribute as _);
        
            config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = item_shop_acall as _);
            config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = item_menu_item_getname as _);
            config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
            config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = item_menu_item_buildattribute as _);
        }
    }
}
#[unity::class("App", "GmapMapInfoContent")]
pub struct GmapMapInfoContent {}

#[skyline::from_offset(0x0252a640)]
pub fn mapinfo_close(this: &GmapMapInfoContent, method_info: OptionalMethod); 

#[unity::class("App", "GmapSequence")]
pub struct GmapSequence {
    /* need to properly add padding if using other fields
    pub proc: ProcInstFields,
    pub scenename: &'static Il2CppString,
    pub scenemode: i32,
    junk: [u64; 8],
    */
    junk: [u8; 0xb8],
    pub map_info: &'static GmapMapInfoContent,
}

pub fn get_gmapsequence() -> &'static GmapSequence {
    let mut method = GmapSequence::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("get_Instance")));
    if method.is_none() { method = GmapSequence::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("get_Instance"))); }
    let get_instance = unsafe { std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> &'static GmapSequence>( method.unwrap().method_ptr, ) };
    get_instance(method.unwrap())
}
#[unity::class("App", "WellSequence")]
pub struct WellSequence {
    proc: ProcInstFields,
}
impl Bindable for WellSequence {}
impl WellSequence {
    pub fn new() -> &'static mut Self {
        let item = Self::instantiate().unwrap();
        item
    }
    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> { unsafe { well_create_desc(self, None) } }
}

#[unity::class("App", "HubItemShopSequence")]
pub struct HubItemShopSequence {}
impl Bindable for HubItemShopSequence {}
impl HubItemShopSequence {
    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> { unsafe { item_shop_create_desc(self, None) } }
}

#[unity::class("App", "HubWeaponShopSequence")]
pub struct HubWeaponShopSequence {}
impl Bindable for HubWeaponShopSequence {}
impl HubWeaponShopSequence {
    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> { unsafe { weapon_shop_create_desc(self, None) } }
}

#[unity::class("App","GodRoomPedestalSequence")]
pub struct GodRoomPedestalSequence {}
impl Bindable for GodRoomPedestalSequence {}
impl GodRoomPedestalSequence {
    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> { unsafe { god_room_create_desc(self, None) } }
}

#[unity::class("App","NoticeBoardSequence")]
pub struct NoticeBoardSequence {}
impl Bindable for NoticeBoardSequence {}

#[unity::class("App","CommonRewardSequence")]
pub struct CommonRewardSequence {}
impl Bindable for CommonRewardSequence {}

#[unity::from_offset("App","ArenaOrderSequence", "CreateBind")]
pub fn arena_order_create_bind(parent: &mut BasicMenu<BasicMenuItem>, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "CreateDesc")]
pub fn well_create_desc<P: Bindable>( this: &P,  method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;

#[unity::from_offset("App","GodRoomPedestalSequence", "CreateDesc")]
pub fn god_room_create_desc<P: Bindable>( this: &P, method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;

#[unity::from_offset("App" ,"NoticeBoardSequence", "CreateDesc")]
pub fn notice_create_desc<P: Bindable>( this: &P, method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;

#[unity::from_offset("App" ,"HubItemShopSequence", "CreateDesc")]
pub fn item_shop_create_desc<P: Bindable>( this: &P, method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;

#[unity::from_offset("App" ,"HubWeaponShopSequence", "CreateDesc")]
pub fn weapon_shop_create_desc<P: Bindable>( this: &P, method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;

#[skyline::from_offset(0x02533950)]
pub fn item_gain_create_bind(parent: &mut BasicMenu<BasicMenuItem>, item_list: &List<ItemData>, title: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x02174d70)]
pub fn animal_create_bind(parent: &mut BasicMenu<BasicMenuItem>, shop: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x02170ee0)]
pub fn clear_locator_access_manager(this: &HubAccessManager, locator: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x021d4830)]
pub fn well_item_bind(this: &mut BasicMenu<BasicMenuItem>, method_info: OptionalMethod) -> BasicMenuResult;
pub fn can_well() -> bool {
    if GameUserData::get_sequence() == 6 { return false; }
    if GameUserData::get_sequence() == 4 {
        return HubUtil::get_current_scene_name().to_string() == "Hub_Solanel";
    }
    return false;
}
#[skyline::hook(offset=0x01f1ac80)]
pub fn notice_board_create_bind(proc: &mut ProcInst, initial_selected: i32, event_handler: *const u8, method_info: OptionalMethod){
    //set_achieve_menu_status();
    let parent = proc.parent.as_ref().unwrap().get_class().get_name();
    GameVariableManager::set_bool(crate::arena::ARENA_KEY, true);
    crate::arena::patch_arena();
    call_original!(proc, initial_selected, event_handler, method_info);
    let config_menu = proc.child.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>();

    if parent == "Hub_Sequence" || parent == "HubMenu"  {
        config_menu.full_menu_item_list.items[0].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = notice_item_acall as _);
        config_menu.full_menu_item_list.items[0].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = notice_item_getname as _);

        config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = weapon_shop_acall as _);
        config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = weapon_menu_item_getname as _);
        config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
        config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = weapon_menu_item_buildattribute as _);

        config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = item_shop_acall as _);
        config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = item_menu_item_getname as _);
        config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
        config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = item_menu_item_buildattribute as _);

        let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = refine_menu_item_acall as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = refine_menu_item_getname as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = refine_menu_item_buildattribute as _);
        config_menu.full_menu_item_list.add(new_menu_item);

        let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = arena_menu_item_acall as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = arena_menu_item_getname as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = arena_menu_item_buildattribute as _);
        config_menu.full_menu_item_list.add(new_menu_item);

        let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = god_room_menu_item_acall as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = god_room_menu_item_getname as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = god_room_menu_item_buildattribute as _);
        config_menu.full_menu_item_list.add(new_menu_item);

        // This is the accessory shop, not animal 
        let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = animal_menu_item_acall as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = animal_getname as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = animal_buildattribute as _);
        config_menu.full_menu_item_list.add(new_menu_item);
        config_menu.reserved_show_row_num = 7;
    }
    else {
        config_menu.full_menu_item_list.items[0].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = investment_item_acall as _);
        config_menu.full_menu_item_list.items[0].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = investment_item_get_name as _);
        config_menu.full_menu_item_list.items[0].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = investment_item_build as _);
        if GameUserData::get_sequence() == 6 { 
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = refine_menu_item_acall as _);
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = refine_menu_item_getname  as _);
               config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = refine_menu_item_buildattribute as _);
           }
        else {
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = well_menu_item_acall as _);
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = well_menu_item_getname as _);
            config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = well_menu_item_buildattribute as _);
        }
        config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = achieve_item_acall as _);
        config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = achieve_item_get_name as _);
        config_menu.full_menu_item_list.items[2].get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = investment_item_build as _);
        config_menu.reserved_show_row_num = 3; 
        return;  
    }
}
#[skyline::hook(offset=0x028b6ef0)]
pub fn hub_menu_create_bind(proc: &mut ProcInst, method_info: OptionalMethod){
    call_original!(proc, method_info);
    let config_menu = proc.child.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>();
    for x in 0..config_menu.full_menu_item_list.len() {
        if config_menu.full_menu_item_list.items[x].get_class_mut().get_name() == "MapInfoItem" {
            config_menu.full_menu_item_list.items[x].get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = notice_item_acall as _);
            config_menu.full_menu_item_list.items[x].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = notice_item_getname  as _);
        }
    }
    if config_menu.reserved_show_row_num < 9 {
        config_menu.reserved_show_row_num += 1;
        let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
        let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
        new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = collector_acall as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = collector_getname as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
        new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = collector_buildattribute as _);
        config_menu.full_menu_item_list.add(new_menu_item);
    }
}

#[skyline::hook(offset=0x01b3d270)]
pub fn gmap_menu_sub_shop_menu_create_bind(parent_menu: &mut BasicMenu<()>, parent_menu_item: &mut ConfigBasicMenuItem, method_info: OptionalMethod){

    call_original!(parent_menu, parent_menu_item, method_info);
    let config_menu = parent_menu.proc.child.as_mut().unwrap().cast_mut::<BasicMenu<BasicMenuItem>>();
    config_menu.reserved_show_row_num = 5;
    
    let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
    let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
    new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = notice_item_acall as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = notice_item_getname as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
    //new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = notice_menu_item_buildattribute as _);
    config_menu.full_menu_item_list.add(new_menu_item);

    let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
    let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
    new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = arena_menu_item_acall as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = arena_menu_item_getname as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = arena_menu_item_buildattribute as _);
    config_menu.full_menu_item_list.add(new_menu_item);

    let cock: &mut Il2CppClass = config_menu.full_menu_item_list.items[0].get_class().clone();
    let new_menu_item = il2cpp::instantiate_class::<BasicMenuItem>(cock).unwrap();
    new_menu_item.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = god_room_menu_item_acall as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = god_room_menu_item_getname as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("GetHelpText").map(|method| method.method_ptr = arena_menu_item_gethelptext as _);
    new_menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = god_room_menu_item_buildattribute as _);
    config_menu.full_menu_item_list.add(new_menu_item);
}
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
        // Auto item collection (limited to 500 items) and auto adopting

    let hub_sequence = HubSequence::get_instance();
    //let access_manager = hub_sequence.get_current_access_data();
    //let access_list = access_manager.access_list;
    let access_list = &mut hub_sequence.get_locator_group().access_list;
    
    let it = get_list_item_class();
    let test = il2cpp::instantiate_class::<List<ItemData>>(it);
    if test.is_err() { return BasicMenuResult::se_miss(); }
    else if it.get_methods().iter().find(|method| method.get_name() == Some(String::from("Add"))).is_none() {return BasicMenuResult::se_miss(); }
    let item_list = test.unwrap();
    let mut animal_count = 0;
    item_list.items = Il2CppArray::new(750).unwrap();
    access_list.iter_mut().for_each(|access_point|{
        if let Some(access) = access_point.access_data {
            if !access.get_is_done() {
                if try_capture_animal(access) {  animal_count += 1; }
                else if let Some(iid) = access.aid {
                    if let Some(w_item) =  ItemData::get(iid) {
                        if unsafe { !is_null_empty(w_item.name, None) && !is_null_empty(w_item.help, None) } {
                            let count = HubUtil::get_item_count_with_bonus(w_item, access.get_item_count() );
                            for _y in 0..count { if item_list.len() < 750 {  if let Some(valid_item) = ItemData::get_mut(iid) { item_list.add( valid_item ); }}}
                            if access_point.item_effect != 0 { unsafe { crate::hub::set_game_object_active(access_point.item_effect, false, None); } }
                        }
                    }
                }
                access.done();
                access_point.done();
            }
        }
    });
    if GameUserData::get_sequence() == 4 { unsafe { item_gain_create_bind(this.menu, item_list, "MID_MENU_TITLE_HUB".into(), None) }; } 
    else if GameUserData::get_sequence() == 5 { unsafe { item_gain_create_bind(this.menu, item_list, "MID_MENU_TITLE_KIZUNA".into(), None) }; }
    if animal_count != 0 { println!("Adopted {} Animal(s)", animal_count); }
    println!("Obtained {} Item(s)", item_list.len());
    BasicMenuResult::se_decide()
}
pub extern "C" fn notice_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        unsafe { 
        if GameUserData::get_sequence() == 6 {
            let gmap = get_gmapsequence();
            mapinfo_close(gmap.map_info, None);  
        }
        if GameUserData::get_sequence() == 4 || GameUserData::get_sequence() == 5 {
            let hub = HubSequence::get_instance();
            let mini_map = hub.get_mini_map();
                if mini_map.is_show() { mini_map.set_mode(0); }
                mini_map.hide_system_menu();
            }
        }
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        let proc = NoticeBoardSequence::instantiate().unwrap();
        proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
        unsafe {
            let descs = notice_create_desc(proc, None);
            proc.create_bind(this.menu, descs, "NoticeBoardSequence");
        }
        BasicMenuResult::se_decide()
    } else { BasicMenuResult::se_miss() }
}
pub extern "C" fn animal_menu_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all =
                unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        unsafe {
            animal_create_bind(this.menu, 0, None);
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
        }
        BasicMenuResult::se_decide()
    } else { BasicMenuResult::se_miss() }
}
pub extern "C" fn refine_menu_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all =
                unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });

        let proc = HubRefineShopSequence::new();
        proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _).unwrap();
        let descs = proc.create_desc();
        proc.create_bind(this.menu, descs, "HubRefineShopSequence");
        BasicMenuResult::se_decide()
    } else { BasicMenuResult::se_miss() }
}
pub extern "C" fn item_shop_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        let proc = HubItemShopSequence::instantiate().unwrap();
        proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
        let descs = proc.create_desc();
        proc.create_bind(this.menu, descs, "HubItemShopSequence");
        BasicMenuResult::se_decide()
    } else { BasicMenuResult::se_miss() }
}
pub extern "C" fn weapon_shop_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        let proc = HubWeaponShopSequence::instantiate().unwrap();
        proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
        let descs = proc.create_desc();
        proc.create_bind(this.menu, descs, "HubWeaponShopSequence");
        BasicMenuResult::se_decide()
    } else { BasicMenuResult::se_miss() }
}

pub extern "C" fn arena_menu_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        if GameUserData::get_sequence() == 6 {
            let gmap = get_gmapsequence();
            unsafe { mapinfo_close(gmap.map_info, None);  }
        }
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        let proc = ArenaOrderSequence::new();
        proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
        unsafe { arena_order_create_bind(this.menu, None); }
        BasicMenuResult::se_decide()
    } else { BasicMenuResult::se_miss() }
}

pub extern "C" fn well_menu_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        if GameUserData::get_sequence() == 6 {
            let _ = Patch::in_text(0x01b2d6d0).bytes(&[0x80, 0x00, 0x80, 0x52]);
            let _ = Patch::in_text(0x01b2d6d4).bytes(&[0xC0, 0x03, 0x5F, 0xD6]);
        }
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        let proc = WellSequence::new();
        proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _).unwrap();
        let descs = proc.create_desc();
        proc.create_bind(this.menu, descs, "WellSequence");
        BasicMenuResult::se_decide()
    } else { BasicMenuResult::se_miss() }
}

pub extern "C" fn god_room_menu_item_acall(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !this.is_attribute_disable() {
        // remove ring cleaning
        let set_hide = &[0x80, 0x00, 0x80, 0x52];
        let set_true = &[0x20, 0x00, 0x80, 0x52];
        let set_return = &[0xC0, 0x03, 0x5F, 0xD6];
        let _ = Patch::in_text(0x01cd97e0).bytes(set_true);
        let _ = Patch::in_text(0x01cd97e4).bytes(set_return);
        let _ = Patch::in_text(0x01cd99a0).bytes(set_hide);
        if GameUserData::get_sequence() == 6 {
            let gmap = get_gmapsequence();
            unsafe { mapinfo_close(gmap.map_info, None); }
        }
        this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
            let close_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
            close_anime_all(this.menu, method.method_info);
        });
        let proc = GodRoomPedestalSequence::instantiate().unwrap();
        proc.get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _).unwrap();
        let descs = proc.create_desc();
        proc.create_bind(this.menu, descs, "GodRoomPedestalSequence");
        BasicMenuResult::se_decide()
    } else {  BasicMenuResult::se_miss() }
}
pub extern "C" fn notice_item_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_MENU_NOTICE_BOARD_TITLE") }
pub extern "C" fn arena_menu_item_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Arena") }
pub extern "C" fn arena_menu_item_gethelptext(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { "".into() }
pub extern "C" fn arena_menu_item_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    if  HubFacilityData::get("AID_闘技場").unwrap().is_complete() { 
        if GameVariableManager::get_bool("G_ARENA_SKIP") { 1 } else { 2 }
    }
    else { 4 }
}

pub extern "C" fn well_menu_item_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Well") }
pub extern "C" fn well_menu_item_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    let refine_data = HubFacilityData::get("AID_不思議な井戸").unwrap();
        if refine_data.is_complete() && can_well() { 1 } else { 4 }
}

pub extern "C" fn god_room_menu_item_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_GodRoom") }
pub extern "C" fn god_room_menu_item_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    let refine_data = HubFacilityData::get("AID_紋章士の間").unwrap();
    if refine_data.is_complete(){ 1 } else { 4 }
}

pub extern "C" fn refine_menu_item_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Blacksmith") }
pub extern "C" fn refine_menu_item_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    let refine_data = HubFacilityData::get("AID_錬成屋").unwrap();
    if refine_data.is_complete() { 1 } else { 4  }
}

pub extern "C" fn item_menu_item_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_ItemShop") }
pub extern "C" fn item_menu_item_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    let refine_data = HubFacilityData::get("AID_道具屋").unwrap();
    if refine_data.is_complete() { 1 } else { 4  }
}

pub extern "C" fn weapon_menu_item_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_WeaponShop") }
pub extern "C" fn weapon_menu_item_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    let refine_data = HubFacilityData::get("AID_武器屋").unwrap();
    if refine_data.is_complete() { 1 } else { 4  }
}

pub extern "C" fn collector_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Area_ItemReturn") }
pub extern "C" fn collector_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    if get_item_count() != 0 { 1 } else { 4 }
}

pub extern "C" fn animal_getname(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_AccessoryShop") }
pub extern "C" fn animal_buildattribute(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> i32 {
    if HubFacilityData::get("AID_アクセサリ").unwrap().is_complete() {
        if HubUtil::get_current_scene_name().to_string() == "Hub_Solanel" { 1 }
        else {  2 }
    } else { 4 }
}
