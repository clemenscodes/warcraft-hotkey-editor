mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TemplateCardNameProps;

assert_component!(TemplateCardName);

/// A template card's name heading. Owns `.template-card-name`.
#[component]
pub fn TemplateCardName(props: TemplateCardNameProps) -> Element {
    let name = props.children.clone();
    rsx! {
        h3 {
            class: CLASS,
            {name}
        }
    }
}
