use dioxus::prelude::*;

/// The legend label's only input: the button name.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendLabelProps {
    #[props(into)]
    pub label: String,
}
