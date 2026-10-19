use super::model::HeroLevelMenuModel;
use dioxus::prelude::*;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

pub(super) struct HeroLevelMenuOption {
    pub(super) level_index: u32,
    pub(super) on_close: EventHandler<()>,
}

pub(super) fn hero_level_options(props: &HeroLevelMenuModel) -> Vec<HeroLevelMenuOption> {
    (1..=MAX_HERO_LEVEL_DISPLAY)
        .map(|level_index| {
            let on_close = props.on_close;
            HeroLevelMenuOption {
                level_index,
                on_close,
            }
        })
        .collect()
}
