use engage::{
    gamemessage::GameMessage, gamesound::GameSound,
    sequence::commonrewardsequence::CommonRewardSequence,
    proc::ProcVoidFunction,
};
pub use super::{*, super::data::{FishingTargetListData, FishingFishData}};
use crate::configmenu::FISH_KEY;


pub struct FishingMenuItem; 
impl BasicMenuItemMethods for FishingMenuItem {
    extern "C" fn a_call(this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() && GameVariableManager::get_number("G_Fishing_PlayCount") > 0 {
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all =
                    unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<BasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });
            close_hub_mini_map();
            TitleBar::hide_footer();
            unsafe { super::fishing::fishing_sequence_create_bind(this.menu, None); }
            super::fishing::fishing_game_sequence_desc_edit(this.menu.proc.child.as_mut().unwrap().descs.get_mut());
            this.menu.proc.child.as_mut().unwrap().get_class_mut().get_virtual_method_mut("OnDispose").map(|method| method.method_ptr = open_anime_all_ondispose as _) .unwrap();
            BasicMenuResult::se_decide()
        } else { BasicMenuResult::se_miss() }
    }
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { Mess::get("MID_Hub_Talk_Fishing") }
    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if HubFacilityData::get("AID_釣り").unwrap().is_complete() { 
            if GameVariableManager::get_number("G_Fishing_PlayCount") > 0 {
                BasicMenuItemAttribute::Enable
            }
            else { BasicMenuItemAttribute::Disable }
        }
        else { BasicMenuItemAttribute::Hide }
    }
}


/// Editing FishingGameSequence Desc to autofish without the scene
pub fn fishing_game_sequence_desc_edit(descs: &mut Array<&mut ProcDesc>) {
    (*descs)[7] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    (*descs)[8] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    (*descs)[9] = ProcDesc::call(ProcVoidFunction::new(None, fishing_game_bind));
    (*descs)[10] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
    (*descs)[11] = ProcDesc::call(ProcVoidMethod::new(None, proc_do_nothing));
}


fn get_random_fish() -> &'static mut FishingFishData {
    let mut rod = 0;
    ItemData::get_list().unwrap().iter().filter(|item| item.usetype == 35 && item.get_inventory() != 0 ).for_each(|item|{
        let name = item.name.to_string();
        if name == "MIID_FishingRod_Normal" && rod == 0 {
            rod = 1;
        }
        if name == "MIID_FishingRod_AllPurpose" {
            rod = 2;
        }
    });
    let list =  FishingTargetListData::get_list().unwrap();
    let filter = match rod {
        1 => "ID_StickB",
        2 => "ID_StickC",
        _ => "ID_StickA",
    };
    FishingGameSequence::class().get_static_fields_mut::<FishingGameSequenceStaticFields>().rod_result = rod;
    let mut sum = 0;
    let stick_set: Vec<_> = list.iter().filter(|t| t.id.contains(filter) ).collect();
    if GameVariableManager::get_bool(FISH_KEY) {
        let mut iid_d = String::new();
        let mut bond = 0;
        stick_set.iter().for_each(|t|{
            let iid = format!("ID_{}", t.fish_id);
            if let Some(data) = FishingFishData::get(iid.as_str()) {
                if data.bond > bond {
                    bond = data.bond;
                    iid_d = iid.clone();
                }
            }
        });
        if let Some(data) = FishingFishData::get_mut(iid_d) {
            return data;
        }
    }


    let prob: Vec<_> = stick_set.iter().map(|t| { sum += t.priority; sum }).collect();
    let rng = Random::get_system();
    if rng.get_value(100) < 20 {
        if let Some(data) = FishingFishData::get_mut(format!("ID_{}",  GameVariableManager::get_string("G_Fishing_TargetFish"))) {
            return  data;
        }
    }

    let value = rng.get_value(sum);
    let index = prob.iter().filter(|&&v| v < value).count();
    let fish_id = format!("ID_{}",
    if index == 0 {
        stick_set[0].fish_id
    }
    else {
        stick_set[index - 1].fish_id
    });
    FishingFishData::get_mut(fish_id).unwrap()
}

pub extern "C" fn fishing_game_bind(proc: &mut ProcInst, method_info: OptionalMethod) {
    unsafe { fishing_game_sequence_create_bind(proc, None); }
    //let desc = proc.child.as_mut().unwrap().descs.get_mut();
    let fishing = proc.child.as_mut().unwrap().cast_mut::<FishingGameSequence>();
    unsafe { set_forecast_fish(0, None); }
    fishing.fishing_script = FishingFish::instantiate().unwrap();
    unsafe { 
        fishing_ctor(fishing.fishing_script, None); 
        fishing.fishing_script.data = get_random_fish();
        set_bonus(fishing, None);
        set_fishing_id(fishing.fishing_script.data.name, method_info);
    }
    // desc[3] = ProcDesc::call(ProcVoidFunction::new(None, select_fish));
    fishing.proc.descs.get_mut()[2] =ProcDesc::call(ProcVoidFunction::new(None, set_bonus));
    fishing.proc.descs.get_mut()[3] = ProcDesc::jump(17);
    // 55 56 57 58 59 
    // App.FishingGameSequence.GetPrizeBond() 55
    // Method$App.FishingGameSequence.GetPrizeItem() 56
    // App.FishingGameSequence.DecreasePlayCount() 57
    // Method$App.FishingGameSequence.IncreasePlayCounter 58
    fishing.proc.descs.get_mut()[55] = ProcDesc::call(ProcVoidFunction::new(None, get_item_rewards));
    fishing.proc.descs.get_mut()[56] = ProcDesc::call(ProcVoidFunction::new(None, get_bond_reward));
    fishing.proc.descs.get_mut()[57] = ProcDesc::call(ProcVoidFunction::new(None, repeat));
    fishing.proc.descs.get_mut()[59] = ProcDesc::call(ProcVoidFunction::new(None, repeat));
    if GameVariableManager::exist("G_FishingBond") { GameVariableManager::set_number("G_FishingBond", 0); }
    else { GameVariableManager::make_entry_norewind("G_FishingBond", 0); }
    for x in 67..74 {
        fishing.proc.descs.get_mut()[x] = ProcDesc::call(ProcVoidMethod::new(None,proc_do_nothing));
    }
}
pub extern "C" fn set_bonus(this: &mut FishingGameSequence, _method_info: OptionalMethod){
    let target = GameVariableManager::get_string("G_Fishing_TargetFish");
    if this.fishing_script.data.name.contains(target) { this.bonus = true; }
    else { this.bonus = false; }
    if GameVariableManager::get_bool(FISH_KEY) { this.bonus = true; }
}
pub extern "C" fn select_fish(this: &mut FishingGameSequence, method_info: OptionalMethod){
    this.fishing_script.data = get_random_fish();
    set_bonus(this, None);
    unsafe {  set_fishing_id(this.fishing_script.data.name, method_info); }
}

pub extern "C" fn get_item_rewards(this: &mut FishingGameSequence, _method_info: OptionalMethod) {
    let nfish = GameVariableManager::get_number("G_Fishing_PlayCount");
    if nfish == 0 { return; }
    let it = crate::utils::get_list_item_class();
    let test = il2cpp::instantiate_class::<List<ItemData>>(it);
    let item_list = test.unwrap();
    item_list.items = Il2CppArray::new(20).unwrap();

    for _x in 0..nfish {
        let bond =  GameVariableManager::get_number("G_FishingBond") +  if this.bonus { 2 } else { 1 } * this.fishing_script.data.bond;
        GameVariableManager::set_number("G_FishingBond", bond);
        unsafe { fishinggame_increase_playcount(this, None); }
        if let Some(item) = ItemData::get_mut(this.fishing_script.data.food_type) {
            item_list.add(item);
        }
        select_fish(this, None);
    }
    GameVariableManager::set_number("G_Fishing_PlayCount", 0);
    CommonRewardSequence::create_bind_for_well(this, item_list, "MID_Hub_Talk_Fishing".into());
}

pub extern "C" fn get_bond_reward(this: &mut FishingGameSequence, _method_info: OptionalMethod) {
    let bond = GameVariableManager::get_number("G_FishingBond");
    if bond == 0 { return; }
    let mess_tag = Mess::create_sprite_tag(2, "Bonds".into());
    let bond_value = unsafe { value_to_string_with_comma(bond, None) };
    Mess::set_argument(0, mess_tag);
    Mess::set_argument(1, bond_value);
    let message = Mess::get("MID_MSG_GET_PIECE");
    GameSound::post_event("ItemGet", None);
    GameMessage::create_key_wait(this, message.to_string());
    GameUserData::add_bond(bond);
}

pub extern "C" fn repeat(this: &mut FishingGameSequence, _method_info: OptionalMethod){
    if GameVariableManager::get_number("G_Fishing_PlayCount") > 0 {
        select_fish(this, None);
        ProcInst::jump(this, 17);
    }
    else { ProcInst::jump(this, 20); }
}


#[unity::from_offset("App", "FishingGameSequence", "IncreasePlayCounter")]
fn fishinggame_increase_playcount(this: &FishingGameSequence, method_info: OptionalMethod);

#[skyline::from_offset(0x01c81ad0)]
fn value_to_string_with_comma(value: i32, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::class("App", "FishingFish")]
pub struct FishingFish {
    junk: [u8; 0x30],
    pub data: &'static mut FishingFishData,
}


#[unity::class("App", "FishingGameSequence")]
pub struct FishingGameSequence {
    pub proc: ProcInstFields,
    junk: [u8; 0x220],
    pub fishing_script: &'static mut FishingFish,
    junk2: [u8; 0xA8],
    pub voice: bool,
    pub bonus: bool,
}
pub struct FishingGameSequenceStaticFields {
    pub rod_result: i32,
}
impl Bindable for FishingGameSequence {}
impl AsMut<ProcInstFields> for FishingGameSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}

impl AsRef<ProcInstFields> for FishingGameSequence {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

#[skyline::from_offset(0x02701620)]
pub fn fishing_game_sequence_create_bind<P: Bindable>(proc: &P, method_info: OptionalMethod);

#[skyline::from_offset(0x0260a4a0)]
pub fn fishing_sequence_create_bind<P: Bindable>(proc: &P, method_info: OptionalMethod);

#[skyline::from_offset(0x02ae4030)]
pub fn set_forecast_fish(this: u64, method_info: OptionalMethod);

#[unity::from_offset("App", "FishingFish", "SelectFish")]
fn fishing_select_fish(this: &FishingFish, method_info: OptionalMethod);

#[skyline::from_offset(0x02a65ac0)]
fn set_fishing_id(value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "FishingFish", ".ctor")]
fn fishing_ctor(this: &FishingFish, method_info: OptionalMethod);
