mod model;
mod view;

pub use view::HelpLegendIconView;
mod style;

use dioxus::prelude::*;
use model::HelpLegendIconModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpLegendIcon(props: HelpLegendIconModel) -> Element {
    let icon = props.icon;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: icon,
        }
    }
}

assert_component!(HelpLegendIcon);
