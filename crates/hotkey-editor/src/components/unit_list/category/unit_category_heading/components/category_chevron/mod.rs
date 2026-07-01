mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::CategoryChevronProps;

assert_component!(CategoryChevron);

/// The disclosure triangle beside a category heading.
#[component]
pub fn CategoryChevron(props: CategoryChevronProps) -> Element {
    let glyph = if props.is_collapsed {
        "\u{25b6}"
    } else {
        "\u{25bc}"
    };
    rsx! {
        span { class: CLASS, {glyph} }
    }
}
