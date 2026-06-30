use dioxus::prelude::*;

/// The legend description's only input: the copy, passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendDescriptionProps {
    pub children: Element,
}
