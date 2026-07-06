mod props;
mod style;

use dioxus::prelude::*;
pub use props::TemplateCardDescriptionProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TemplateCardDescription);

/// A template card's description line. Owns `.template-card-description`.
#[component]
pub fn TemplateCardDescription(props: TemplateCardDescriptionProps) -> Element {
    let description = props.children.clone();
    rsx! {
        p { class: CLASS, {description} }
    }
}
