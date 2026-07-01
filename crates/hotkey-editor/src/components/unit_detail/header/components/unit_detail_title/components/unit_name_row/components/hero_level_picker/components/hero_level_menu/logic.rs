use super::components::hero_level_option::HeroLevelOptionProps;
use super::props::HeroLevelMenuProps;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

/// The ten level options, each finished with its index, the current level, and the
/// signals it writes on select.
pub(super) fn hero_level_options(props: &HeroLevelMenuProps) -> Vec<HeroLevelOptionProps> {
    (1..=MAX_HERO_LEVEL_DISPLAY)
        .map(|level_index| {
            let current_level = props.current_level;
            let selected_hero_level = props.selected_hero_level;
            let level_picker_open = props.level_picker_open;
            HeroLevelOptionProps {
                level_index,
                current_level,
                selected_hero_level,
                level_picker_open,
            }
        })
        .collect()
}
