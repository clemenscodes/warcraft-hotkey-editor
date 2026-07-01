mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TemplateCardDescriptionProps;

assert_component!(TemplateCardDescription);

/// A template card's description line. Owns `.template-card-description`.
#[component]
pub fn TemplateCardDescription(props: TemplateCardDescriptionProps) -> Element {
    let description = props.children.clone();
    rsx! {
        p {
            class: CLASS,
            {description}
        }
    }
}
