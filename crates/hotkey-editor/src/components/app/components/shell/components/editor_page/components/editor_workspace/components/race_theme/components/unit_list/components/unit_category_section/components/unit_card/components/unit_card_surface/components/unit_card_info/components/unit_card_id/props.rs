use super::state::UnitCardIdState;
use dioxus::prelude::*;
use warcraft_api::{Race, WarcraftObjectId};

/// The unit id, the race whose accent it takes when selected, and whether its
/// card is selected (which tints it).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIdProps {
    pub unit_id: WarcraftObjectId,
    pub race: Race,
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
