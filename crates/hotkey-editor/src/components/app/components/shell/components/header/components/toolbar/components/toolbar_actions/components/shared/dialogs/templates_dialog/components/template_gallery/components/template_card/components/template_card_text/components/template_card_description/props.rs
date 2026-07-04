use dioxus::prelude::*;

/// The description line's only input: the template description, passed as
/// children.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardDescriptionProps {
    pub children: Element,
}
