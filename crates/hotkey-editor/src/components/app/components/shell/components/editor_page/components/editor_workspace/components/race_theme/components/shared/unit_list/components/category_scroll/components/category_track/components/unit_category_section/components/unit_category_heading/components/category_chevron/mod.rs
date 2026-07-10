mod props;
mod view;

pub use view::CategoryChevronView;
mod style;

use dioxus::prelude::*;
use props::CategoryChevronProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(CategoryChevron);
