mod props;
mod style;

use dioxus::prelude::*;
pub use props::ActiveCategoryTabProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ActiveCategoryTab);

/// The selected category tab: gold, highlighted, and marked as the current page.
#[component]
pub fn ActiveCategoryTab(props: ActiveCategoryTabProps) -> Element {
    let label = props.label;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "option",
            aria_selected: true,
            aria_current: "page",
            onclick: on_click,
            {label}
        }
    }
}
