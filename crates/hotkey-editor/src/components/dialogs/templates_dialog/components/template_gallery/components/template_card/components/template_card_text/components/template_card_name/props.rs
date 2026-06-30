use dioxus::prelude::*;

/// The name heading's only input: the template name, passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardNameProps {
    pub children: Element,
}
