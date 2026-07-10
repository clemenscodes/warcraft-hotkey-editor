mod props;
mod view;

pub use view::AbilityDescriptionView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use props::AbilityDescriptionProps;

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

assert_component!(AbilityDescription);
