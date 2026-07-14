use super::view::HotkeyOverrideSectionView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::presentation::UnitOverrideTarget;
use dioxus::prelude::*;

/// The section holding the hotkey override: the "Hotkey override" heading over the
/// override card, flowing below the command grids on every band.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideSectionModel {
    pub(crate) override_target: UnitOverrideTarget,
}

impl From<&HotkeyOverrideSectionView> for HotkeyOverrideSectionModel {
    fn from(view: &HotkeyOverrideSectionView) -> Self {
        let HotkeyOverrideSectionView { override_target } = view.clone();
        Self { override_target }
    }
}

impl ddd::Model for HotkeyOverrideSectionModel {
    type View = HotkeyOverrideSectionView;
}
