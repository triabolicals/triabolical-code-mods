use item::RewardData;
use skyline::patching::Patch;
use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuItemAttribute, BasicMenuResult};
use engage::gamevariable::*;
use engage::gamedata::*;
use super::*;

pub fn patch_gift(){
    let ret = &[0xC0, 0x03, 0x5F, 0xD6];
    match GameVariableManager::get_number(GIFT_KEY) {
        1 => {
            Patch::in_text(0x023f3f18).bytes(ret).unwrap();
            Patch::in_text(0x023f4088).bytes(ret).unwrap();
        }
        2 => {
            Patch::in_text(0x023f3fc8).bytes(ret).unwrap();
            Patch::in_text(0x023f4138).bytes(ret).unwrap();
        }
        3 => {
            Patch::in_text(0x023f3f18).bytes(ret).unwrap();
            Patch::in_text(0x023f4088).bytes(ret).unwrap();
            Patch::in_text(0x023f3fc8).bytes(ret).unwrap();
            Patch::in_text(0x023f4138).bytes(ret).unwrap();
            Patch::in_text(0x0253d7c0).bytes(&[0x00,0x00,0x80, 0x52]).unwrap();
            Patch::in_text(0x0253d8b0).bytes(&[0x00,0x00,0x80, 0x52]).unwrap();
        }
        _ => {
            Patch::in_text(0x023f3f18).bytes(&[0x3a,0xff,0xff,0x17]).unwrap();   //DLC0
            Patch::in_text(0x023f3fc8).bytes(&[0x0e,0xff,0xff,0x17]).unwrap();  //Patch0
            Patch::in_text(0x023f4088).bytes(&[0xde,0xfe,0xff,0x17]).unwrap();   //DLC1
            Patch::in_text(0x023f4138).bytes(&[0xb2,0xfe,0xff, 0x17]).unwrap();  //Patch3
            Patch::in_text(0x0253d8b0).bytes(&[0x70,0xda,0x12, 0x94]).unwrap();
            Patch::in_text(0x0253d7c0).bytes(&[0xac,0xda,0x12, 0x94]).unwrap();
        }
    }
}

pub struct Giftmod;
impl ConfigBasicMenuItemSwitchMethods for Giftmod {
    fn init_content(_this: &mut ConfigBasicMenuItem){ patch_gift(); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if RewardData::get_list().is_some_and(|list| list.len() < 4){
            this.command_text = "Corrupted".into();
            this.help_text = "Reward / Well Data is corrupted in merged Item.xml.".into();
            this.update_text();
            return BasicMenuResult::new();
        }

        let toggle = GameVariableManager::get_number(GIFT_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(GIFT_KEY, result );
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            patch_gift();
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.help_text = match GameVariableManager::get_number(GIFT_KEY){ 
            1 => { "Accept only patch update items." },
            2 => { "Accept only paid DLC items." },
            3 => { "Accept no gift items." },
            _ => { "Accept both patch and DLC gift items." } 
        }.into();
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        this.command_text = match GameVariableManager::get_number(GIFT_KEY){ 
            1 => { "Patch Updates." },
            2 => { "Expansion Pass" },
            3 => { "No Freebies" },
            _ => { "All Gifts" },
        }.into();
    }
}



extern "C" fn gift() -> &'static mut ConfigBasicMenuItem { 
    let menu_item = ConfigBasicMenuItem::new_switch::<Giftmod>("Gift Options");
    menu_item.get_class_mut().get_virtual_method_mut("BuildAttribute").map(|method| method.method_ptr = gift_build_attribute as _);
    menu_item
}

pub fn gift_install(){ cobapi::install_game_setting(gift); }


fn gift_build_attribute(_this: &ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute  {

    if crate::utils::has_dlc() {
        if GameVariableManager::get_bool("G_拠点_DLC特典アイテム0受け取り済み") || GameVariableManager::get_bool("G_拠点_DLC特典アイテム1受け取り済み") {
            return BasicMenuItemAttribute::Hide;
        }
        else { return BasicMenuItemAttribute::Enable; }
    }
    if GameVariableManager::get_bool("G_拠点_Patch0特典アイテム受け取り済み") || GameVariableManager::get_bool("G_拠点_Patch3特典アイテム受け取り済み") {
        BasicMenuItemAttribute::Hide
    }
    else { BasicMenuItemAttribute::Enable }
}

