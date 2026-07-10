use super::components::hero_level_option::HeroLevelOptionProps;
use super::props::HeroLevelMenuProps;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

/// The ten level options, each finished with its index and the menu's open signal it
/// closes on select. Each option reads the selected level from context itself.
pub(super) fn hero_level_options(props: &HeroLevelMenuProps) -> Vec<HeroLevelOptionProps> {
    (1..=MAX_HERO_LEVEL_DISPLAY)
        .map(|level_index| {
            let level_picker_open = props.level_picker_open;
            HeroLevelOptionProps {
                level_index,
                level_picker_open,
            }
        })
        .collect()
}
