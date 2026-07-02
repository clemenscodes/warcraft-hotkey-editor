use dioxus::prelude::*;

/// A stat row's value. `is_zero` mutes it (used for absent mana / regen).
#[derive(Props, Clone, PartialEq)]
pub struct StatRowValueProps {
    #[props(into)]
    pub text: String,
    #[props(default)]
    pub is_zero: bool,
}
