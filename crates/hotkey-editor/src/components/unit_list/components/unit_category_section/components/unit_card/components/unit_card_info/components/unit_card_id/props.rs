use super::state::UnitCardIdState;
use dioxus::prelude::*;

/// The unit id text plus whether its card is selected (which tints it).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIdProps {
    #[props(into)]
    pub text: String,
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
