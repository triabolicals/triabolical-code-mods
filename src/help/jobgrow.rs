use super::*;

#[unity::class("App", "ClassChangeJobMenuContent")]
pub struct ClassChangeJobMenuContent {
    junk: [u8; 0x108],
    pub help_text: &'static mut TextMeshProUGUI,
}

#[unity::class("App", "ClassChangeChangeJobData")]
pub struct ClassChangeChangeJobData {
    pub job: &'static JobData,
}

#[unity::class("App", "ClassChangeJobMenuItem")]
pub struct ClassChangeJobMenuItem {
    pub menu: &'static mut ClassChangeJobMenu,
    junk: [u8; 0x50],
    pub job_data: &'static ClassChangeChangeJobData,
}
#[unity::class("App", "ClassChangeJobMenu")]
pub struct ClassChangeJobMenu {
    menu_stuff: [u8; 0x60],
    pub menu_item_content: &'static mut ClassChangeJobMenuContent,
}


#[skyline::hook(offset=0x01ea52d0)]
pub fn class_change_set_job_details_hook(this: &mut ClassChangeJobMenuContent, data: &ClassChangeChangeJobData, method_info: OptionalMethod) {
    call_original!(this, data, method_info);
    match GameVariableManager::get_number("G_JobGrowth") {
        1 => {
            let stats = create_job_growth_string(data.job);
            let name = format!("{} {}", Mess::get(data.job.name), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
            let final_str = concat_string!(name, "\n", stats).into();
            this.help_text.set_text(final_str, true);
        },
        2 => {
            let old_unit = unsafe { class_change_get_unit(None) };
            old_unit.class_change(data.job);
            let stats = unit_total_growths(old_unit);
            let name = format!("{} {} {}", Mess::get(old_unit.get_job().name), Mess::get_name(old_unit.person.pid), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
            let final_str = concat_string!(name, "\n", stats).into();
            this.help_text.set_text(final_str, true);
        },
        3 => {
            let new_unit = unsafe { class_change_get_unit(None) };
            new_unit.class_change(data.job);
            let old_unit = unsafe { class_change_get_unit(None) };
            let stats = unit_diff_growths(old_unit, new_unit);
            let name = format!("{}: {} -> {}",  Mess::get_name(old_unit.person.pid), Mess::get(old_unit.get_job().name), Mess::get(data.job.name));
            let final_str = concat_string!(name, "\n", stats).into();
            this.help_text.set_text(final_str, true);
        },
        _ => {},
    }
}

fn create_job_growth_string(job: &JobData) -> String {
    let mut out = "".to_string();
    let diff = job.get_diff_grow();
    let mut count = 0;
    let stat_order = [0, 1, 6, 2, 3, 4, 5, 7, 8];
    for x in stat_order {
        let stat = if diff[x] != 0 {  format!("{}: {}%", crate::utils::get_stat_label(x), diff[x]) }
            else { format!("{}: -", crate::utils::get_stat_label(x))};
            out = 
                if count == 0 { stat }
                else if count == 4 { format!("{}\n{}", out, stat) }
                else { format!("{}, {}", out, stat) };
            count += 1;
        //}
    }
    return out;
}

fn unit_total_growths(unit: &Unit) -> String {
    let mut out = "".to_string();
    let mut count = 0;
    let stat_order = [0, 1, 6, 2, 3, 4, 5, 7, 8];
    for x in stat_order {
        let stat = format!("{}: {}%", crate::utils::get_stat_label(x as usize), unit.get_capability_grow(x, false));
            out = 
                if count == 0 { stat }
                else if count == 4 { format!("{}\n{}", out, stat) }
                else { format!("{}, {}", out, stat) };
            count += 1;
        //}
    }
    return out;
}

fn unit_diff_growths(before: &Unit, after: &Unit) -> String {
    let mut out = "".to_string();
    let mut count = 0;
    let stat_order = [0, 1, 6, 2, 3, 4, 5, 7, 8];
    for x in stat_order {
        let value = after.get_capability_grow(x, false) - before.get_capability_grow(x, false);
        let stat = format!("{}: {}%", crate::utils::get_stat_label(x as usize), value);
            out = 
                if count == 0 { stat }
                else if count == 4 { format!("{}\n{}", out, stat) }
                else { format!("{}, {}", out, stat) };
            count += 1;
        //}
    }
    return out;

}

pub fn job_menu_item_x_call(this: &mut ClassChangeJobMenuItem, _method_info: OptionalMethod) -> i32 {
    let current_mode = GameVariableManager::get_number("G_JobGrowth");
    let new_mode = ( current_mode + 1 ) % 4;
    GameVariableManager::set_number("G_JobGrowth", new_mode);
    match new_mode {
        1 => {
            let stats = create_job_growth_string(this.job_data.job);
            let name = format!("{} {}", Mess::get(this.job_data.job.name), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
            let final_str = concat_string!(name, "\n", stats).into();
            this.menu.menu_item_content.help_text.set_text(final_str, true);
        },
        2 => {
            let old_unit = unsafe { class_change_get_unit(None) };
            old_unit.class_change(this.job_data.job);
            let stats = unit_total_growths(old_unit);
            let name = format!("{} {} {}",  Mess::get(old_unit.get_job().name), Mess::get_name(old_unit.person.pid), Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE"));
            let final_str = concat_string!(name, "\n", stats).into();
            this.menu.menu_item_content.help_text.set_text(final_str, true);
        },
        3 => {
            let new_unit = unsafe { class_change_get_unit(None) };
            new_unit.class_change(this.job_data.job);
            let old_unit = unsafe { class_change_get_unit(None) };
            let stats = unit_diff_growths(old_unit, new_unit);
            let name = format!("{}: {} -> {}", Mess::get_name(old_unit.person.pid), Mess::get(old_unit.get_job().name), Mess::get(this.job_data.job.name));
            let final_str = concat_string!(name, "\n", stats).into();
            this.menu.menu_item_content.help_text.set_text(final_str, true);
        },
        _ => {
            let help_text = Mess::get(this.job_data.job.help);
            this.menu.menu_item_content.help_text.set_text(help_text, true);
        },
    }
    return 0x80;
}
pub fn job_menu_item_selected(this: &mut ClassChangeJobMenuItem, _method_info: OptionalMethod) {
    unsafe { job_menu_item_on_selected(this, None); }
    let old_unit = unsafe { class_change_get_unit(None) };
    old_unit.class_change(this.job_data.job);
    crate::sortie::accessory::reload_unit_info(old_unit);
}

#[skyline::from_offset(0x019c7ac0)]
pub fn job_menu_item_on_selected(this: &mut ClassChangeJobMenuItem, method_info: OptionalMethod);

#[skyline::from_offset(0x01ea4680)]
pub fn class_change_get_unit(method_info: OptionalMethod) -> &'static Unit;

pub fn job_menu_calls_install() {
    let menu = Il2CppClass::from_name("App", "ClassChangeJobMenu").unwrap().get_nested_types().iter().find(|x| x.get_name() == "ClassChangeJobMenuItem").unwrap();
    let menu_mut = Il2CppClass::from_il2cpptype(menu.get_type()).unwrap();
    menu_mut.get_virtual_method_mut("XCall").map(|method| method.method_ptr = job_menu_item_x_call as _);
    menu_mut.get_virtual_method_mut("OnSelect").map(|method| method.method_ptr = job_menu_item_selected as _);
    println!("Replaced Virtual Method of ClassChangeJobMenuItem");
}
