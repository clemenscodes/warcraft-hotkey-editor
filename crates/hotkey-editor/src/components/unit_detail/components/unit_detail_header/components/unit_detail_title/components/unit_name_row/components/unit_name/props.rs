use dioxus::prelude::*;

/// The unit's display name.
#[derive(Props, Clone, PartialEq)]
pub struct UnitNameProps {
    pub text: &'static str,
}
