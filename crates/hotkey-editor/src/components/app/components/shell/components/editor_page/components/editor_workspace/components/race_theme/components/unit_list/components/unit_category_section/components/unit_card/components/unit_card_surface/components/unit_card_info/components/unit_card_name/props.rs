use dioxus::prelude::*;

/// The unit's display name, plus whether its card is selected (which paints it
/// white in the mobile carousel tile).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardNameProps {
    #[props(into)]
    pub text: String,
    pub is_selected: bool,
}
