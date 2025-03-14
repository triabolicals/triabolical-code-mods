use super::*;

#[unity::hook("App","HelpParamSetter", "SetItemData")]
pub fn help_param_setter_set_item_data_hook(this: &mut HelpParamSetter, frame: u64, data: Option<&ItemData>, unit: &Unit, god: &GodUnit, ring: u64, endurance: i32, item: Option<&UnitItem>, is_use_enchant: bool, method_info: OptionalMethod){
    call_original!(this, frame, data, unit, god, ring, endurance, item, is_use_enchant, method_info);
    if data.is_some() && item.is_some() {
        if data.unwrap().usetype == 1  {
            let power = item.unwrap().get_power();
            let power0: i32  = data.unwrap().power.into();
            let text = this.contexts_text.get_text().to_string();
            let mut power_string = format!(": {}\n", power0);
            if power < power0 { power_string = format!(": {} ({})\n", power, power-power0);  }
            else if power0 < power { power_string = format!(": {} (+{})\n", power, power-power0); }
            let might_str =  concat_string!(Mess::get("MID_SYS_Mt").to_string(), power_string);

            let eff = check_effectiveness_unit(unit, item.unwrap()); 
            if eff > 1 {
                let atk_type = this.title_atk.get_text().to_string();
                let atk_value = this.value_atk.get_text().to_string();
                let atk: Result<i32, _> = atk_value.parse();
                if atk.is_ok() {
                    let base_atk = atk.unwrap() as i32;
                    let mut count = 0;
                    let mut eff_atk_str = "".to_string();
                    for x in 2..6 {
                        if eff & (1 << x) != 0 {
                            let eff_atk = base_atk + (x - 1)*power;
                            if count == 0 {  eff_atk_str = format!("{} ({}x)", eff_atk, x); }
                            else { eff_atk_str = format!("{} / {} ({}x)", eff_atk_str, eff_atk, x); }
                            count +=1;
                        }
                    }
                    power_string = format!(" {}: {} \n", atk_type, eff_atk_str ).into();
                    let final_str = concat_string!(might_str, Mess::get("MID_SYS_Eff").to_string(), power_string, text);
                    this.contexts_text.set_text(final_str.into(), true);
                    return;
                }
            }
            let final_str = concat_string!(might_str, text).into();
            this.contexts_text.set_text(final_str, true);
        }
    }
}


pub fn get_position_stack(unit: &Unit) -> i32 {
    if unit.has_sid("SID_劇毒".into()) { return 5; }
    else if unit.has_sid("SID_猛毒".into()) { return 3; }
    else if unit.has_sid("SID_毒".into()) { return 1; }    
    return 0;
}
pub fn check_effectiveness_unit(unit: &Unit, item: &UnitItem) -> i32 {
    let mut result = 0;
    if item.flags & 1 == 1 {
        if let Some(item_equip_skills) = item.get_equipped_skills() {
            item_equip_skills.list.item.iter()
                .for_each(|skill_entity| {
                    if let Some(skill) = skill_entity.get_skill() {
                        let eff = skill.get_efficacy_value();
                        if eff > 1 { result |= 1 << eff; }
                    }
                }
            );
        }
    }
    if let Some(mask) = unit.mask_skill {
        mask.iter()
            .filter(|skill| skill.get_category() != 3 && skill.get_category() != 4 && skill.get_skill().is_some_and(|s| s.get_efficacy_value() > 1))
            .for_each(|skill|{
                let eff = skill.get_skill().unwrap().get_efficacy_value();
                if eff > 1 { result |= 1 << eff; }
            }
        );
    }
    result 
}