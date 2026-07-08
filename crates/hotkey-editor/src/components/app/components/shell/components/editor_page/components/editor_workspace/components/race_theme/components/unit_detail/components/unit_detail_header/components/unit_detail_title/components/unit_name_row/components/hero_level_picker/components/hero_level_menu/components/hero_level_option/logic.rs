use super::props::HeroLevelOptionProps;
use dioxus::prelude::*;

/// The option's shaped view: whether it is the active level, its label, and the
/// select handler.
pub(super) struct HeroLevelOptionPresentation {
    pub(super) is_active: bool,
    pub(super) label: String,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&HeroLevelOptionProps> for HeroLevelOptionPresentation {
    fn from(props: &HeroLevelOptionProps) -> Self {
        let level_index = props.level_index;
        let is_active = level_index == props.current_level;
        let label = format!("Level {level_index}");
        let mut selected_hero_level = props.selected_hero_level;
        let mut level_picker_open = props.level_picker_open;
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
