use super::logic::{HeroLevelOptionInputs, HeroLevelOptionPresentation};
use super::props::HeroLevelOptionProps;
use crate::services::editor_state::context::use_editor_state;

/// Reads the selected hero level from context and shapes the option's presentation:
/// whether it is the active level, its label, and the select handler that writes the
/// level and closes the menu.
pub(super) fn use_hero_level_option(props: &HeroLevelOptionProps) -> HeroLevelOptionPresentation {
    let selected_hero_level = use_editor_state().selected_hero_level();
    let inputs = HeroLevelOptionInputs {
        level_index: props.level_index,
        selected_hero_level,
        level_picker_open: props.level_picker_open,
    };
    HeroLevelOptionPresentation::from(inputs)
}
