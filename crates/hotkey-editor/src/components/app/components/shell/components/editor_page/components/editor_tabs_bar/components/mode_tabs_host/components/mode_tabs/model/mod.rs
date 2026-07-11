use super::view::ModeTabsView;
use dioxus::prelude::*;
use warcraft_api::UnitMode;

/// The mode column's only concern: which mode is active (to mark the current button),
/// and a select handler to dispatch when a mode is chosen. The cascade the selection
/// triggers — default unit, slot reset — is the domain's job behind `select_mode`.
#[derive(Props, Clone, Copy, PartialEq)]
pub struct ModeTabsModel {
    pub unit_mode: Signal<UnitMode>,
    pub on_select: EventHandler<UnitMode>,
}

impl From<&ModeTabsView> for ModeTabsModel {
    fn from(view: &ModeTabsView) -> Self {
        let ModeTabsView {
            unit_mode,
            on_select,
        } = view.clone();
        Self {
            unit_mode,
            on_select,
        }
    }
}

impl ddd::Model for ModeTabsModel {
    type View = ModeTabsView;
}
