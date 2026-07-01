mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::TemplateCardNameProps;
use style::CLASS;
assert_component!(TemplateCardName);

/// A template card's name heading. Owns `.template-card-name`.
#[component]
pub fn TemplateCardName(props: TemplateCardNameProps) -> Element {
    let name = props.children.clone();
    rsx! {
        h3 { class: CLASS, {name} }
    }
}
