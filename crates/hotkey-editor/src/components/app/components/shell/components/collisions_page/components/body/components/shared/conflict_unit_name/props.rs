use dioxus::prelude::*;
/// A unit's name on a hotkey/position collision card.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictUnitNameProps {
    #[props(into)]
    pub text: String,
}
