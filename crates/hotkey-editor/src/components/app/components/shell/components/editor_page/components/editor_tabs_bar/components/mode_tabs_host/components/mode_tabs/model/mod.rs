use super::view::ModeTabsView;
use dioxus::prelude::*;
use warcraft_api::UnitMode;
use warcraft_api::UnitModeSelection;

#[derive(Props, Clone, Copy, PartialEq)]
pub struct ModeTabsModel {
    pub unit_modes: Signal<UnitModeSelection>,
    pub on_select: EventHandler<UnitMode>,
}

impl From<&ModeTabsView> for ModeTabsModel {
    fn from(view: &ModeTabsView) -> Self {
        let ModeTabsView {
            unit_modes,
            on_select,
        } = view.clone();
        Self {
            unit_modes,
            on_select,
        }
    }
}

impl ddd::Model for ModeTabsModel {
    type View = ModeTabsView;
}
