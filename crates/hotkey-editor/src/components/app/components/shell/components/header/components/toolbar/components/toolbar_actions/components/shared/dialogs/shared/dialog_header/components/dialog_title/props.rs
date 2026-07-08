use dioxus::prelude::*;

/// The title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    #[props(into)]
    pub title: String,
}
