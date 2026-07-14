use super::model::HeroLevelMenuModel;
use dioxus::prelude::*;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

pub(super) struct HeroLevelMenuOption {
    pub(super) level_index: u32,
    pub(super) level_picker_open: Signal<bool>,
}

pub(super) fn hero_level_options(props: &HeroLevelMenuModel) -> Vec<HeroLevelMenuOption> {
    (1..=MAX_HERO_LEVEL_DISPLAY)
        .map(|level_index| {
            let level_picker_open = props.level_picker_open;
            HeroLevelMenuOption {
                level_index,
                level_picker_open,
            }
        })
        .collect()
}
