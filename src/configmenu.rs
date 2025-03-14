use engage::script::EventScriptCommand;
pub use unity::prelude::*;
pub use skyline::patching::Patch;
pub use engage::{
    menu::{BasicMenuResult, BasicMenuItemAttribute, BasicMenuItem, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    gamevariable::*,
    mess::*,
    script::EventScript,
    dialog::yesno::*,
    gamedata::{GamedataArray, item::RewardData},
};

pub mod bgm;
pub mod map;
pub mod support;
pub mod cutscene;
pub mod rng;
//pub mod cook;
pub mod level;
pub mod smash;
pub mod exp;
pub mod gift;
pub mod gender;
pub mod dlc;
pub mod fish;
pub mod ai;
pub mod growth;

pub const LEVEL_DIS_KEY: &str = "G_LEVEL_TYPE";
pub const GROWTH_KEY: &str = "G_GROWTH_TYPE";
pub const GIFT_KEY: &str = "G_GIFT";
pub const RNG_KEY: &str = "G_RNG_TYPE";
pub const SMASH_KEY: &str = "G_Smash_Attacks";
pub const BGM_KEY: &str = "G_BGM";
pub const COOK_KEY: &str = "G_CHEF";
pub const SUPPORT_KEY: &str = "G_SUPPORT_TYPE";
pub const EXP_KEY: &str = "G_EXP_TYPE";
pub const MAP_KEY: &str = "G_MAP_SKIP";
pub const CUTSCENES_KEY: &str = "G_CUTSCENE";
pub const DLC_COMPLETE: &str = "G_DLC_Complete";
pub const AI_KEY: &str = "G_AI";
pub const FISH_KEY: &str = "G_AutoFish";

pub fn register_code_mod_keys() {
    GameVariableManager::make_entry(LEVEL_DIS_KEY, 1);
    GameVariableManager::make_entry(GROWTH_KEY, 0);
    GameVariableManager::make_entry(BGM_KEY, 0);
    GameVariableManager::make_entry(RNG_KEY, 0);
    GameVariableManager::make_entry(SMASH_KEY, 0);
    GameVariableManager::make_entry(COOK_KEY, 0);
    GameVariableManager::make_entry(GIFT_KEY, 0);
    GameVariableManager::make_entry(EXP_KEY, 0);
    GameVariableManager::make_entry(CUTSCENES_KEY, 1);
    GameVariableManager::make_entry(SUPPORT_KEY, 0);
    GameVariableManager::make_entry(MAP_KEY, 0);
    GameVariableManager::make_entry(SMASH_KEY, 0);
    GameVariableManager::make_entry(AI_KEY, 0);
    GameVariableManager::make_entry(FISH_KEY, 0);    
    GameVariableManager::make_entry(DLC_COMPLETE, 0);
    GameVariableManager::make_entry("G_Chef_Index", 0);
    GameVariableManager::make_entry("G_JobGrowth", 0);
    GameVariableManager::make_entry("G_DLC_Complete2", 0);
    println!("My Code Mods variables are created");
}

pub fn initialize_and_install() {
    rng::intialize_hybrid_table();
    gender::gender_install();
    bgm::bgm_install(); 
    cutscene::cutscene_install(); //Cutscenes
    support::support_install(); //Support
    map::map_mod_install(); // Map Dialogue/Tutorials
    ai::ai_install();   // AI + Support Info
    // cook::cook_install(); // Cooking Outcomes
    level::level_install(); //Level Display and Growth Type
    rng::rng_install(); // RNG 
    fish::fishing_install();
    gift::gift_install(); //Gift Settings
    dlc::dlc_map_install();
    exp::exp_install(); 
    growth::growth_install();
    smash::install_smash();
    println!("Finish with ConfigMenu");
}

pub fn scene_loading_event_update() {
    register_code_mod_keys();
    dlc::complete_dlc_chapters();
    map::clear_all_tutorials();
    support::update_reliances_event();
    gender::get_lueur_name_gender();
}

pub extern "C" fn patch_code_mods(script: &EventScript) {
    println!("MyCodeMods Script Functions");
    register_code_mod_keys();
    script.register_action("Movie", cutscene::movie);
    script.register_action("PuppetDemo", cutscene::puppet_demo);

    gift::patch_gift();
    support::patch_support();
    // cutscene::patch_cutscenes();
    map::patch_map();
    smash::patch_smash();
    ai::patch_ignorance();
    bgm::patch_imm_bgm();
    growth::patch_growth();
    // cook::patch_cook();

    if RewardData::get_list().is_some_and(|list| list.len() < 4) || RewardData::get_list().is_none() { Patch::in_text(0x023f3c00).bytes(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap(); }
    else { Patch::in_text(0x023f3c00).bytes(&[0xff, 0xc3, 0x01, 0xd1]).unwrap(); }
}

pub fn install_config_hooks() {
    skyline::install_hooks!(
        support::reliance_can_level_up_hook,
        rng::calc_action_hook,
        rng::probability_100_hook, 
        rng::battle_math_hook,
        exp::exp_sequence_create_bind_hook
    );
    println!("Finish with installing ConfigMenu Hook");
}


/*
ai::help_param_setter_set_person,
character::set_tip_text, 
string::help_param_setter_set_item_data_hook,
evel::level_up_window_setup_hook,
string::help_param_setter_set_god
set_job_details,
 level::set_total_level,
 exp::set_battle_info_hook
level::unit_info_set_level_hook
*/
// menus::hub::hub_menu_create_bind);