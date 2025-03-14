use unity::prelude::*;
use engage::gamedata::{Gamedata, StructBaseFields};

#[unity::class("App", "FoodstuffData")]
pub struct FoodstuffData {
    pub parent: StructBaseFields,
    pub iid: &'static Il2CppString,
}
impl Gamedata for FoodstuffData {}

#[unity::class("App", "MascotFoodData")]
pub struct MascotFoodData {
    pub parent: StructBaseFields,
    pub iid: &'static Il2CppString,
    pub value: i32,
}
impl Gamedata for MascotFoodData {}

#[unity::class("App", "FishingFishData")]
pub struct FishingFishData {
    pub parent: StructBaseFields,
    pub name: &'static Il2CppString,
    idc: [u8; 0x18],
    pub food_type: &'static Il2CppString,
    pub bond: i32,
    idc_floats: [f32; 8],
    pub catch_time: f32,
    pub catch_time_rnd_add: f32,
    pub escape_time: f32,
    pub hp: f32,
    pub lethal_hp: f32,
    pub regen_per_sec: f32,
}
impl Gamedata for FishingFishData {}


#[unity::class("App", "FishingTargetListData")]
pub struct FishingTargetListData {
    pub parent: StructBaseFields,
    pub id: &'static Il2CppString,
    pub fish_id: &'static Il2CppString,
    pub priority: i32,
}
impl Gamedata for FishingTargetListData {}