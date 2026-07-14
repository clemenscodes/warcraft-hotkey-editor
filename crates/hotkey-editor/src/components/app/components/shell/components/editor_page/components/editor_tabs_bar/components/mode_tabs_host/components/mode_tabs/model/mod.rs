use super::view::ModeTabsView;
use dioxus::prelude::*;
use warcraft_api::UnitMode;

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
