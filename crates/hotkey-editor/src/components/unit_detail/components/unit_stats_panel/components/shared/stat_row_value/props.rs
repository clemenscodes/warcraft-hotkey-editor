use dioxus::prelude::*;

/// A stat row's value. Absent when the row carries a gain instead, so the row
/// renders it as a self-guarding slot. `is_zero` mutes it (used for absent mana).
#[derive(Props, Clone, PartialEq)]
pub struct StatRowValueProps {
    #[props(default)]
    pub text: Option<String>,
    #[props(default)]
    pub is_zero: bool,
}
