mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::AbilityDescriptionProps;

assert_component!(AbilityDescription);

/// The primary ubertip / tip text block for an ability or upgrade; renders nothing
/// when there is no description.
#[component]
pub fn AbilityDescription(props: AbilityDescriptionProps) -> Element {
    let description_lines = props.description_lines;
    if description_lines.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS,
            for description_line in description_lines {
                p { {description_line} }
            }
        }
    }
}
