use super::state::UnitCardIdState;
use super::view::UnitCardIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id and whether its card is selected (which dispatches the selected look;
/// the accent colour is read from `--race-color`).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIdModel {
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
}

impl UnitCardIdModel {
    pub(super) fn state(&self) -> UnitCardIdState {
        if self.is_selected {
            UnitCardIdState::Selected
        } else {
            UnitCardIdState::Normal
        }
    }
}

impl From<&UnitCardIdView> for UnitCardIdModel {
    fn from(view: &UnitCardIdView) -> Self {
        let UnitCardIdView {
            unit_id,
            is_selected,
        } = view.clone();
        Self {
            unit_id,
            is_selected,
        }
    }
}

impl ddd::Model for UnitCardIdModel {
    type View = UnitCardIdView;
}
