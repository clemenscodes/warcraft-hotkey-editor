use super::components::active_hero_level_option::ActiveHeroLevelOptionProps;
use super::components::idle_hero_level_option::IdleHeroLevelOptionProps;
use super::props::HeroLevelOptionProps;
use dioxus::prelude::*;

/// The option's shaped view: whether it is the active level, its label, and the
/// select handler. The dispatcher reads `is_active` to pick the look, then builds the
/// chosen variant's props.
pub(super) struct HeroLevelOptionPresentation {
    is_active: bool,
    label: String,
    onclick: EventHandler<MouseEvent>,
}

impl HeroLevelOptionPresentation {
    pub(super) fn is_active(&self) -> bool {
        self.is_active
    }
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

impl From<&HeroLevelOptionPresentation> for ActiveHeroLevelOptionProps {
    fn from(presentation: &HeroLevelOptionPresentation) -> Self {
        let label = presentation.label.clone();
        let onclick = presentation.onclick;
        Self { label, onclick }
    }
}

impl From<&HeroLevelOptionPresentation> for IdleHeroLevelOptionProps {
    fn from(presentation: &HeroLevelOptionPresentation) -> Self {
        let label = presentation.label.clone();
        let onclick = presentation.onclick;
        Self { label, onclick }
    }
}
