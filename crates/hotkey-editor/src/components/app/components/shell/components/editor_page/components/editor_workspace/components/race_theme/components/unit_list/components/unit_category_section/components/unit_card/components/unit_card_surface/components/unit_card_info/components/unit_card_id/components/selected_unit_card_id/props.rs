use dioxus::prelude::*;

/// The unit id text and the `data-race` attribute value the code element carries
/// (which selects the race accent color).
#[derive(Props, Clone, PartialEq)]
pub struct SelectedUnitCardIdProps {
    pub race_attribute: &'static str,
    #[props(into)]
    pub text: String,
}
