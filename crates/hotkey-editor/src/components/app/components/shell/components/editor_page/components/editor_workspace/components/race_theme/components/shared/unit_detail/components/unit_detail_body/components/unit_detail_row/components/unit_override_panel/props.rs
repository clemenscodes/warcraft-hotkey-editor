use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::logic::UnitOverrideTarget;
use dioxus::prelude::*;

/// The right column holding the hotkey override: the "Hotkey override" heading over the
/// override card. On phones it becomes a sticky bottom sheet, widened and shifted out of
/// the card's padding.
#[derive(Props, Clone, PartialEq)]
pub struct UnitOverridePanelProps {
    pub override_target: UnitOverrideTarget,
}
