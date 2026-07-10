use super::view::HeroLevelMenuView;
use dioxus::prelude::*;

/// The hero-level dropdown menu: the open signal its options close when chosen. Each
/// option reads the selected level from context to mark the active one and to write.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelMenuProps {
    pub level_picker_open: Signal<bool>,
}

impl From<&HeroLevelMenuView> for HeroLevelMenuProps {
    fn from(view: &HeroLevelMenuView) -> Self {
        let HeroLevelMenuView { level_picker_open } = view.clone();
        Self { level_picker_open }
    }
}

impl ddd::Props for HeroLevelMenuProps {
    type View = HeroLevelMenuView;
}
