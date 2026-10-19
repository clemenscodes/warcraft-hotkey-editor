use super::model::HeroLevelOptionModel;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;

pub(super) struct HeroLevelOptionPresentation {
    pub(super) is_active: bool,
    pub(super) label: String,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for HeroLevelOptionPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for HeroLevelOptionPresentation {
    type Model = HeroLevelOptionModel;
}

pub(super) fn use_hero_level_option(props: &HeroLevelOptionModel) -> HeroLevelOptionPresentation {
    let mut selected_hero_level = use_editor_state().selected_hero_level();
    let level_index = props.level_index;
    let is_active = level_index == *selected_hero_level.read();
    let label = format!("Level {level_index}");
    let on_close = props.on_close;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        selected_hero_level.set(level_index);
        on_close.call(());
    });
    HeroLevelOptionPresentation {
        is_active,
        label,
        onclick,
    }
}
