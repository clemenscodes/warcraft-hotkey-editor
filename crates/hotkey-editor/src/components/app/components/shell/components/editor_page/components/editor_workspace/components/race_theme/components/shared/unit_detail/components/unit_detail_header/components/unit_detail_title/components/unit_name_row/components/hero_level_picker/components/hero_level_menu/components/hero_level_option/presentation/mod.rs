use super::model::HeroLevelOptionModel;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;

pub(super) struct HeroLevelOptionInputs {
    pub(super) level_index: u32,
    pub(super) selected_hero_level: Signal<u32>,
    pub(super) level_picker_open: Signal<bool>,
}

pub(super) struct HeroLevelOptionPresentation {
    is_active: bool,
    label: String,
    onclick: EventHandler<MouseEvent>,
}

impl HeroLevelOptionPresentation {
    pub(super) fn is_active(&self) -> bool {
        self.is_active
    }

    pub(super) fn label(&self) -> &str {
        &self.label
    }

    pub(super) fn onclick(&self) -> EventHandler<MouseEvent> {
        self.onclick
    }
}

impl From<HeroLevelOptionInputs> for HeroLevelOptionPresentation {
    fn from(inputs: HeroLevelOptionInputs) -> Self {
        let level_index = inputs.level_index;
        let mut selected_hero_level = inputs.selected_hero_level;
        let is_active = level_index == *selected_hero_level.read();
        let label = format!("Level {level_index}");
        let mut level_picker_open = inputs.level_picker_open;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            selected_hero_level.set(level_index);
            level_picker_open.set(false);
        });
        Self {
            is_active,
            label,
            onclick,
        }
    }
}

pub(super) fn use_hero_level_option(props: &HeroLevelOptionModel) -> HeroLevelOptionPresentation {
    let selected_hero_level = use_editor_state().selected_hero_level();
    let inputs = HeroLevelOptionInputs {
        level_index: props.level_index,
        selected_hero_level,
        level_picker_open: props.level_picker_open,
    };
    HeroLevelOptionPresentation::from(inputs)
}

impl ddd::Presentation for HeroLevelOptionPresentation {
    type Model = HeroLevelOptionModel;
}
