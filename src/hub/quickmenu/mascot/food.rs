use engage::proc::ProcInstFields;

use super::*;

pub extern "C" fn mascot_create_food_eat_bind(this: &'static mut MascotMenuSequence, _method_info: OptionalMethod) {
    if let Some(food_data) = this.food_stuff {
        let food_eat = MascotFoodEatSequence::instantiate().unwrap();
        unsafe { mascot_food_eat_ctor(food_eat, food_data, None); }
        let descs = Il2CppArray::from_slice(create_mascot_eat_desc(food_eat)).unwrap();
        food_eat.create_bind(this, descs, "MascotEattingFood");
    }
}


extern "C" fn mascot_done_eating_food(_this: &'static mut MascotFoodEatSequence, _method_info: OptionalMethod) { HubVariableMascot::done_food(); }
extern "C" fn mascot_eat_food(this: &'static mut MascotFoodEatSequence, _method_info: OptionalMethod) {
    if let Some(food) = this.food {
        if let Some(item) = ItemData::get(food.iid) {
            item.add_inventory(-1);
            if let Some(mascot_food) = MascotFoodData::get(food.iid) {
                HubVariableMascot::add_point(mascot_food.value);
                if let Some(menu) = engage::util::get_singleton_proc_instance::<MascotMenuSequence>() { menu.mascot_friendly_gague.try_popup(); }
            }
        }
    }
}

fn create_mascot_eat_desc(this: &'static MascotFoodEatSequence) -> Vec<&'static mut ProcDesc> {
    let mascot_food_gain_bond_method = unsafe { std::mem::transmute(food_get_bond::get_ref().method_ptr) };
    vec![
        ProcDesc::call(ProcVoidMethod::new(this, mascot_food_gain_bond_method)),
        ProcDesc::call(ProcVoidMethod::new(this, mascot_eat_food)),
        unsafe { std::mem::transmute(ProcDesc::yiel().unwrap()) },
        ProcDesc::call(ProcVoidMethod::new(this, mascot_done_eating_food)),
        ProcDesc::end()
    ]
}

pub fn food_eat_build_attr(_item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
    if HubVariableMascot::is_done_food() { BasicMenuItemAttribute::Disable }
    else { BasicMenuItemAttribute::Enable }
}

pub fn food_eat_a_call(item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
    if !item.is_attribute_disable() { unsafe { original_food_eat_acall(item, None) } }
    else { BasicMenuResult::se_miss() }
}

#[unity::class("App", "MascotFoodEatSequence")]
pub struct MascotFoodEatSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_loaded: bool,
    pub food: Option<&'static FoodstuffData>,
}

impl Bindable for MascotFoodEatSequence {}

#[skyline::from_offset(0x02035390)]
fn mascot_food_eat_ctor(this: &MascotFoodEatSequence, food: &FoodstuffData, _method_info: OptionalMethod);

#[unity::from_offset("App", "MascotFoodEatSequence", "GetBond")]
fn food_get_bond(this: &'static MascotFoodEatSequence, _method_info: OptionalMethod);

#[skyline::from_offset(0x0208d2a0)]
fn original_food_eat_acall(item: &BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult;
