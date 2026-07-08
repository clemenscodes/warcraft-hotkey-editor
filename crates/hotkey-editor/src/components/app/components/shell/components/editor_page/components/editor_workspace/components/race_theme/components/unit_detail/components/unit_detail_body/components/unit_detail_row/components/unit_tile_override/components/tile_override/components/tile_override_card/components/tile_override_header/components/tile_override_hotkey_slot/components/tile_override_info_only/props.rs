use dioxus::prelude::*;

/// The note shown for a passive ability that has no hotkey field.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideInfoOnlyProps {
    #[props(into)]
    pub text: String,
}
