use std::fmt;

use warcraft_api::{SystemKeybind, SystemKeybindClass};

use crate::db::WARCRAFT_SYSTEM_KEYBINDS;

const INVENTORY_SLOT_IDS: [&str; 6] = ["itm1", "itm2", "itm3", "itm4", "itm5", "itm6"];
const HERO_SELECTION_IDS: [&str; 3] = ["her1", "her2", "her3"];
const CONTROL_GROUP_IDS: [&str; 10] = [
    "Ctr1", "Ctr2", "Ctr3", "Ctr4", "Ctr5", "Ctr6", "Ctr7", "Ctr8", "Ctr9", "Ctr0",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemHotkeysCategory {
    Inventory,
    HeroSelection,
    ControlGroups,
    GeneralCommands,
    Menu,
    Camera,
    Observer,
    Replay,
}

impl SystemHotkeysCategory {
    pub const ALL: [SystemHotkeysCategory; 8] = [
        SystemHotkeysCategory::Inventory,
        SystemHotkeysCategory::HeroSelection,
        SystemHotkeysCategory::ControlGroups,
        SystemHotkeysCategory::GeneralCommands,
        SystemHotkeysCategory::Menu,
        SystemHotkeysCategory::Camera,
        SystemHotkeysCategory::Observer,
        SystemHotkeysCategory::Replay,
    ];

    pub fn entries(self) -> Vec<&'static SystemKeybind> {
        match self {
            SystemHotkeysCategory::Inventory => Self::collect_in_order(&INVENTORY_SLOT_IDS),
            SystemHotkeysCategory::HeroSelection => Self::collect_in_order(&HERO_SELECTION_IDS),
            SystemHotkeysCategory::ControlGroups => Self::collect_in_order(&CONTROL_GROUP_IDS),
            SystemHotkeysCategory::GeneralCommands => Self::collect_general_commands(),
            SystemHotkeysCategory::Menu => Self::collect_by_class(SystemKeybindClass::Menu),
            SystemHotkeysCategory::Camera => Self::collect_by_class(SystemKeybindClass::Camera),
            SystemHotkeysCategory::Observer => Self::collect_by_class(SystemKeybindClass::Observer),
            SystemHotkeysCategory::Replay => Self::collect_by_class(SystemKeybindClass::Replay),
        }
    }

    fn collect_in_order(section_ids: &'static [&'static str]) -> Vec<&'static SystemKeybind> {
        let mut ordered: Vec<&'static SystemKeybind> = Vec::with_capacity(section_ids.len());
        for wanted_id in section_ids {
            for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
                if entry.section_id() == *wanted_id {
                    ordered.push(entry);
                    break;
                }
            }
        }
        ordered
    }

    fn collect_by_class(class: SystemKeybindClass) -> Vec<&'static SystemKeybind> {
        WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .filter(|entry| entry.class() == class)
            .collect()
    }

    fn collect_general_commands() -> Vec<&'static SystemKeybind> {
        WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .filter(|entry| {
                if entry.class() != SystemKeybindClass::Game {
                    return false;
                }
                let id = entry.section_id();
                !INVENTORY_SLOT_IDS.contains(&id) && !HERO_SELECTION_IDS.contains(&id)
            })
            .collect()
    }
}

impl fmt::Display for SystemHotkeysCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SystemHotkeysCategory::Inventory => "Inventory",
            SystemHotkeysCategory::HeroSelection => "Hero Selection",
            SystemHotkeysCategory::ControlGroups => "Control Groups",
            SystemHotkeysCategory::GeneralCommands => "General Commands",
            SystemHotkeysCategory::Menu => "Menu",
            SystemHotkeysCategory::Camera => "Camera",
            SystemHotkeysCategory::Observer => "Observer Mode",
            SystemHotkeysCategory::Replay => "Replay",
        };
        formatter.write_str(label)
    }
}
