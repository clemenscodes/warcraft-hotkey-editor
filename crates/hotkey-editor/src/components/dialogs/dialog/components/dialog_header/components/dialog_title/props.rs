use dioxus::prelude::*;

/// The title's only input: the heading text, passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    pub children: Element,
}
