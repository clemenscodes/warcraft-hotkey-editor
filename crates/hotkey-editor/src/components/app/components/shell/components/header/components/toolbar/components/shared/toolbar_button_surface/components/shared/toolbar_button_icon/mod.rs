mod model;
mod view;

pub use view::ToolbarButtonIconView;
mod style;

use dioxus::prelude::*;
use model::ToolbarButtonIconModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToolbarButtonIcon(props: ToolbarButtonIconModel) -> Element {
    let icon = props.icon;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: icon,
        }
    }
}

assert_component!(ToolbarButtonIcon);
