use dioxus::prelude::*;

/// The unit's display name inside a card. Its selected-state colour is the
/// `--name-color` custom property the selected card surface publishes, so the name
/// needs no selected flag of its own.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardNameProps {
    #[props(into)]
    pub text: String,
}
