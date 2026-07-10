use super::props::HeroLevelMenuProps;
use dioxus::prelude::*;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

/// One entry the menu offers: the level it selects and the menu's open signal it closes
/// on select. Each option reads the selected level from context itself.
pub(super) struct HeroLevelMenuOption {
    pub(super) level_index: u32,
    pub(super) level_picker_open: Signal<bool>,
}

/// The ten level options, each finished with its index and the menu's open signal.
pub(super) fn hero_level_options(props: &HeroLevelMenuProps) -> Vec<HeroLevelMenuOption> {
    (1..=MAX_HERO_LEVEL_DISPLAY)
        .map(|level_index| {
            let level_picker_open = props.level_picker_open;
            let option = HeroLevelMenuOption {
                level_index,
                level_picker_open,
            };
            option
        })
        .collect()
}
