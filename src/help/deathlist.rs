use super::*;
use engage::{random::Random, force::*};

#[unity::class("App", "LoadingLogo")]
pub struct LoadingLogo {
   __: [u8; 0x60],
   pub title_text: &'static mut TextMeshProUGUI,
   pub tips_text: &'static mut TextMeshProUGUI,
}

#[unity::hook("App", "LoadingLogo", "SetTipsData")]
pub fn loading_set_tip_text_hook(this: &mut LoadingLogo, tips: u64, method_info: OptionalMethod){
   let force = Force::get(ForceType::Dead);
   call_original!(this, tips, method_info);
   let rng = Random::get_game();
   if rng.get_value(2) < 1 { return; }
   if force.is_none() { return; }
   let dead_force = force.unwrap();
   let count = dead_force.get_count();
   if count == 0 { return; }
   let mut string_dead = format!("{} Dead", count);
   this.title_text.set_text( format!("{} Dead", count).into(), true);
   let mut force_iter = Force::iter(dead_force);
   let mut unit_count = 0;
   while let Some(unit) = force_iter.next() {
        let name = Mess::get_name(unit.person.pid);
        if let Some(dead) = unit.record.get_dead_chapter() {
            let prefix = Mess::get(format!("{}_PREFIX", dead.name)).to_string();
            let dead_chapter_name = Mess::get(dead.name);

            if unit_count != 0 { string_dead = format!("{}\n{} - {}: {}", string_dead, name, prefix, dead_chapter_name);}
            else { string_dead = format!("{} - {}: {}", name, prefix, dead_chapter_name); }
        }
        else {
            if unit_count != 0 {
                if unit_count % 2 == 0 { string_dead = format!("{} \n {}", string_dead, name); }
                else { string_dead = format!("{} - {}", string_dead, name); }
            }
            else { string_dead = name.to_string(); }
        }
        unit_count += 1;
    }
    this.tips_text.set_text(string_dead.into(), true);
}
