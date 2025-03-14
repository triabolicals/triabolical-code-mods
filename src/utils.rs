use il2cpp::class::VirtualInvoke;
use skyline::patching::Patch;
use unity::prelude::*;
use concat_string::concat_string;
use engage::{
    backgroundmanager::BackgroundManager, menu::{ BasicMenuItem, BasicMenuItemMethods},
    mess::*, proc::ProcInst,
    titlebar::TitleBar,
};

pub const MID_STATS: &[&str] = &["MID_SYS_HP", "MID_SYS_Str", "MID_SYS_Tec", "MID_SYS_Spd", "MID_SYS_Lck", "MID_SYS_Def", "MID_SYS_Mag", "MID_SYS_Res"];

pub fn create_menu_item_impl<Methods: BasicMenuItemMethods>(class: &Il2CppClass, _methods: Methods) -> &'static mut BasicMenuItem {
    let cloned = class.clone();
    let item:&mut BasicMenuItem = il2cpp::instantiate_class(cloned).unwrap();
    let method = cloned.get_methods().iter().find(|method| method.get_name() == Some(String::from(".ctor"))).unwrap();
    let ctor = unsafe {
        std::mem::transmute::<_, extern "C" fn(&BasicMenuItem, &MethodInfo) -> ()> (method.method_ptr,)
    };
    ctor(item, method);
    item.get_class_mut()
        .get_virtual_method_mut("GetName")
        .map(|method| method.method_ptr = Methods::get_name as _)
        .unwrap();

    item
        .get_class_mut()
        .get_virtual_method_mut("GetHelpText")
        .map(|method| method.method_ptr = Methods::get_help_text as _);

    item.get_class_mut()
        .get_virtual_method_mut("ACall")
        .map(|method| method.method_ptr = Methods::a_call as _)
        .unwrap();

    item.get_class_mut()
        .get_virtual_method_mut("BuildAttribute")
        .map(|method| method.method_ptr = Methods::build_attributes as _)
        .unwrap();

    item

}

pub extern "C" fn proc_do_nothing(_this: &mut ProcInst, _method_info: OptionalMethod) {} 

pub fn get_list_item_class() -> &'static Il2CppClass {
    let common_rewards_sequence = engage::sequence::commonrewardsequence::CommonRewardSequence::instantiate().unwrap();
    let methods = common_rewards_sequence.get_class().get_methods();
    let ctor_parameters = methods[0].get_parameters();
    let para = unity::prelude::Il2CppClass::from_il2cpptype( ctor_parameters[2].parameter_type ).unwrap();
    return para;
}

pub extern "C" fn open_anime_all_ondispose(this: &mut ProcInst, _method_info: OptionalMethod) {
    // restore_instructions();
    this.parent.as_ref().unwrap().get_class().get_virtual_method("OpenAnimeAll").map(|method| {
        let open_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&ProcInst, &MethodInfo)>(method.method_info.method_ptr) };
        open_anime_all(this.parent.as_ref().unwrap(), method.method_info);
    });
    if BackgroundManager::is_bind() { BackgroundManager::unbind(); }
    TitleBar::show_footer();
}

pub fn on_str() -> &'static Il2CppString { Mess::get("MID_CONFIG_TUTORIAL_ON") }
pub fn off_str() -> &'static Il2CppString { Mess::get("MID_CONFIG_TUTORIAL_OFF") }

pub fn setting_str(string: &str) -> &'static Il2CppString  {
    let lang =unsafe { get_lang(None) };
    let mess = Mess::get(string).to_string();
    let config = Mess::get("MID_MENU_CONFIG").to_string();
    match lang {
        //English (US)
        1|2|10 => { concat_string!(mess, " ",  config) },
        //Spanish (AM)
        3|4|5 => { concat_string!(config, " de ", mess) },
        //Italian
        6 => { concat_string!(config, " del ", mess) }
        //German
        7 => { concat_string!(mess, config.to_lowercase()) }
        //Sim Chinese
        _ => { concat_string!(mess, " ",  config) }
    }.into()
}
pub fn get_stat_with_value(index: usize, v: i8) -> &'static Il2CppString {
    let value_str = if v < 0 { format!("{}", v) } else { format!("+{}", v) };
    let stat_str = Mess::get(MID_STATS[index]);
    return concat_string!(stat_str.to_string(), value_str).into();
}

pub fn get_stat_label(index: usize) -> String {
    match index {
        0 => { return Mess::get("MID_SYS_HP").to_string();}
        1 => { return Mess::get("MID_SYS_Str").to_string();}
        2 => { return Mess::get("MID_SYS_Tec").to_string();}
        3 => { return Mess::get("MID_SYS_Spd").to_string();}
        4 => { return Mess::get("MID_SYS_Lck").to_string();}
        5 => { return Mess::get("MID_SYS_Def").to_string();}
        6 => { return Mess::get("MID_SYS_Mag").to_string();}
        7 => { return Mess::get("MID_SYS_Res").to_string();}
        8 => { return Mess::get("MID_SYS_Phy").to_string();}
        9 => { return Mess::get("MID_SYS_Vis").to_string();}
        10 => { return Mess::get("MID_SYS_Mov").to_string();}
        11 => { return Mess::get("MID_SYS_Avo").to_string(); }
        12 => { return Mess::get("MID_SYS_Crit").to_string();}
        13 => { return Mess::get("MID_SYS_Hit").to_string();}
        14 => { return  Mess::get("MID_SYS_Mt").to_string(); }
        15 => { return Mess::get("MID_SYS_Secure").to_string(); }
        16 => { return Mess::get("MID_SYS_Weight").to_string(); } 
        _ => { return "".to_string(); }
    }
}

pub fn get_nested_virtual_methods_mut(namespace: &str, class_name: &str, nested_class: &str, method_name: &str) -> Option<&'static mut VirtualInvoke> {
    if let Some(cc) = Il2CppClass::from_name(namespace, class_name).unwrap().get_nested_types().iter()
        .find(|x| x.get_name() == nested_class) {
        let menu_mut = Il2CppClass::from_il2cpptype(cc.get_type()).unwrap();
        menu_mut.get_virtual_method_mut(method_name)
    }
    else { None }
}

pub fn return_bool_instruction(address: usize, value: bool){
    mov_w0_bool_instruction(address, value);
    return_instruction(address+0x4);
 }
 pub fn mov_w0_bool_instruction(address: usize, value: bool){
    if value { Patch::in_text(address).bytes(&[0x20,0x00, 0x80, 0x52]).unwrap(); }
    else { Patch::in_text(address).bytes(&[0x00,0x00, 0x80, 0x52]).unwrap(); }
}
pub fn return_instruction(address: usize) {
    Patch::in_text(address).bytes(&[0xC0, 0x03, 0x5F, 0xD6]).unwrap();
}
pub fn has_dlc() -> bool { unsafe { has_content(0, None) } }

#[skyline::from_offset(0x029f4270)]
pub fn has_content(content: i32, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01bdbc80)]
pub fn get_lang(method_info: OptionalMethod) -> i32;