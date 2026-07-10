use super::view::UnitOverridePanelView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::logic::UnitOverrideTarget;
use dioxus::prelude::*;

/// The right column holding the hotkey override: the "Hotkey override" heading over the
/// override card. On phones it becomes a sticky bottom sheet, widened and shifted out of
/// the card's padding.
#[derive(Props, Clone, PartialEq)]
pub struct UnitOverridePanelProps {
    pub(crate) override_target: UnitOverrideTarget,
}

impl From<&UnitOverridePanelView> for UnitOverridePanelProps {
    fn from(view: &UnitOverridePanelView) -> Self {
        let UnitOverridePanelView { override_target } = view.clone();
        Self { override_target }
    }
}

impl ddd::Props for UnitOverridePanelProps {
    type View = UnitOverridePanelView;
}
