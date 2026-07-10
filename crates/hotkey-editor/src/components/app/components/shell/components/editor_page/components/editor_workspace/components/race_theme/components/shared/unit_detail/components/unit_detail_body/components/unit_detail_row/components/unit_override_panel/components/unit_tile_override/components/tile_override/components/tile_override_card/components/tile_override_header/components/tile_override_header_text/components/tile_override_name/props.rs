use dioxus::prelude::*;

/// The active ability / unit name shown in the override panel header.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideNameProps {
    #[props(into)]
    pub text: String,
}
