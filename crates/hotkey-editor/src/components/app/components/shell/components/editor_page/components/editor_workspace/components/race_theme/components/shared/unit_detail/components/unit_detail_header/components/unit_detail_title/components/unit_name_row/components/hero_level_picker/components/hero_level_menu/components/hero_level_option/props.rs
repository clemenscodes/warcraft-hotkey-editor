use dioxus::prelude::*;

/// One selectable hero level in the dropdown: which level it offers, and the menu's
/// open signal it closes when chosen. Whether it is the current level and the level it
/// writes are read from editor context, so the selected-level signal is not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelOptionProps {
    pub level_index: u32,
    pub level_picker_open: Signal<bool>,
}
