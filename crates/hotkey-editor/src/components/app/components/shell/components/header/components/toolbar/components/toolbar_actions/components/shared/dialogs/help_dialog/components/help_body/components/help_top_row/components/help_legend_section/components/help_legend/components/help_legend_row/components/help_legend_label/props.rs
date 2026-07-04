use dioxus::prelude::*;

/// The legend label's only input: the button name, passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendLabelProps {
    pub children: Element,
}
