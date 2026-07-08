use dioxus::prelude::*;

/// The description line's only input: the template description.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardDescriptionProps {
    #[props(into)]
    pub description: String,
}
