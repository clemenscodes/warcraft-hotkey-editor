use dioxus::prelude::*;

/// The legend description's only input: the copy.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendDescriptionProps {
    #[props(into)]
    pub description: String,
}
