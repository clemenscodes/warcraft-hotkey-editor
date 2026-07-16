use super::view::UnitDetailBodyView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::presentation::UnitCommandGridSlots;
use crate::services::customkeys::queries::unit_override_target_query::UnitOverrideTargetView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailBodyModel {
    pub(crate) grid_slots: UnitCommandGridSlots,
    pub(crate) override_target: UnitOverrideTargetView,
}

impl From<&UnitDetailBodyView> for UnitDetailBodyModel {
    fn from(view: &UnitDetailBodyView) -> Self {
        let UnitDetailBodyView {
            grid_slots,
            override_target,
        } = view.clone();
        Self {
            grid_slots,
            override_target,
        }
    }
}

impl ddd::Model for UnitDetailBodyModel {
    type View = UnitDetailBodyView;
}
