mod model;
mod view;

pub use view::HelpInlineIconView;
mod style;

use dioxus::prelude::*;
use model::HelpInlineIconModel;
use style::CLASS;
use tw_macro::assert_component;

/// A toolbar glyph dropped inline into a workflow sentence. A leaf: the step
/// passes which icon to draw.
#[component]
pub fn HelpInlineIcon(props: HelpInlineIconModel) -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: props.icon,
        }
    }
}

assert_component!(HelpInlineIcon);
