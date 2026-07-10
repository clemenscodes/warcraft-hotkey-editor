use dioxus::prelude::*;

/// The section title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct HelpSectionTitleProps {
    #[props(into)]
    pub title: String,
}
