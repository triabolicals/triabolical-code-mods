use engage::{
    gamevariable::GameVariableManager,
    gameuserdata::GameUserData,
    gamedata::{Gamedata, dispos::ChapterData},
    sequence::mainmenusequence::MainMenuSequenceLabel,
};
use cobapi::{Event, SystemEvent};

const TITLE_SEQUENCE: i32 = -988690862;
const MAINMENU_SEQUENCE: i32 = -1912552174;
const PROC_SCENE: i32 = -1118443598;
const MAP_HUMAN_SEQUENCE: i32 = 1525873615;

pub extern "C" fn create_settings_install_menu_hooks(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::ProcInstJump {proc, label } => {
                let proc_label = *label;
                if proc.hashcode == MAINMENU_SEQUENCE {
                    if proc_label == MainMenuSequenceLabel::ExecuteGameStart as i32 {
                        GameUserData::set_chapter(ChapterData::get("CID_M001").unwrap());
                        GameVariableManager::set_bool("G_Cleared_M000", true);
                        crate::configmenu::register_code_mod_keys();
                        crate::configmenu::gender::get_lueur_name_gender();
                    }
                }
                else if proc.hashcode == TITLE_SEQUENCE && *label == 0 {
                    crate::hub::music::add_to_juke_box();
                    crate::misc::set_global_completed();
                    crate::help::help_menu_call_install();
                    crate::hub::quickmenu::hub_menu_build_attrs_install();
                    crate::mapsave::map_save_menu_edits();
                    crate::gmapshop::gmapmenu_shop_acall_install();
                    crate::sortie::sortie_menu_installs();
                }
                else if proc.hashcode == PROC_SCENE {
                    crate::configmenu::scene_loading_event_update();
                }
                else if proc.hashcode == MAP_HUMAN_SEQUENCE { crate::mapsave::map_save_proc_edit(proc);  }
            }
            _ => {}
        }
    }
}