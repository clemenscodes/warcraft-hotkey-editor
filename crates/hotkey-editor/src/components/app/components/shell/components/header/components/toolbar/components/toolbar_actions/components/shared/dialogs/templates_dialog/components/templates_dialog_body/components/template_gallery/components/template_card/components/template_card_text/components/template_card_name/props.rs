use dioxus::prelude::*;

/// The name heading's only input: the template name.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardNameProps {
    #[props(into)]
    pub name: String,
}
