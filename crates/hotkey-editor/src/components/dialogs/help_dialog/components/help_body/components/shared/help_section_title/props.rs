use dioxus::prelude::*;

/// The section title's only input: the heading text, passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct HelpSectionTitleProps {
    pub children: Element,
}
