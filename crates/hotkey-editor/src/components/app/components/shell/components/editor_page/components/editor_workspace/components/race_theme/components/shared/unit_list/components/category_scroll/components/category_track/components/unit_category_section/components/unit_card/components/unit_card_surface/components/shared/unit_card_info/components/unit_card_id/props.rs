use super::state::UnitCardIdState;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id and whether its card is selected (which dispatches the selected look;
/// the accent colour is read from `--race-accent`).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIdProps {
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
}

impl UnitCardIdProps {
    pub(super) fn state(&self) -> UnitCardIdState {
        if self.is_selected {
            UnitCardIdState::Selected
        } else {
            UnitCardIdState::Normal
        }
    }
}
