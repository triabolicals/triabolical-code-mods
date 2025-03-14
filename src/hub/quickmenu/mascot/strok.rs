use super::*;

pub extern "C" fn add_strok_pts(this: &'static mut MascotMenuSequence, _method_info: OptionalMethod) {
    if !HubVariableMascot::done_strok() { 
       HubVariableMascot::add_point(5); 
       this.mascot_friendly_gague.try_popup();
   } 
}

pub fn strok_a_call(item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !item.is_attribute_disable() {
        if !HubVariableMascot::done_strok() { HubVariableMascot::add_point(5); }
        unsafe { original_strok_a_call(item, None) }
    }
    else { BasicMenuResult::se_miss() }
}

pub fn strok_build_attr(_item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if HubVariableMascot::done_strok() { BasicMenuItemAttribute::Disable }
    else { BasicMenuItemAttribute::Enable }
}


#[skyline::from_offset(0x0208d3b0)]
fn original_strok_a_call(_item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult;