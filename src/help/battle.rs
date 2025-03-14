use super::*;
use crate::configmenu::RNG_KEY;
use engage::random::Random;

//To Display Growth Rates on help stat text
#[unity::hook("App", "HelpParamSetter", "SetBattleInfo")]
pub fn set_battle_info_hook(this: u64, engine: u64, unit: &mut Unit, text_type: i32, text: &Il2CppString, method_info: OptionalMethod){
    if text_type == 3 && GameVariableManager::get_number(RNG_KEY) == 5 {
        let rng_queue = rng_queue_string();
        call_original!(this, engine, unit, text_type, concat_string!(text.to_string(), "\n", rng_queue).into(), method_info);
        return;
    }
    else if text_type == 5 && GameVariableManager::get_number(RNG_KEY) == 5 {
        let game_rng = Random::get_game();
        call_original!(this, engine, unit, text_type, text, method_info); 
        game_rng.get_value(100);
        return;
    }
    if unit.force.is_none() { 
        call_original!(this, engine, unit, text_type, text, method_info); 
        return; 
    }
    let force = unit.force.unwrap().force_type;
    let stat_index = match text_type {
        0 => 0,
        8 => 1,
        10 => 2,
        7 => 3,
        14 => 4,
        12 => 5,
        9 => 6,
        13 => 7,
        15 => 8,
        _ => -1,
    };
    if stat_index == -1 { return; } // text is not a stat
    let grow = unit.get_capability_grow(stat_index, false);   // total growth rate
    let grow2 = unit.get_capability_grow(stat_index, true);   // personal growth rate
    let mut oko = String::new();
    let mut growth_str = String::new();
    if text_type == 0 && force < 3 {
        let hp = unit.get_hp();
        let posion = super::item::get_position_stack(unit);
        let phys = hp + unit.get_capability(5, true) - posion;
        let phys2 = phys + unit.get_capability(5, true) - posion;
        let phys4 = phys + 3*unit.get_capability(5, true) - 3*posion;

        let phys_str = format!("\n{} {}: {} / {} / {}", Mess::get("MID_SYS_Weapon_Attack"), Mess::get("MID_SYS_Dmg"),
            phys, phys2 / 2 + (phys2 % 2).signum(), phys4 / 4 + (phys4 % 4).signum());

        let mag = hp + unit.get_capability(7, true) - posion;
        let mag2 = mag + unit.get_capability(7, true) - posion;
        let mag4 = mag + 3*unit.get_capability(7, true) - 3*posion;

        let mag_str = format!("\n{} {}: {} / {} / {}", Mess::get("MID_SYS_Magic_Attack").to_string(), Mess::get("MID_SYS_Dmg").to_string(), 
            mag, mag2 / 2 + (mag2 % 2).signum(), mag4 / 4 + (mag4 % 4).signum());

        oko = concat_string!(phys_str, mag_str).into();
    }
    if ( force == 0 || force == 3 ) && grow > 0  {
        let mut level_str = format!(": {}%", grow);
        if grow2 != grow {
            if grow-grow2 < 0 { level_str = format!(": {}%", grow).into(); }
            else { level_str = format!(": {}% ({}% + {}%)", grow, grow2, grow-grow2); }
        }
        growth_str = concat_string!("\n", Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE").to_string(), level_str);
    }
    if GameVariableManager::get_number(crate::configmenu::LEVEL_DIS_KEY) == 1 {
        let next_level = predict_level_up(unit);
        if next_level[ stat_index as usize] > 0 {
            growth_str = format!("{}\nNext Lvl: +{}", growth_str, next_level[ stat_index as usize]).into();
        }

    }
    call_original!(this, engine, unit, text_type, concat_string!(text.to_string(), oko, growth_str).into(), method_info);
}

fn rng_queue_string() -> String {
    let game_rng = Random::get_game();
    let rng = Random::instantiate().unwrap();
    let rng_hybrid = Random::instantiate().unwrap();
    rng.copy_from(game_rng);
    rng_hybrid.copy_from(game_rng);
    let mut rng_str = "1RN:\t".to_string();
    let mut rng_hybrid_str = "HRN:\t".to_string();
    for _x in 0..25 {
        let rng_v = rng.get_value(100000) / 1000;
        let rng_h = rng_hybrid.get_value(10000) / 100;
        rng_hybrid_str = format!("{} {}", rng_hybrid_str, crate::configmenu::rng::get_hybrid_to_display(rng_h));
        rng_str = format!("{} {}", rng_str, rng_v).to_string(); 
    }
    format!("\n{}\n{}", rng_str, rng_hybrid_str)
}

fn predict_level_up(unit: &mut Unit) -> [i8; 10] {
    let mut base_cap: [i8; 10] = [0; 10];
    let mut level_cap: [i8; 10] = [0; 10];
    let mut growth_cap: [u8; 10] = [0; 10];
    for x in 0..10 {
        level_cap[x] = unit.level_capability.capability[x];
        base_cap[x] = unit.base_capability.capability[x];
        growth_cap[x] = unit.grow_capability[x];
    }

    let mut out: [i8; 10] = [0; 10];
    let old_level = unit.level;
    unit.set_level(1);
    let seed = unit.grow_seed;
    unit.level_up(2);
    for x in 0..10 {
        out[x] = unit.base_capability.capability[x] - base_cap[x];
        unit.set_base_capability(x as i32, base_cap[x] as i32);
        unit.level_capability.capability[x] = level_cap[x];
        unit.grow_capability[x] = growth_cap[x];
    } 
    unit.set_level(old_level as i32);
    unit.grow_seed = seed;
    return out;
}