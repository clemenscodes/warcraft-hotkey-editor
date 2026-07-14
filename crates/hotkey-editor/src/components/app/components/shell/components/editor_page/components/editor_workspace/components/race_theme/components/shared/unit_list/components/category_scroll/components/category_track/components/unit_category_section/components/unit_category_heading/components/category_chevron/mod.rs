mod model;
mod view;

pub use view::CategoryChevronView;
mod style;

use dioxus::prelude::*;
use model::CategoryChevronModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CategoryChevron(props: CategoryChevronModel) -> Element {
    let glyph = if props.is_collapsed {
        "\u{25b6}"
    } else {
        "\u{25bc}"
    };
    rsx! {
        span {
            class: CLASS,
            {glyph}
        }
    }
}

assert_component!(CategoryChevron);
